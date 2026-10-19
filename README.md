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
You will need GTK4, D-Bus/AT-SPI development packages, and the `gtk4-layer-shell` library. Note that `gtk4-layer-shell` might not be in your standard package manager (unlike the GTK3 version), so you may need to build it from source if it isn't available.

```bash
# Basic dependencies
sudo apt-get install -y \
  build-essential \
  pkg-config \
  libgtk-4-dev \
  libglib2.0-dev

# If your distro provides gtk4-layer-shell-dev, install it.
# Otherwise, you can build it from source:
# git clone https://github.com/wmww/gtk4-layer-shell.git
# cd gtk4-layer-shell && meson setup build && ninja -C build && sudo ninja -C build install
```

## Setup and Installation
1. Ensure your Rust toolchain is installed (e.g. using rustup).
2. Clone the repository and navigate into the `kbd-daemon` folder.
3. Build the daemon and CLI:
   ```bash
   cargo build --release
   ```
4. Copy the compiled binaries to your system path:
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
In your `~/.config/hypr/hyprland.conf`, you can bind a global shortcut to activate Scan Mode. For example, using `Super + S`:

```conf
# Launch the daemon on startup
exec-once = kbd-daemon

# Trigger the overlay using Super+S
bind = SUPER, S, exec, kbd-cli trigger-scan
```

## How It Works
When triggered via the CLI, `kbd-daemon` receives an IPC command on a Unix socket inside your `$XDG_RUNTIME_DIR`. It then searches the accessibility tree via AT-SPI to find interactive elements and determines their bounding boxes.

It spawns a GTK4 fullscreen, transparent `layer-shell` overlay over your desktop which grabs exclusive keyboard focus. Each interactable element is assigned a small keystroke sequence label. Typing the sequence will directly trigger the element's action via AT-SPI, or simulate a mouse click at the element's coordinates as a fallback.
