use crate::ui::{Colors, Typography, Spacing, card_frame, primary_button};
use egui;
use image::DynamicImage;
use std::sync::Arc;

#[derive(Clone)]
pub enum OutputType {
    Text(String),
    Markdown(String),
    Image(Arc<DynamicImage>),
}

pub struct OutputRenderer;

impl OutputRenderer {
    pub fn render(output: &OutputType, ui: &mut egui::Ui) {
        match output {
            OutputType::Text(text) => {
                card_frame(ui, |ui| {
                    ui.label(
                        egui::RichText::new(text)
                            .size(Typography::LG)
                            .color(Colors::FOREGROUND)
                    );
                });
            }
            OutputType::Markdown(md) => {
                // Simple markdown rendering as formatted text
                card_frame(ui, |ui| {
                    // Split by paragraphs and format
                    for paragraph in md.split("\n\n") {
                        if paragraph.trim().is_empty() {
                            continue;
                        }
                        
                        // Check if it's a heading
                        if paragraph.starts_with("# ") {
                            ui.label(
                                egui::RichText::new(&paragraph[2..])
                                    .size(Typography::XXXXL)
                                    .strong()
                            );
                        } else if paragraph.starts_with("## ") {
                            ui.label(
                                egui::RichText::new(&paragraph[3..])
                                    .size(Typography::XXXL)
                                    .strong()
                            );
                        } else if paragraph.starts_with("### ") {
                            ui.label(
                                egui::RichText::new(&paragraph[4..])
                                    .size(Typography::XXL)
                                    .strong()
                            );
                        } else {
                            ui.label(
                                egui::RichText::new(paragraph)
                                    .size(Typography::LG)
                            );
                        }
                        
                        ui.add_space(Spacing::BASE);
                    }
                });
            }
            OutputType::Image(img) => {
                ui.add_space(Spacing::BASE);
                
                let rgba = img.to_rgba8();
                let pixels = rgba.as_flat_samples();
                
                let color_image = egui::ColorImage::from_rgba_unmultiplied(
                    [img.width() as usize, img.height() as usize],
                    pixels.as_slice(),
                );

                let texture = ui.ctx().load_texture(
                    "generated_image",
                    color_image,
                    egui::TextureOptions::default(),
                );

                // Image with sharp border
                egui::Frame::none()
                    .stroke(egui::Stroke::new(2.0, Colors::BORDER))
                    .show(ui, |ui| {
                        ui.image(&texture);
                    });
                
                ui.add_space(Spacing::XS);
                
                // Download button
                if primary_button(ui, "DOWNLOAD IMAGE").clicked() {
                    let img_clone = Arc::clone(img);
                    std::thread::spawn(move || {
                        if let Some(path) = rfd::FileDialog::new()
                            .set_file_name("generated_image.png")
                            .add_filter("PNG Image", &["png"])
                            .add_filter("JPEG Image", &["jpg", "jpeg"])
                            .save_file()
                        {
                            let extension = path.extension()
                                .and_then(|e| e.to_str())
                                .unwrap_or("png");
                            
                            let result = match extension.to_lowercase().as_str() {
                                "jpg" | "jpeg" => img_clone.save_with_format(&path, image::ImageFormat::Jpeg),
                                _ => img_clone.save_with_format(&path, image::ImageFormat::Png),
                            };
                            
                            if let Err(e) = result {
                                eprintln!("Failed to save image: {}", e);
                            }
                        }
                    });
                }
            }
        }
    }
}
