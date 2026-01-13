# Wave Browser Roadmap

## Phase 1: Foundation (Completed)
- [x] Project Structure (Monorepo)
- [x] Product Specification
- [x] **Wave Shield Implementation**
    - [x] Basic URL filtering
    - [x] Integration with Rust `adblock` libraries
- [x] **Core Engine Setup**
    - [x] Fork/Import Servo components (Dependencies defined)
    - [x] Basic Window creation (Desktop: Windows, macOS, Linux compatibility)
    - [x] Engine <-> Shell Architecture (Events & Texture Sharing)

## Phase 2: The Shell (In Progress)
- [x] **UI Framework**
    - [x] Native Rust UI (`eframe` / `egui`)
    - [x] Theming Engine (Catppuccin & Zen/Mica effects)
    - [x] Sidebar & Top Bar Implementation
- [ ] **Spaces & Layouts**
    - [x] Logic for grouping Panels (Spaces logic)
    - [x] Basic UI for Spaces (Sidebar)
    - [ ] Split-screen rendering methods

## Phase 3: Extensions & Power Features
- [ ] **Extension Compatibility Layer**
    - [ ] Manifest V3 parser
    - [ ] Bridge `chrome.*` APIs to Wave internal APIs
- [ ] **Sync System** (Firebase)

## Phase 4: Polish & Launch
- [ ] Alpha Release (Windows/Linux)
- [ ] Android Beta
