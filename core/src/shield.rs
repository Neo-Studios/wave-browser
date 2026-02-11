use adblock::engine::Engine;
use adblock::lists::{FilterSet, ParseOptions};
use adblock::request::Request;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Decision {
    Allow,
    Block,
    Sanitize, // Remove tracking parameters but allow the request
}

pub struct WaveShield {
    enabled: bool,
    engine: Arc<Mutex<Engine>>,
}

impl WaveShield {
    pub fn new() -> Self {
        // Initialize with an empty filter set for now
        // In a real scenario, we would load EasyList etc. here
        let filter_set = FilterSet::new(false);
        let engine = Engine::from_filter_set(filter_set, true);

        Self { 
            enabled: true,
            engine: Arc::new(Mutex::new(engine)), 
        }
    }

    pub fn load_filters(&self, filter_lines: Vec<String>) {
        let mut engine = self.engine.lock().unwrap();
        let mut filter_set = FilterSet::new(false);
        
        // Add filters to the set
        filter_set.add_filters(&filter_lines, ParseOptions::default());
        
        // Rebuild the engine with new filters
        *engine = Engine::from_filter_set(filter_set, true);
    }

    pub fn should_allow_request(&self, url: &str, source_url: &str, resource_type: &str) -> Decision {
        if !self.enabled {
            return Decision::Allow;
        }

        let engine = self.engine.lock().unwrap();
        
        // Check using the adblock engine
        // 'source_url' is the page making the request (e.g. "https://example.com")
        // 'url' is the request being made (e.g. "https://ads.doubleclick.net/...")
        let request = Request::new(url, source_url, resource_type).unwrap_or_else(|_| Request::new("http://example.com", "", "").unwrap());
        let check_result = engine.check_network_request(&request);

        if check_result.matched {
            return Decision::Block;
        }

        Decision::Allow
    }
}

pub enum ResourceType {
    Script,
    Image,
    XHR,
    Frame,
    Stylesheet,
    Other,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shield_blocking() {
        let shield = WaveShield::new();
        
        // Load a simple blocking rule
        // ||example.com^ matches any URL typically from example.com
        let rules = vec![
            "||ads.badsite.com^".to_string(),
        ];
        shield.load_filters(rules);

        // Test Blocking
        // url: the bad ad url
        // source: the site we are visiting
        // type: script
        let decision = shield.should_allow_request(
            "https://ads.badsite.com/banner.js", 
            "https://mysite.com", 
            "script"
        );

        match decision {
            Decision::Block => assert!(true),
            _ => panic!("Should have blocked the ad url"),
        }

        // Test Allowing
        let decision_allow = shield.should_allow_request(
            "https://mysite.com/style.css", 
            "https://mysite.com", 
            "stylesheet"
        );

        match decision_allow {
            Decision::Allow => assert!(true),
            _ => panic!("Should have allowed the safe url"),
        }
    }
}
