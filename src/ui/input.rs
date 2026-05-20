use crate::agents::{Agent, AgentInputType};
use crate::ui::{searchable_dropdown, Colors, Typography, Spacing, InputSize, Typefaces};
use egui;
use std::collections::HashMap;

pub struct InputRenderer;

fn bordered_input(
    ui: &mut egui::Ui,
    add_input: impl FnOnce(&mut egui::Ui) -> egui::Response,
) -> egui::Response {
    ui.scope(|ui| {
        ui.style_mut().visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, Colors::BORDER);
        ui.style_mut().visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, Colors::BORDER_HOVER);
        ui.style_mut().visuals.widgets.active.bg_stroke = egui::Stroke::new(2.0, Colors::ACCENT);
        add_input(ui)
    })
    .inner
}

impl InputRenderer {
    pub fn render(
        agent: &Agent,
        values: &mut HashMap<String, String>,
        search_queries: &mut HashMap<String, String>,
        ui: &mut egui::Ui,
    ) {
        for (name, input) in &agent.inputs {
            ui.add_space(Spacing::XXXS);
            
            // Professional field label with required indicator
            let label_text = if input.required.unwrap_or(false) {
                format!("{} *", name)
            } else {
                name.clone()
            };
            
            ui.label(
                egui::RichText::new(label_text)
                    .font(Typefaces::meta(Typography::SM))
                    .color(Colors::MUTED_FOREGROUND)
            );
            
            ui.add_space(Spacing::NONE);

            let value = values.entry(name.clone()).or_insert_with(|| {
                input.default.clone().unwrap_or_default()
            });

            match input.input_type {
                AgentInputType::Text => {
                    let text_edit = egui::TextEdit::singleline(value)
                        .desired_width(ui.available_width())
                        .font(egui::TextStyle::Body)
                        .hint_text("Enter text...")
                        .margin(egui::vec2(InputSize::PADDING_X, InputSize::PADDING_Y));
                    bordered_input(ui, |ui| ui.add(text_edit));
                }
                AgentInputType::Textarea => {
                    let text_edit = egui::TextEdit::multiline(value)
                        .desired_width(ui.available_width())
                        .desired_rows(4)
                        .font(egui::TextStyle::Monospace)
                        .hint_text("Enter details...")
                        .margin(egui::vec2(InputSize::PADDING_X, InputSize::PADDING_Y));
                    bordered_input(ui, |ui| ui.add(text_edit));
                }
                AgentInputType::Select => {
                    if let Some(options) = &input.options {
                        searchable_dropdown(
                            ui,
                            &format!("agent_input_{}", name),
                            value,
                            options,
                            search_queries,
                        );
                    }
                }
                AgentInputType::Number => {
                    let text_edit = egui::TextEdit::singleline(value)
                        .desired_width(ui.available_width())
                        .font(egui::TextStyle::Monospace)
                        .hint_text("0")
                        .margin(egui::vec2(InputSize::PADDING_X, InputSize::PADDING_Y));
                    bordered_input(ui, |ui| ui.add(text_edit));
                }
                AgentInputType::Boolean => {
                    let mut checked = value == "true";
                    if ui.checkbox(
                        &mut checked, 
                        egui::RichText::new("Enabled")
                            .font(Typefaces::body(Typography::BASE))
                    ).changed() {
                        *value = checked.to_string();
                    }
                }
                AgentInputType::File => {
                    ui.horizontal(|ui| {
                        let text_edit = egui::TextEdit::singleline(value)
                            .desired_width((ui.available_width() - 140.0).max(100.0))
                            .font(egui::TextStyle::Monospace)
                            .hint_text("Select file...")
                            .margin(egui::vec2(InputSize::PADDING_X, InputSize::PADDING_Y));
                        bordered_input(ui, |ui| ui.add(text_edit));

                        if ui.button("Browse").clicked() {
                            if let Some(path) = rfd::FileDialog::new().pick_file() {
                                *value = path.display().to_string();
                            }
                        }

                        if !value.is_empty() && ui.button("Clear").clicked() {
                            value.clear();
                        }
                    });
                }
            }

            ui.add_space(Spacing::XXXS);
        }
    }
}
