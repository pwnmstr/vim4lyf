#pragma once
#include <string>
#include <functional>

class DaemonIpc {
public:
    DaemonIpc(const std::string& path);
    void start(std::function<void(const std::string&)> on_trigger);

private:
    std::string socket_path;
};
