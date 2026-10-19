mod backend;
mod daemon;
mod mapper;
mod overlay;
mod scanner;

use daemon::ipc::DaemonIpc;
use log::{error, info};
use mapper::{KeyMapper, SequenceStyle};
use overlay::Overlay;
use scanner::atspi::Scanner;
use std::collections::HashMap;
use tokio::task;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting kbd-daemon...");

    let xdg_runtime_dir = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".into());
    let ipc_path = format!("{}/kbd-daemon.sock", xdg_runtime_dir);
    let ipc = DaemonIpc::new(&ipc_path);

    // Run the IPC server
    ipc.start(|| async {
        info!("Handling trigger-scan event");

        let scanner = match Scanner::new().await {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to initialize AT-SPI scanner: {}", e);
                return;
            }
        };

        match scanner.scan_active_window().await {
            Ok(elements) => {
                info!("Found {} elements", elements.len());

                let mapper = KeyMapper::new(SequenceStyle::QuadrantAsdf);
                let mapping = mapper.generate_mapping(elements.len());

                // GTK needs to run on a blocking thread because it manages its own event loop
                task::spawn_blocking(move || {
                    info!("Spawning overlay...");
                    let overlay = Overlay::new();
                    overlay.draw(mapping, elements);
                });
            }
            Err(e) => {
                error!("Scan failed: {}", e);
            }
        }
    })
    .await
    .unwrap();
}
