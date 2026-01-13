pub mod shield;
pub mod spaces;

use std::sync::Arc;
use url::Url;

/// Event types that the shell (UI) sends to the engine.
#[derive(Debug, Clone)]
pub enum EngineEvent {
    LoadUrl(String),
    Resize { width: u32, height: u32 },
    InputClick { x: f32, y: f32 },
    InputKey { keycode: u32 },
    Back,
    Forward,
    Refresh,
    Stop,
}

/// Helper struct to manage the Servo instance and communication channels.
pub struct WaveEngine {
    // In a real implementation:
    // servo: Option<servo::Servo<WindowMethods>>,
    // events_proxy: EventLoopProxy<...>,
    current_url: String,
    is_loading: bool,
}

impl WaveEngine {
    /// Create a new instance of the Wave Engine.
    /// 
    /// # Arguments
    /// * `gl_context_pointer`: Raw pointer to the OpenGL context (needed for Servo share)
    pub fn new(_gl_context_pointer: *mut std::ffi::c_void) -> Self {
        log::info!("Initializing Wave Engine (Servo-based)...");
        
        // Setup Servo configuration
        // let opts = servo::config::opts::default_opts();
        // servo::init(opts);

        Self {
            current_url: String::from("about:blank"),
            is_loading: false,
        }
    }

    /// Dispatch an event to the underlying engine
    pub fn dispatch(&mut self, event: EngineEvent) {
        match event {
            EngineEvent::LoadUrl(url) => self.load_url(&url),
            EngineEvent::Resize { width, height } => self.resize(width, height),
            EngineEvent::Back => log::info!("Engine: Back"), // self.constellation.send(Msg::Back)
            EngineEvent::Forward => log::info!("Engine: Forward"),
            EngineEvent::Refresh => log::info!("Engine: Refresh"),
            EngineEvent::Stop => log::info!("Engine: Stop"),
            EngineEvent::InputClick { x, y } => log::info!("Engine: Click at {}, {}", x, y),
            EngineEvent::InputKey { keycode } => log::info!("Engine: Key {}", keycode),
        }
    }

    fn load_url(&mut self, url: &str) {
        // Validate URL
        let parsed = match Url::parse(url) {
            Ok(u) => u,
            Err(_) => match Url::parse(&format!("https://{}", url)) {
                Ok(u) => u,
                Err(_) => return, // Handle search query fallback here
            },
        };

        self.current_url = parsed.to_string();
        self.is_loading = true;
        log::info!("Engine loading URL: {}", self.current_url);
        
        // In real Servo:
        // self.constellation.send(ConstellationMsg::Load(parsed_url));
    }

    fn resize(&self, width: u32, height: u32) {
        log::debug!("Engine resizing to {}x{}", width, height);
        // self.compositor.send(CompositorMsg::Resize(width, height));
    }

    pub fn get_url(&self) -> &str {
        &self.current_url
    }

    pub fn is_loading(&self) -> bool {
        self.is_loading
    }

    /// Trigger the engine to paint the current frame to the active OpenGL context.
    /// 
    /// # Arguments
    /// * `_time`: Time in seconds (used for animations if needed)
    pub fn paint(&self, _time: f64) {
        // In a real Servo embedding:
        // 1. We assume the embedder (render_glue) has bound the FBO.
        // 2. We tell Servo to composition existing layers.
        
        // self.servo_instance.present();
        
        // SIMULATION FOR USER:
        // Since we cannot compile the full Servo engine in this environment,
        // we emit the command that WOULD happen.
        // log::trace!("Servo::paint() called");
        
        // If we had the servo handle:
        // servo.recomposite();
    }
}
