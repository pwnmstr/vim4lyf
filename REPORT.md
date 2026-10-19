# Architecture and Feasibility Report

## Overview
This project is a lightweight Linux daemon and companion CLI utility designed to enable keyboard-driven interactions with graphical desktop elements. The primary target environment is Wayland, specifically wlroots-based compositors like Hyprland. It uses AT-SPI2 for discovering accessible UI elements and Wayland protocols for rendering overlays and simulating fallback input.

## Components

1. **Daemon (`daemon`)**: Runs in the background, listening on a Unix domain socket for IPC commands from the CLI. Manages the state machine (Normal Mode -> Scan Mode -> Click Mode).
2. **CLI Utility (`cli`)**: A lightweight command-line tool. You will bind commands like `cli trigger-scan` in your `hyprland.conf` to wake the daemon.
3. **Accessibility Scanner (`scanner`)**: Connects to the D-Bus AT-SPI interface. When "Scan Mode" is triggered, it queries the accessibility tree of the currently active window/desktop to find elements with clickable/interactable roles (e.g., PushButton, Link, MenuItem) and extracts their screen bounding boxes.
4. **Key-mapping Engine (`mapper`)**: Assigns a unique key sequence to each detected element. As requested, this uses a modular design so that an `a/s/d/f` quadrant-based approach or a sequential `h/j/k/l` approach can be plugged in easily.
5. **Overlay Renderer (`overlay`)**: Uses GTK with `gtk-layer-shell` to draw a transparent fullscreen surface over the display. When activated, it grabs keyboard focus (Wayland `keyboard_interactivity`) so that the sequence typed by the user is captured directly by the overlay. It draws the assigned key sequences as small badges over the respective bounding boxes.
6. **Input Handler (`input`)**: Parses the keystrokes captured by the overlay window. If the typed sequence matches an element, it transitions to Click Mode. If the `Escape` key is pressed, it closes the overlay and returns to Normal Mode.
7. **Interaction Backend (`backend`)**:
    - **Primary**: Attempts to invoke the `DoAction` method via AT-SPI directly on the accessible node.
    - **Fallback**: If the node cannot be activated directly, uses the Wayland `wlr-virtual-pointer-unstable-v1` protocol to move the virtual cursor to the center of the bounding box and send a left-click event.

## Feasibility and Platform Considerations

### 1. Wayland Input Capture
In X11, an application could install a global keyboard hook. In Wayland, this is fundamentally prohibited for security.
**Solution**: We rely on the desktop environment's own shortcut daemon (e.g., Hyprland binds). The global shortcut triggers our CLI, which signals the daemon to open the `layer-shell` overlay. The overlay requests exclusive keyboard focus while active. This is the standard, secure Wayland approach.

### 2. Accessibility (AT-SPI) Support
We use `zbus` and the `atspi` crate to traverse the accessibility tree.
**Risks**:
- Some applications (notably Electron apps like VSCode, Discord, or older Chromium versions) may not expose their accessibility trees by default. Users often need to launch them with flags like `--enable-features=AccessibilityObjectModel` or `--force-renderer-accessibility`.
- Coordinate translation: AT-SPI returns absolute screen coordinates. Wayland compositors handle scaling and display offsets differently. We must ensure that the coordinates returned by AT-SPI accurately map to the logical pixels used by the GTK overlay.

### 3. Fallback Input Simulation
Wayland does not have an `XTest` equivalent for simulating mouse clicks globally.
**Solution**: `wlr-virtual-pointer`. This protocol allows clients to create a virtual input device. It is well-supported in wlroots-based compositors like Hyprland. However, it requires the compositor to grant the application permission to use it (usually granted by default in Hyprland).

### 4. Language Choice
Rust is chosen for its memory safety, excellent `zbus` bindings for D-Bus/AT-SPI, and strong Wayland ecosystem (`wayland-client`, `smithay-client-toolkit`, `gtk-layer-shell` bindings).

## Development Roadmap
- **Phase 1**: Architecture validation (This report).
- **Phase 2**: Project scaffolding, AT-SPI tree traversal, and stubbed key-mapper.
- **Phase 3**: GTK layer-shell overlay rendering and Wayland keyboard grab.
- **Phase 4**: Virtual pointer fallback and IPC daemonization.
- **Phase 5**: Testing, optimization, and documentation.
