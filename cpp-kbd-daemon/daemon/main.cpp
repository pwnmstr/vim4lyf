#include <gtkmm.h>
#include <iostream>
#include <thread>
#include <cstdlib>
#include <glibmm/dispatcher.h>

#include "scanner.h"
#include "overlay.h"
#include "ipc.h"

// Global dispatcher to communicate from IPC thread to GTK main thread
Glib::Dispatcher dispatcher;
std::string target_region = "all";
Glib::RefPtr<Gtk::Application> global_app;

void on_ipc_trigger() {
    Scanner scanner;
    if (scanner.init()) {
        auto elements = scanner.scan_active_window();
        Overlay::draw_filtered(global_app, elements, target_region);
    }
}

int main(int argc, char *argv[]) {
    std::cout << "Starting C++ kbd-daemon GTK loop..." << std::endl;

    global_app = Gtk::Application::create("com.github.kbd-daemon.cpp.overlay");

    dispatcher.connect(sigc::ptr_fun(&on_ipc_trigger));

    // Spawn IPC thread
    std::thread ipc_thread([]() {
        const char* xdg_runtime = std::getenv("XDG_RUNTIME_DIR");
        std::string socket_path = xdg_runtime ? std::string(xdg_runtime) : "/tmp";
        socket_path += "/kbd-daemon.sock";

        DaemonIpc ipc(socket_path);
        ipc.start([](const std::string& region) {
            target_region = region;
            dispatcher.emit();
        });
    });

    global_app->hold(); // Keep app running even without windows open
    global_app->run();

    ipc_thread.join();
    return 0;
}
