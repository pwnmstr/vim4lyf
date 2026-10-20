#pragma once
#include <string>
#include <vector>

struct InteractableElement {
    std::string name;
    int role; // Corresponds to AtspiRole
    int x;
    int y;
    int width;
    int height;
    std::string path;
    std::string bus_name;
};

class Scanner {
public:
    Scanner();
    ~Scanner();
    bool init();
    std::vector<InteractableElement> scan_active_window();
};
