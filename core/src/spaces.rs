use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Space {
    pub id: String,
    pub name: String,
    pub panels: Vec<Panel>,
    pub layout: Layout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Panel {
    pub id: String,
    pub title: String,
    pub url: String,
    // Future: History stack, zoom level, scroll position
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Layout {
    Single, // One panel full width
    SplitVertical(String, String), // Two panels left/right (IDs)
    SplitHorizontal(String, String), // Two panels top/bottom (IDs)
    Grid(Vec<String>), // N panels in a grid
}

impl Space {
    pub fn new(name: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            panels: Vec::new(),
            layout: Layout::Single,
        }
    }

    pub fn add_panel(&mut self, url: &str) {
        let panel = Panel {
            id: uuid::Uuid::new_v4().to_string(),
            title: "New Panel".to_string(),
            url: url.to_string(),
        };
        self.panels.push(panel);
    }
}
