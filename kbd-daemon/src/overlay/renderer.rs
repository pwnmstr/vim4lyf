use crate::scanner::atspi::InteractableElement;
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, CssProvider, Fixed, Label, STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use log::info;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Overlay {
    app: Application,
}

impl Overlay {
    pub fn new() -> Self {
        let app = Application::builder()
            .application_id("com.github.kbd-daemon.overlay")
            .build();

        Self { app }
    }

    pub fn draw(&self, mapping: HashMap<String, usize>, elements: Vec<InteractableElement>) {
        let mapping_rc = Rc::new(mapping);
        let elements_rc = Rc::new(elements);
        let current_sequence = Rc::new(RefCell::new(String::new()));

        self.app.connect_activate(move |app| {
            info!("Drawing GTK layer-shell overlay");

            let window = ApplicationWindow::new(app);
            window.init_layer_shell();
            window.set_layer(Layer::Overlay);
            window.auto_exclusive_zone_enable();

            // Fill the whole screen
            window.set_anchor(Edge::Top, true);
            window.set_anchor(Edge::Bottom, true);
            window.set_anchor(Edge::Left, true);
            window.set_anchor(Edge::Right, true);

            // Allow keyboard focus grabbing (Wayland)
            window.set_keyboard_mode(KeyboardMode::Exclusive);

            // Setup CSS
            let provider = CssProvider::new();
            provider.load_from_data(
                "window { background-color: rgba(0, 0, 0, 0.3); }
                 label.sequence-badge {
                     background-color: #ffb86c;
                     color: #282a36;
                     padding: 2px 6px;
                     border-radius: 4px;
                     font-family: monospace;
                     font-weight: bold;
                     font-size: 14px;
                 }",
            );
            gtk4::style_context_add_provider_for_display(
                &gtk4::gdk::Display::default().unwrap(),
                &provider,
                STYLE_PROVIDER_PRIORITY_APPLICATION,
            );

            let fixed = Fixed::new();

            for (seq, idx) in mapping_rc.iter() {
                let el = &elements_rc[*idx];
                let label = Label::new(Some(seq));
                label.add_css_class("sequence-badge");
                fixed.put(&label, el.x as f64, el.y as f64);
            }

            // Keyboard event handling
            let key_controller = gtk4::EventControllerKey::new();
            let app_clone = app.clone();
            let mapping_clone = Rc::clone(&mapping_rc);
            let elements_clone = Rc::clone(&elements_rc);
            let seq_clone = Rc::clone(&current_sequence);

            key_controller.connect_key_pressed(move |_, keyval, _keycode, _state| {
                let key_name = keyval.name().unwrap_or_else(|| "".into());

                if key_name == "Escape" {
                    info!("Escape pressed, closing overlay");
                    app_clone.quit();
                    return glib::Propagation::Stop;
                }

                if key_name.len() == 1 {
                    let mut seq = seq_clone.borrow_mut();
                    seq.push_str(&key_name);
                    info!("Current sequence: {}", seq);

                    if let Some(&idx) = mapping_clone.get(seq.as_str()) {
                        let el = &elements_clone[idx];
                        info!("Matched sequence '{}' to element '{}'", seq, el.name);

                        // NOTE: Backend interactions (AT-SPI / wlr-virtual-pointer) are stubbed here
                        // for the UI mock prototype.

                        app_clone.quit();
                        return glib::Propagation::Stop;
                    }

                    // Reset if sequence gets too long without a match
                    if seq.len() > 4 {
                        seq.clear();
                    }
                }

                glib::Propagation::Proceed
            });
            window.add_controller(key_controller);

            window.set_child(Some(&fixed));
            window.present();
        });

        // Run GTK loop
        self.app.run();
    }
}
