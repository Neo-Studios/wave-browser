# Wave Engine Build Instructions

Integrating Servo is complex. This document tracks the dependencies required.

## Prerequisites (Windows)
1.  **Python 3.11+**
2.  **Visual Studio 2022** with C++ Desktop Development tools.
3.  **LLVM/Clang** (Servo requires `libclang`).
    *   Set `LIBCLANG_PATH` environment variable.
4.  **GStreamer** (for media playback).
5.  **Perl** (Strawberry Perl).

## Integration Strategy
We are pulling `servo` from git in `core/Cargo.toml`.
*   *Warning:* Servo takes 30-60 minutes to compile from scratch.
*   *Warning:* Embedding Servo in `egui` requires sharing the OpenGL context, which is tricky. `wgpu` (used by recent eframe) and Servo's `webrender` (OpenGL) don't always play nice together easily.

## Planned Architecture for Integration
1.  **Shared Texture:** Wave Engine renders the web page to an off-screen GL Texture.
2.  **Egui Image:** The shell takes that texture ID and renders it as a simple Image widget in the center of the screen.
3.  **Input Forwarding:** Mouse clicks on the Image widget are translated to Servo input events.
