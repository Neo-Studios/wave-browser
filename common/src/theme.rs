use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CatppuccinFlavor {
    Latte,
    Frappe,
    Macchiato,
    Mocha,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AccentColor {
    Rosewater,
    Flamingo,
    Pink,
    Mauve,
    Red,
    Maroon,
    Peach,
    Yellow,
    Green,
    Teal,
    Sky,
    Sapphire,
    Blue,
    Lavender,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub flavor: CatppuccinFlavor,
    pub accent: AccentColor,
    pub use_mica: bool, // Zen mode transparency
    pub background_opacity: f32,
}

impl Theme {
    pub fn default_wave() -> Self {
        Self {
            name: "Wave Default".to_string(),
            flavor: CatppuccinFlavor::Mocha,
            accent: AccentColor::Blue,
            use_mica: true,
            background_opacity: 0.8,
        }
    }

    pub fn zen() -> Self {
        Self {
            name: "Zen Mode".to_string(),
            flavor: CatppuccinFlavor::Macchiato,
            accent: AccentColor::Lavender,
            use_mica: true,
            background_opacity: 0.5, // High transparency
        }
    }
}

// Helper to get hex codes for Catppuccin Mocha (as an example)
impl CatppuccinFlavor {
    pub fn base(&self) -> u32 {
        match self {
            Self::Mocha => 0x1e1e2e,
            Self::Macchiato => 0x24273a,
            Self::Frappe => 0x303446,
            Self::Latte => 0xeff1f5,
        }
    }

    pub fn mantle(&self) -> u32 {
        match self {
            Self::Mocha => 0x181825,
            Self::Macchiato => 0x1e2030,
            Self::Frappe => 0x292c3c,
            Self::Latte => 0xe6e9ef,
        }
    }

    pub fn text(&self) -> u32 {
        match self {
            Self::Mocha => 0xcdd6f4,
            Self::Macchiato => 0xcad3f5,
            Self::Frappe => 0xc6d0f5,
            Self::Latte => 0x4c4f69,
        }
    }
}
