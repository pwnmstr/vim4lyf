# kbd-daemon (C++ implementation)

This is the C++ port of the Wayland GTK4 accessibility keyboard scanner prototype.

## Dependencies
Requires `cmake`, `libgtkmm-4.0-dev`, `gtk4-layer-shell`, and `libatspi2.0-dev`.

## Build
```bash
mkdir build && cd build
cmake ..
make -j4
```

## Usage
Run the daemon:
```bash
./daemon/kbd-daemon
```

Trigger a scan using the CLI:
```bash
./cli/kbd-cli trigger-scan top-left
```
