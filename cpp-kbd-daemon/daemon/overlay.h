#pragma once
#include <gtkmm.h>
#include <string>
#include <unordered_map>
#include <vector>
#include "scanner.h"

class Overlay {
public:
    static void draw_filtered(Glib::RefPtr<Gtk::Application> app,
                              const std::vector<InteractableElement>& elements,
                              const std::string& region);
};
