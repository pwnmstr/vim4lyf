# Keyboard-Driven Linux Interaction Daemon

A lightweight Linux daemon and companion CLI that allows you to identify and activate on-screen clickable elements entirely through the keyboard, minimizing mouse usage.

## Features
- **Wayland Native**: Integrates seamlessly with `wlroots`-based compositors like Hyprland using `layer-shell` and `wlr-virtual-pointer`.
- **Accessibility API Support**: Uses AT-SPI to scan for interactable UI elements like buttons, links, tabs, and menus.
- **Fast and Lightweight**: Built in Rust.
- **Customizable Key Mappings**: Generates keyboard shortcuts over detected UI elements using `a, s, d, f` (quadrant-based) or sequential mappings.

## Requirements
- A Wayland compositor supporting `wlr-layer-shell-unstable-v1` and `wlr-virtual-pointer-unstable-v1` (e.g., Hyprland, Sway).
- AT-SPI2 for accessibility tree querying.
- GTK4 and `gtk4-layer-shell` for the UI overlay.

### Build Dependencies (Ubuntu/Debian)
```bash
sudo apt-get install -y \
  build-essential \
  pkg-config \
  libgtk-4-dev \
  libglib2.0-dev
```

## Setup and Installation
1. Ensure your Rust toolchain is installed (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`).
2. Clone the repository and build:
   ```bash
   cd kbd-daemon
   cargo build --release
   ```
3. Copy the compiled binaries to your path:
   ```bash
   sudo cp target/release/kbd-daemon /usr/local/bin/
   sudo cp target/release/kbd-cli /usr/local/bin/
   ```

## Usage
Start the daemon in the background (you may want to add this to your startup script or `.config/hypr/hyprland.conf`):
```bash
kbd-daemon &
```

Then, trigger "Scan Mode" to show labels on your screen:
```bash
kbd-cli trigger-scan
```

### Hyprland Integration
In your `~/.config/hypr/hyprland.conf`, you can bind a global shortcut to activate Scan Mode:
```conf
# Launch the daemon on startup
exec-once = kbd-daemon

# Trigger the overlay using Super+S
bind = SUPER, S, exec, kbd-cli trigger-scan
```

## How It Works
When triggered, `kbd-daemon` searches the accessibility tree via AT-SPI to find interactive elements and determines their bounding boxes. It then spawns a GTK4 fullscreen, transparent `layer-shell` overlay over your desktop.

The overlay grabs exclusive keyboard focus. Each interactable element is assigned a small keystroke sequence label. Typing the sequence will directly trigger the element's action via AT-SPI, or simulate a mouse click at the element's coordinates as a fallback.
