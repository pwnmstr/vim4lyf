#include "ipc.h"
#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>
#include <iostream>
#include <vector>

DaemonIpc::DaemonIpc(const std::string& path) : socket_path(path) {}

void DaemonIpc::start(std::function<void(const std::string&)> on_trigger) {
    int server_fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (server_fd == -1) {
        std::cerr << "[IPC] Failed to create socket" << std::endl;
        return;
    }

    unlink(socket_path.c_str());

    struct sockaddr_un addr;
    memset(&addr, 0, sizeof(addr));
    addr.sun_family = AF_UNIX;
    strncpy(addr.sun_path, socket_path.c_str(), sizeof(addr.sun_path) - 1);

    if (bind(server_fd, (struct sockaddr*)&addr, sizeof(addr)) == -1) {
        std::cerr << "[IPC] Failed to bind socket" << std::endl;
        return;
    }

    if (listen(server_fd, 5) == -1) {
        std::cerr << "[IPC] Failed to listen on socket" << std::endl;
        return;
    }

    std::cout << "[IPC] Daemon listening on " << socket_path << std::endl;

    while (true) {
        int client_fd = accept(server_fd, NULL, NULL);
        if (client_fd == -1) {
            continue;
        }

        char buffer[1024];
        ssize_t bytes_read = read(client_fd, buffer, sizeof(buffer) - 1);
        if (bytes_read > 0) {
            buffer[bytes_read] = '\0';
            std::string msg(buffer);

            // Trim newlines if any
            msg.erase(msg.find_last_not_of(" \n\r\t") + 1);

            if (msg.find("trigger-scan") == 0) {
                std::string region = "all";
                size_t space_pos = msg.find(' ');
                if (space_pos != std::string::npos) {
                    region = msg.substr(space_pos + 1);
                }

                std::cout << "[IPC] Received scan trigger for region: " << region << std::endl;
                on_trigger(region);
            }
        }
        close(client_fd);
    }
}
