# Wave: Technical Architecture & Planning

## 1. Repository Structure
*   **Style:** Monorepo
*   **Goal:** Shared logic across Windows, Linux, and Android.

## 2. The "Wave Engine"
*   **Base:** Fork/Derivative of **Servo**. 
*   **Strategy:** "Servo but custom." We will leverage Servo's high-performance, parallelized layout engine but heavily modify it to support the unique "Wave" feature set.
*   **Extension Runtime:**
    *   **Challenge:** Implementing the WebExtensions API (`chrome.*` and `browser.*`) on top of Servo.
    *   **Approach:** A compatibility layer written in Rust that bridges JS extension calls to internal engine methods. This allows Wave to load `.crx` and `.xpi` files directly.

## 3. Technology Stack
*   **Core Logic:** Rust (Wave Engine).
*   **UI Layer:** 
    *   **Architecture:** Native Rust. No HTML/CSS for the browser shell.
    *   **Desktop:** `winit` for windowing + Custom GPU renderer (using `wgpu` or `iced`).
    *   **Styling:** Custom "Theming Engine" supporting:
        *   **Presets:** Catppuccin (Latte, Frappé, Macchiato, Mocha).
        *   **Effects:** OS-native blur (Mica/Acrylic on Windows, Vibrancy on MacOS).
    *   **Mobile:** Native Android.
*   **Build System:** Cargo.
*   **Build System:** Cargo (Rust's package manager) with workspace support.

## 4. Platform Specifics
### Windows
*   Installer method (MSI/EXE).
*   Integration with Windows notifications.

### Linux
*   Packaging: Flatpak, Snap, AppImage, and distro-specific (DEB/RPM).
*   Wayland/X11 compatibility.

### Android
*   JNI bindings to the Core Engine.
*   Material Design adaptation of the "Panel" concept.
