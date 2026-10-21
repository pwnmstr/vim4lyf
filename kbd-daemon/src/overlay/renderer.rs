use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Label, Fixed, CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION};
use gtk4_layer_shell::{LayerShell, Edge, Layer, KeyboardMode};
use log::info;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use crate::scanner::atspi::InteractableElement;
use crate::mapper::{KeyMapper, SequenceStyle};
use crate::overlay::macros::{save_macro, get_macro};

#[derive(Clone, Default)]
struct MacroState {
    recording_register: Option<char>,
    playing_register: Option<char>,
    count_prefix: String,
}

pub struct Overlay;

impl Overlay {
    pub fn draw_filtered(app: &Application, all_elements: Vec<InteractableElement>, region: &str) {
        let elements_rc = Rc::new(all_elements);
        let region_str = region.to_string();

        info!("Drawing GTK layer-shell overlay for region: {}", region_str);

        let display = gtk4::gdk::Display::default().unwrap();
        let monitors = display.monitors();

        let mut target_monitor = monitors.item(0).unwrap().downcast::<gtk4::gdk::Monitor>().unwrap();
        for i in 0..monitors.n_items() {
            let mon = monitors.item(i).unwrap().downcast::<gtk4::gdk::Monitor>().unwrap();
            target_monitor = mon;
            break;
        }

        let geometry = target_monitor.geometry();

        let base_x = geometry.x();
        let base_y = geometry.y();
        let width = geometry.width();
        let height = geometry.height();

        let (min_x, max_x, min_y, max_y) = match region_str.as_str() {
            "top-left" => (base_x, base_x + width / 2, base_y, base_y + height / 2),
            "top-right" => (base_x + width / 2, base_x + width, base_y, base_y + height / 2),
            "bottom-left" => (base_x, base_x + width / 2, base_y + height / 2, base_y + height),
            "bottom-right" => (base_x + width / 2, base_x + width, base_y + height / 2, base_y + height),
            _ => (base_x, base_x + width, base_y, base_y + height),
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
        let macro_state = Rc::new(RefCell::new(MacroState::default()));

        let window = ApplicationWindow::new(app);
        window.init_layer_shell();
        window.set_layer(Layer::Overlay);

        window.set_anchor(Edge::Top, true);
        window.set_anchor(Edge::Bottom, true);
        window.set_anchor(Edge::Left, true);
        window.set_anchor(Edge::Right, true);

        window.set_keyboard_mode(KeyboardMode::Exclusive);

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

            let relative_x = el.x - base_x;
            let relative_y = el.y - base_y;
            fixed.put(&label, relative_x as f64, relative_y as f64);
        }

        let key_controller = gtk4::EventControllerKey::new();
        let mapping_clone = Rc::clone(&mapping_rc);
        let elements_clone = Rc::clone(&filtered_rc);
        let seq_clone = Rc::clone(&current_sequence);
        let state_clone = Rc::clone(&macro_state);
        let window_clone = window.clone();

        key_controller.connect_key_pressed(move |_, keyval, _keycode, _state| {
            // Get string representation. Handle standard letters/digits.
            // Some special chars like '@' might appear as "at".
            let key_name = keyval.name().unwrap_or_else(|| "".into());
            let mut char_key = '\0';

            if key_name.len() == 1 {
                char_key = key_name.chars().next().unwrap();
            } else if key_name == "at" {
                char_key = '@';
            }

            if key_name == "Escape" {
                info!("Escape pressed, closing overlay");
                window_clone.close();
                return glib::Propagation::Stop;
            }

            if char_key != '\0' {
                let mut state = state_clone.borrow_mut();

                // 1. Parsing digits for repeat count (only at the start of a command)
                if char_key.is_ascii_digit() && seq_clone.borrow().is_empty() && state.recording_register.is_none() && state.playing_register.is_none() {
                    state.count_prefix.push(char_key);
                    info!("Count prefix: {}", state.count_prefix);
                    return glib::Propagation::Stop;
                }

                // 2. Playback state (`@` + `<reg>`)
                if char_key == '@' && seq_clone.borrow().is_empty() && state.recording_register.is_none() {
                    state.playing_register = Some('?');
                    info!("Awaiting register for playback...");
                    return glib::Propagation::Stop;
                }

                if state.playing_register == Some('?') {
                    state.playing_register = Some(char_key);
                    if let Some(target_path) = get_macro(char_key) {
                        let repeat_count = if state.count_prefix.is_empty() { 1 } else { state.count_prefix.parse::<usize>().unwrap_or(1) };
                        info!("Playing back macro '{}' targeting path '{}' (Repeat: {}x)", char_key, target_path, repeat_count);
                        // Trigger backend action * repeat_count
                        window_clone.close();
                    } else {
                        info!("Register '{}' is empty.", char_key);
                        state.playing_register = None; // Abort
                    }
                    return glib::Propagation::Stop;
                }

                // 3. Recording state (`q` + `<reg>`)
                if char_key == 'q' && seq_clone.borrow().is_empty() && state.recording_register.is_none() && state.playing_register.is_none() {
                    info!("Entering macro record mode, awaiting register...");
                    state.recording_register = Some('?');
                    return glib::Propagation::Stop;
                }

                if state.recording_register == Some('?') {
                    state.recording_register = Some(char_key);
                    info!("Recording macro to register '{}'", char_key);
                    return glib::Propagation::Stop;
                }

                // 4. Normal label selection
                let mut seq = seq_clone.borrow_mut();
                seq.push(char_key);
                info!("Current sequence: {}", seq);

                if let Some(&idx) = mapping_clone.get(seq.as_str()) {
                    let el = &elements_clone[idx];

                    let repeat_count = if state.count_prefix.is_empty() {
                        1
                    } else {
                        state.count_prefix.parse::<usize>().unwrap_or(1)
                    };

                    info!("Matched sequence '{}' to element '{}' (Repeat: {}x)", seq, el.name, repeat_count);

                    if let Some(reg) = state.recording_register {
                        if reg != '?' {
                            info!("Recorded target '{}' to register '{}'", el.name, reg);
                            save_macro(reg, el.path.clone());
                        }
                    }

                    // Trigger backend action * repeat_count
                    window_clone.close();
                    return glib::Propagation::Stop;
                }

                if seq.len() > 4 {
                    seq.clear();
                    state.count_prefix.clear();
                }
            }

            glib::Propagation::Proceed
        });
        window.add_controller(key_controller);

        window.set_child(Some(&fixed));
        window.present();
    }
}
