#include <iostream>
#include <string>
#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>
#include <cstdlib>

int main(int argc, char *argv[]) {
    if (argc < 2) {
        std::cerr << "Usage: kbd-cli <command> [args]" << std::endl;
        std::cerr << "Commands:" << std::endl;
        std::cerr << "  trigger-scan [region]    Trigger scan mode" << std::endl;
        std::cerr << "                           Regions: all, top-left, top-right, bottom-left, bottom-right" << std::endl;
        return 1;
    }

    std::string command = argv[1];

    if (command == "trigger-scan") {
        std::string region = "all";
        if (argc >= 3) {
            region = argv[2];
        }

        const char* xdg_runtime = std::getenv("XDG_RUNTIME_DIR");
        std::string socket_path = xdg_runtime ? std::string(xdg_runtime) : "/tmp";
        socket_path += "/kbd-daemon.sock";

        int fd = socket(AF_UNIX, SOCK_STREAM, 0);
        if (fd == -1) {
            std::cerr << "Failed to create socket." << std::endl;
            return 1;
        }

        struct sockaddr_un addr;
        memset(&addr, 0, sizeof(addr));
        addr.sun_family = AF_UNIX;
        strncpy(addr.sun_path, socket_path.c_str(), sizeof(addr.sun_path) - 1);

        if (connect(fd, (struct sockaddr*)&addr, sizeof(addr)) == -1) {
            std::cerr << "Failed to connect to daemon socket at " << socket_path << ". Is kbd-daemon running?" << std::endl;
            close(fd);
            return 1;
        }

        std::string msg = "trigger-scan " + region;
        if (write(fd, msg.c_str(), msg.length()) == -1) {
            std::cerr << "Failed to write to socket." << std::endl;
        } else {
            std::cout << "Scan mode triggered for region: " << region << std::endl;
        }

        close(fd);
    } else {
        std::cerr << "Unknown command: " << command << std::endl;
        return 1;
    }

    return 0;
}
