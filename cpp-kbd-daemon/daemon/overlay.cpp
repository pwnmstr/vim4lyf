#include "overlay.h"
#include "mapper.h"
#include <gtk4-layer-shell.h>
#include <iostream>

void Overlay::draw_filtered(Glib::RefPtr<Gtk::Application> app,
                            const std::vector<InteractableElement>& all_elements,
                            const std::string& region) {

    std::cout << "[Overlay] Drawing GTK layer-shell overlay for region: " << region << std::endl;

    auto display = Gdk::Display::get_default();
    auto monitors = display->get_monitors();

    // Default to first monitor
    auto target_monitor = std::dynamic_pointer_cast<Gdk::Monitor>(monitors->get_object(0));
    if (!target_monitor) {
        std::cerr << "Failed to find monitor." << std::endl;
        return;
    }

    Gdk::Rectangle geometry;
    target_monitor->get_geometry(geometry);

    int base_x = geometry.get_x();
    int base_y = geometry.get_y();
    int width = geometry.get_width();
    int height = geometry.get_height();

    int min_x = base_x, max_x = base_x + width;
    int min_y = base_y, max_y = base_y + height;

    if (region == "top-left") {
        max_x = base_x + width / 2;
        max_y = base_y + height / 2;
    } else if (region == "top-right") {
        min_x = base_x + width / 2;
        max_y = base_y + height / 2;
    } else if (region == "bottom-left") {
        max_x = base_x + width / 2;
        min_y = base_y + height / 2;
    } else if (region == "bottom-right") {
        min_x = base_x + width / 2;
        min_y = base_y + height / 2;
    }

    std::vector<InteractableElement> filtered_elements;
    for (const auto& el : all_elements) {
        int center_x = el.x + (el.width / 2);
        int center_y = el.y + (el.height / 2);
        if (center_x >= min_x && center_x <= max_x && center_y >= min_y && center_y <= max_y) {
            filtered_elements.push_back(el);
        }
    }

    KeyMapper mapper(SequenceStyle::QuadrantAsdf);
    auto mapping = mapper.generate_mapping(filtered_elements.size());

    // Create new window
    auto window = new Gtk::ApplicationWindow();
    window->set_application(app);

    // Initialize layer shell (we must cast to underlying C object for gtk4-layer-shell)
    GtkWindow* c_window = GTK_WINDOW(window->gobj());
    gtk_layer_init_for_window(c_window);
    gtk_layer_set_layer(c_window, GTK_LAYER_SHELL_LAYER_OVERLAY);

    gtk_layer_set_anchor(c_window, GTK_LAYER_SHELL_EDGE_TOP, true);
    gtk_layer_set_anchor(c_window, GTK_LAYER_SHELL_EDGE_BOTTOM, true);
    gtk_layer_set_anchor(c_window, GTK_LAYER_SHELL_EDGE_LEFT, true);
    gtk_layer_set_anchor(c_window, GTK_LAYER_SHELL_EDGE_RIGHT, true);

    gtk_layer_set_keyboard_mode(c_window, GTK_LAYER_SHELL_KEYBOARD_MODE_EXCLUSIVE);

    // CSS
    auto provider = Gtk::CssProvider::create();
    provider->load_from_data(
        "window { background-color: rgba(0, 0, 0, 0.3); }"
        "label.sequence-badge {"
        "    background-color: #ffb86c;"
        "    color: #282a36;"
        "    padding: 2px 6px;"
        "    border-radius: 4px;"
        "    font-family: monospace;"
        "    font-weight: bold;"
        "    font-size: 14px;"
        "}"
    );
    Gtk::StyleContext::add_provider_for_display(display, provider, GTK_STYLE_PROVIDER_PRIORITY_APPLICATION);

    auto fixed = Gtk::make_managed<Gtk::Fixed>();

    for (const auto& [seq, idx] : mapping) {
        const auto& el = filtered_elements[idx];
        auto label = Gtk::make_managed<Gtk::Label>(seq);
        label->add_css_class("sequence-badge");

        int relative_x = el.x - base_x;
        int relative_y = el.y - base_y;
        fixed->put(*label, relative_x, relative_y);
    }

    window->set_child(*fixed);

    // Keyboard Controller
    auto key_controller = Gtk::EventControllerKey::create();

    // Need to store state in a shared struct to keep it alive
    struct State {
        std::string current_seq;
        std::unordered_map<std::string, size_t> mapping;
        std::vector<InteractableElement> elements;
        Gtk::ApplicationWindow* win;
    };
    auto state = std::make_shared<State>();
    state->mapping = mapping;
    state->elements = filtered_elements;
    state->win = window;

    key_controller->signal_key_pressed().connect(
        [state](guint keyval, guint keycode, Gdk::ModifierType modifier) -> bool {
            auto key_name_g = gdk_keyval_name(keyval);
            std::string key_name = key_name_g ? key_name_g : "";

            if (key_name == "Escape") {
                std::cout << "Escape pressed, closing overlay" << std::endl;
                state->win->close();
                return true; // Stop propagation
            }

            if (key_name.length() == 1) {
                state->current_seq += key_name;
                std::cout << "Current sequence: " << state->current_seq << std::endl;

                if (state->mapping.count(state->current_seq) > 0) {
                    size_t idx = state->mapping[state->current_seq];
                    std::cout << "Matched sequence '" << state->current_seq
                              << "' to element '" << state->elements[idx].name << "'" << std::endl;
                    state->win->close();
                    return true;
                }

                if (state->current_seq.length() > 4) {
                    state->current_seq.clear();
                }
            }

            return false; // Proceed
        },
        true // After
    );

    window->add_controller(key_controller);
    window->present();

    // Ensure window memory is managed properly when closed
    window->signal_hide().connect([window]() {
        delete window;
    });
}
