use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Label, Fixed, CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION};
use gtk4_layer_shell::{LayerShell, Edge, Layer, KeyboardMode};
use log::info;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use crate::scanner::atspi::InteractableElement;
use crate::mapper::{KeyMapper, SequenceStyle};

pub struct Overlay;

impl Overlay {
    pub fn draw_filtered(app: &Application, all_elements: Vec<InteractableElement>, region: &str) {
        let elements_rc = Rc::new(all_elements);
        let region_str = region.to_string();

        info!("Drawing GTK layer-shell overlay for region: {}", region_str);

        // Get display bounds to calculate quadrants
        let display = gtk4::gdk::Display::default().unwrap();
        let monitors = display.monitors();

        // We find the primary monitor (or default to the first if primary is missing)
        let mut target_monitor = monitors.item(0).unwrap().downcast::<gtk4::gdk::Monitor>().unwrap();
        for i in 0..monitors.n_items() {
            let mon = monitors.item(i).unwrap().downcast::<gtk4::gdk::Monitor>().unwrap();
            // Optional: determine if it's the active/focused monitor. Here we assume the first one.
            target_monitor = mon;
            break;
        }

        let geometry = target_monitor.geometry();

        // Handle multi-monitor offsets!
        let base_x = geometry.x();
        let base_y = geometry.y();
        let width = geometry.width();
        let height = geometry.height();

        let (min_x, max_x, min_y, max_y) = match region_str.as_str() {
            "top-left" => (base_x, base_x + width / 2, base_y, base_y + height / 2),
            "top-right" => (base_x + width / 2, base_x + width, base_y, base_y + height / 2),
            "bottom-left" => (base_x, base_x + width / 2, base_y + height / 2, base_y + height),
            "bottom-right" => (base_x + width / 2, base_x + width, base_y + height / 2, base_y + height),
            _ => (base_x, base_x + width, base_y, base_y + height), // "all"
        };

        let mut filtered_elements = Vec::new();
        for el in elements_rc.iter() {
            let center_x = el.x + (el.width / 2);
            let center_y = el.y + (el.height / 2);

            if center_x >= min_x && center_x <= max_x && center_y >= min_y && center_y <= max_y {
                filtered_elements.push(el.clone());
            }
        }

        info!("After quadrant filtering, {} elements remain", filtered_elements.len());

        let mapper = KeyMapper::new(SequenceStyle::QuadrantAsdf);
        let mapping = mapper.generate_mapping(filtered_elements.len());

        let mapping_rc = Rc::new(mapping);
        let filtered_rc = Rc::new(filtered_elements);
        let current_sequence = Rc::new(RefCell::new(String::new()));

        let window = ApplicationWindow::new(app);
        window.init_layer_shell();
        window.set_layer(Layer::Overlay);

        // Do NOT use auto_exclusive_zone_enable for a transparent full screen overlay!
        // It pushes other windows away in wlroots!

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
             }"
        );
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let fixed = Fixed::new();

        for (seq, idx) in mapping_rc.iter() {
            let el = &filtered_rc[*idx];
            let label = Label::new(Some(seq));
            label.add_css_class("sequence-badge");

            // Adjust label position relative to monitor coordinates if needed
            let relative_x = el.x - base_x;
            let relative_y = el.y - base_y;
            fixed.put(&label, relative_x as f64, relative_y as f64);
        }

        // Keyboard event handling
        let key_controller = gtk4::EventControllerKey::new();
        let mapping_clone = Rc::clone(&mapping_rc);
        let elements_clone = Rc::clone(&filtered_rc);
        let seq_clone = Rc::clone(&current_sequence);
        let window_clone = window.clone();

        key_controller.connect_key_pressed(move |_, keyval, _keycode, _state| {
            let key_name = keyval.name().unwrap_or_else(|| "".into());

            if key_name == "Escape" {
                info!("Escape pressed, closing overlay");
                window_clone.close();
                return glib::Propagation::Stop;
            }

            if key_name.len() == 1 {
                let mut seq = seq_clone.borrow_mut();
                seq.push_str(&key_name);
                info!("Current sequence: {}", seq);

                if let Some(&idx) = mapping_clone.get(seq.as_str()) {
                    let el = &elements_clone[idx];
                    info!("Matched sequence '{}' to element '{}'", seq, el.name);

                    window_clone.close();
                    return glib::Propagation::Stop;
                }

                if seq.len() > 4 {
                    seq.clear();
                }
            }

            glib::Propagation::Proceed
        });
        window.add_controller(key_controller);

        window.set_child(Some(&fixed));
        window.present();
    }
}
