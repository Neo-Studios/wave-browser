# Wave: The Breatheable Browser - Product Specification

## 1. Vision
Wave aims to be the "breatheable" browser—a lightweight, privacy-first, and highly customizable web browser that respects user attention. It replaces the clutter of modern browsers with a clean, spatial interface and robust built-in protections.

## 2. Core Pillars
*   **Privacy & Security:** Not an afterthought. Ad-blocking, tracker-blocking, and fingerprinting protection are native.
*   **Performance:** A custom engine to ensure lightweight resource usage.
*   **Adaptability:** "Something for everyone" via distinct modes (Focus, Dev, etc.).
*   **Open Source:** Transparency and community-driven development.

## 3. Key Terminology
*   **Spaces:** Top-level containers for context switching (e.g., "Work", "Personal", "Research"). A Space contains its own set of Panels and Layouts.
*   **Panels:** Replaces "Tabs". 
    *   Panels can be individual web pages.
    *   **Layouts:** Panels can be combined into split-screen or tiled layouts within a single view. (e.g., A coding tutorial on the left, a playground editor on the right).

## 4. Feature Set

### 4.1. Privacy Shield
*   **Wave Shield:** Custom-built ad and tracker blocking engine.
    *   Replaces generic "Native Ad-Blocker".
    *   Deep integration with the Wave Engine for maximum performance.
*   **True Incognito:** 
    *   **Network:** Optional routing via Tor or built-in VPN.
    *   **Isolation:** RAM-only session handling. No data touches the disk.
    *   **Anti-Fingerprinting:** 
        *   Spoofs canvas, audio context, and screen resolution.
        *   **User Agent:** Explicitly set to `Wave (<Operating System>)` (e.g., `Wave (Windows 11)`, `Wave (Linux)`). This identifies the browser without leaking version numbers or specific build details.

### 4.2. Universal Extensions
*   **Goal:** Drop-in compatibility for existing ecosystems.
*   **Support:** 
    *   Chrome Extensions (Manifest V3)
    *   Firefox Add-ons (WebExtensions)
*   **Implication:** A robust compatibility layer to map `chrome.*` and `browser.*` APIs to the Wave Engine.

### 4.3. Cloud & Sync
*   **Provider:** Firebase.
*   **Authentication:** 
    *   GitHub
    *   Apple
    *   Google
*   **Sync Data:** Spaces, Active Panels/Layouts, History, Bookmarks, and Settings across devices.

### 4.4. Modes
*   **Focus Mode:** 
    *   Hides non-essential UI elements.
    *   Blocks notifications.
    *   Potentially limits access to "distracting" sites defined by the user.
*   **Dev Mode:**
    *   First-class developer tools integration.
    *   Network throttling simulatiors?
    *   Mobile view toggles?
    *   Integration with local IDEs?

### 4.3. Interface
*   **Vertical/Spatial Panels:** Moving away from the horizontal strip.
*   **Workspaces:** Grouping Panels by context.

## 5. Platform Support
*   **Initial Launch:** Windows, MacOS, Linux, Android.
*   **Strategy:** Cross-platform from Day 1 using a Monorepo structure.

## 6. Business Model
*   Open Source (License TBD - likely MPL, GPL, or MIT).
