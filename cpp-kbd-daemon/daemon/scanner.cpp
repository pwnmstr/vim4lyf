#include "scanner.h"
#include <iostream>

Scanner::Scanner() {}
Scanner::~Scanner() {}

bool Scanner::init() {
    // In a real implementation we would call atspi_init() here
    // but for the UI mock prototype, we just return true.
    std::cout << "[Scanner] Initialized mock AT-SPI connection." << std::endl;
    return true;
}

std::vector<InteractableElement> Scanner::scan_active_window() {
    std::vector<InteractableElement> elements;

    // Mock data to match the Rust prototype
    elements.push_back({"Applications Menu", 0, 50, 50, 100, 30, "/mock/appmenu", "org.a11y.mock"});
    elements.push_back({"Firefox Browser", 0, 400, 200, 800, 600, "/mock/firefox", "org.a11y.mock"});
    elements.push_back({"Close Button", 0, 1150, 210, 30, 30, "/mock/closebtn", "org.a11y.mock"});

    return elements;
}
