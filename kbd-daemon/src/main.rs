mod scanner;
mod mapper;
mod overlay;
mod backend;
mod daemon;

use log::info;
use scanner::atspi::Scanner;
use overlay::Overlay;
use daemon::ipc::DaemonIpc;
use gtk4::prelude::*;
use gtk4::Application;
use std::thread;
use crate::scanner::atspi::InteractableElement;

fn main() {
    env_logger::init();
    info!("Starting kbd-daemon GTK loop...");

    let app = Application::builder()
        .application_id("com.github.kbd-daemon.overlay")
        .build();

    app.connect_activate(move |app_ref| {
        info!("GTK Application activated, starting IPC server in background...");

        let (sender, receiver) = async_channel::unbounded::<String>();
        let (element_sender, element_receiver) = async_channel::unbounded::<(Vec<InteractableElement>, String)>();

        // Local GTK loop to handle UI drawing requests without moving the GTK App across threads
        let app_for_draw = app_ref.clone();
        glib::MainContext::default().spawn_local(async move {
            while let Ok((elements, region)) = element_receiver.recv().await {
                Overlay::draw_filtered(&app_for_draw, elements, &region);
            }
        });

        // Intermediate tokio context to spawn scanner which is thread-safe (just DBus info)
        glib::MainContext::default().spawn_local(async move {
            while let Ok(region) = receiver.recv().await {
                info!("Main GTK thread received scan request for region: {}", region);
                let element_sender_clone = element_sender.clone();
                let region_clone = region.clone();

                thread::spawn(move || {
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(async {
                        if let Ok(scanner) = Scanner::new().await {
                            if let Ok(elements) = scanner.scan_active_window().await {
                                let _ = element_sender_clone.send((elements, region_clone)).await;
                            }
                        }
                    });
                });
            }
        });

        // Background thread running pure Tokio IPC daemon
        let sender_for_ipc = sender.clone();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                let xdg_runtime_dir = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".into());
                let ipc_path = format!("{}/kbd-daemon.sock", xdg_runtime_dir);
                let ipc = DaemonIpc::new(&ipc_path);

                ipc.start(move |region: String| {
                    let sender_clone = sender_for_ipc.clone();
                    async move {
                        let _ = sender_clone.send(region).await;
                    }
                }).await.unwrap();
            });
        });
    });

    app.run();
}
