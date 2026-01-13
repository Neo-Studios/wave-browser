use constants::WINDOW_TITLE;
use eframe::{egui, glow};
#[cfg(target_os = "windows")]
use window_vibrancy::{apply_mica, apply_acrylic, Color};
#[cfg(target_os = "macos")]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
use std::sync::Arc;

use wave_core::{shield::WaveShield, WaveEngine, EngineEvent, spaces::Space};
use wave_common::theme::{Theme, CatppuccinFlavor};

mod constants {
    pub const WINDOW_TITLE: &str = "Wave Browser";
}
mod render_glue;

struct WaveApp {
    shield: WaveShield,
    engine: WaveEngine,
    theme: Theme,
    renderer: Option<render_glue::OffscreenRenderer>,
    url_input: String,
    // Spaces Logic
    spaces: Vec<Space>,
    active_space_idx: usize,
}

impl WaveApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let theme = Theme::default_wave();
        
        let mut visuals = egui::Visuals::dark();
        let bg_color = egui::Color32::from_rgb(30, 30, 46);
        visuals.panel_fill = bg_color;
        visuals.window_fill = bg_color;
        cc.egui_ctx.set_visuals(visuals);

        // Initialize OpenGL Glue
        let renderer = if let Some(gl) = cc.gl.as_ref() {
            Some(render_glue::OffscreenRenderer::new(gl.clone(), 800, 600))
        } else {
            None
        };

        // Initialize Engine
        let engine = WaveEngine::new(std::ptr::null_mut());

        // Initialize Default Spaces
        let mut work_space = Space::new("Work");
        work_space.add_panel("https://github.com");
        work_space.add_panel("https://slack.com");

        let mut personal_space = Space::new("Personal");
        personal_space.add_panel("https://youtube.com");

        Self {
            shield: WaveShield::new(),
            engine,
            theme,
            renderer,
            url_input: String::from("https://example.com"),
            spaces: vec![work_space, personal_space],
            active_space_idx: 0,
        }
    }
}

impl eframe::App for WaveApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        frame.winit_window().map(|window| {
            #[cfg(target_os = "windows")]
            if self.theme.use_mica {
                 let _ = apply_mica(window, None);
            }
            #[cfg(target_os = "macos")]
            if self.theme.use_mica {
                // MacOS Vibrancy (Zen Mode)
                let _ = apply_vibrancy(window, NSVisualEffectMaterial::HudWindow, None, None)
                    .map_err(|e| log::warn!("Failed to apply vibrancy: {:?}", e));
            }
        });

        // Sidebar
        egui::SidePanel::left("wave_sidebar")
            .resizable(true)
            .default_width(250.0)
            .show(ctx, |ui| {
                ui.add_space(20.0);
                ui.heading("Wave");
                ui.separator();
                ui.label("OPEN PANELS");
                if ui.button(" 🏠  Home ").clicked() {
                    self.engine.dispatch(EngineEvent::LoadUrl("about:home".into()));
                    self.url_input = "about:home".into();
                }
                if ui.button(" 🔍  Search ").clicked() {
                    // Focus search
                }
                ui.add_space(20.0);
                ui.label("SPACES");
                
                // Dynamic Spaces List
                let mut space_clicked_idx = None;
                for (idx, space) in self.spaces.iter().enumerate() {
                    let is_active = idx == self.active_space_idx;
                    let label = if is_active {
                        format!("👉 {}", space.name)
                    } else {
                        format!("  📁 {}", space.name)
                    };
                    
                    if ui.selectable_label(is_active, label).clicked() {
                        space_clicked_idx = Some(idx);
                    }
                    
                    if is_active {
                        // Show panels in this space
                        for panel in &space.panels {
                            ui.label(format!("    📄 {}", panel.title));
                        }
                    }
                }

                if let Some(idx) = space_clicked_idx {
                    self.active_space_idx = idx;
                }

                ui.add_space(20.0);
                if ui.button(" + New Space ").clicked() {
                    self.spaces.push(Space::new("New Space"));
                }
                
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.label(format!("Shield: {}", if true { "Active" } else { "Off" }));
                    ui.separator();
                });
            });

        // Top Bar
        egui::TopBottomPanel::top("wave_topbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button(" < ").clicked() {
                    self.engine.dispatch(EngineEvent::Back);
                }
                if ui.button(" > ").clicked() {
                    self.engine.dispatch(EngineEvent::Forward);
                }
                if ui.button(" ↻ ").clicked() {
                    self.engine.dispatch(EngineEvent::Refresh);
                }
                
                // URL Bar Logic
                let response = ui.add(egui::TextEdit::singleline(&mut self.url_input).desired_width(f32::INFINITY));
                if response.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.engine.dispatch(EngineEvent::LoadUrl(self.url_input.clone()));
                }
            });
        });

        // Central Content
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();
            let (w, h) = (available_size.x as i32, available_size.y as i32);

            if let Some(renderer) = &mut self.renderer {
                if renderer.width != w || renderer.height != h {
                    renderer.resize(w, h);
                    self.engine.dispatch(EngineEvent::Resize { width: w as u32, height: h as u32 });
                }

                // EXECUTE ENGINE PAINT
                // 1. Prepare offscreen buffer
                renderer.begin_paint();
                
                // 2. Tell Wave Engine (Servo) to draw into the current context
                let time = ctx.input(|i| i.time);
                self.engine.paint(time);
                
                // 3. Finish up
                renderer.end_paint();

                let rect = ui.max_rect();
                
                // Capture clicks on the web view
                let response = ui.interact(rect, ui.id(), egui::Sense::click());
                if response.clicked() {
                    if let Some(pos) = response.hover_pos() {
                        self.engine.dispatch(EngineEvent::InputClick { 
                            x: pos.x - rect.left(), 
                            y: pos.y - rect.top() 
                        });
                    }
                }

                ui.painter().add(egui::PaintCallback {
                    rect,
                    callback: Arc::new(eframe::egui_glow::CallbackFn::new(move |_info, _painter| {
                         // Real engine blitting would involve:
                         // 1. Binding the renderer's texture
                         // 2. Drawing a textured quad over 'rect'
                    })),
                });
                
                // Overlay info (simulating what the web page might be)
                ui.centered_and_justified(|ui| {
                     ui.label(format!("Browsing: {}\nEngine Surface: {}x{}", self.engine.get_url(), w, h));
                });
            }
        });
        
        ctx.request_repaint();
    }
    
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() 
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_title(WINDOW_TITLE)
            .with_transparent(true), // Crucial for Vibrancy
        ..Default::default()
    };

    eframe::run_native(
        WINDOW_TITLE,
        native_options,
        Box::new(|cc| Box::new(WaveApp::new(cc))),
    )
}
