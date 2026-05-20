mod app;
mod agents;
mod openrouter;
mod render;
mod storage;
mod settings;
mod ui;

use app::OpenAgentApp;
use ui::configure_style;

fn load_icon() -> Option<egui::IconData> {
    let icon_path = std::path::Path::new("RouterStudio.png");
    
    if !icon_path.exists() {
        eprintln!("Warning: RouterStudio.png not found");
        return None;
    }
    
    match image::open(icon_path) {
        Ok(img) => {
            let rgba = img.to_rgba8();
            let (width, height) = rgba.dimensions();
            
            Some(egui::IconData {
                rgba: rgba.into_raw(),
                width: width,
                height: height,
            })
        }
        Err(e) => {
            eprintln!("Error loading icon: {}", e);
            None
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([1000.0, 700.0])
            .with_title("OPENAGENT")
            .with_icon(
                load_icon().unwrap_or_else(|| {
                    // Fallback: indigo accent color icon
                    egui::IconData {
                        rgba: vec![99u8, 102, 241, 255].repeat(32 * 32), // Indigo-500
                        width: 32,
                        height: 32,
                    }
                })
            ),
        ..Default::default()
    };

    eframe::run_native(
        "OPENAGENT",
        options,
        Box::new(|cc| {
            // Configure Minimalist Monochrome design system
            configure_style(&cc.egui_ctx);
            Ok(Box::new(OpenAgentApp::new(cc)))
        }),
    )
}
