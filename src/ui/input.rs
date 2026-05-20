use crate::agents::{Agent, AgentInputType};
use crate::ui::{Colors, Typography, Spacing, InputSize};
use egui;
use std::collections::HashMap;

pub struct InputRenderer;

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
                    .size(Typography::SM)
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
                    ui.add(text_edit);
                }
                AgentInputType::Textarea => {
                    let text_edit = egui::TextEdit::multiline(value)
                        .desired_width(ui.available_width())
                        .desired_rows(4)
                        .font(egui::TextStyle::Monospace)
                        .hint_text("Enter details...")
                        .margin(egui::vec2(InputSize::PADDING_X, InputSize::PADDING_Y));
                    ui.add(text_edit);
                }
                AgentInputType::Select => {
                    if let Some(options) = &input.options {
                        let search_key = format!("search_{}", name);
                        let search_query = search_queries.entry(search_key).or_insert_with(String::new);
                        
                        egui::ComboBox::from_id_salt(name)
                            .selected_text(
                                egui::RichText::new(value.as_str())
                                    .size(Typography::BASE)
                            )
                            .width(ui.available_width())
                            .height(320.0)
                            .show_ui(ui, |ui| {
                                // Search input at the top
                                ui.add_space(Spacing::NONE);
                                let search_response = ui.add(
                                    egui::TextEdit::singleline(search_query)
                                        .hint_text("Search...")
                                        .desired_width(ui.available_width() - Spacing::XXS)
                                        .font(egui::TextStyle::Body)
                                        .margin(egui::vec2(InputSize::PADDING_X, InputSize::PADDING_Y))
                                );
                                
                                // Auto-focus search when dropdown opens
                                if ui.memory(|mem| mem.has_focus(search_response.id)) {
                                    search_response.request_focus();
                                }
                                
                                ui.add_space(Spacing::XXXS);
                                ui.separator();
                                ui.add_space(Spacing::NONE);
                                
                                // Filter options based on search query
                                let filtered_options: Vec<&String> = options
                                    .iter()
                                    .filter(|opt| {
                                        if search_query.is_empty() {
                                            true
                                        } else {
                                            opt.to_lowercase().contains(&search_query.to_lowercase())
                                        }
                                    })
                                    .collect();
                                
                                // Show filtered options
                                if filtered_options.is_empty() {
                                    ui.label(
                                        egui::RichText::new("No matches found")
                                            .size(Typography::SM)
                                            .color(Colors::MUTED_FOREGROUND)
                                    );
                                } else {
                                    for option in filtered_options {
                                        if ui.selectable_value(
                                            value, 
                                            option.clone(), 
                                            egui::RichText::new(option)
                                                .size(Typography::BASE)
                                        ).clicked() {
                                            // Clear search when option is selected
                                            search_query.clear();
                                        }
                                    }
                                }
                            });
                    }
                }
                AgentInputType::Number => {
                    let text_edit = egui::TextEdit::singleline(value)
                        .desired_width(ui.available_width())
                        .font(egui::TextStyle::Monospace)
                        .hint_text("0")
                        .margin(egui::vec2(InputSize::PADDING_X, InputSize::PADDING_Y));
                    ui.add(text_edit);
                }
                AgentInputType::Boolean => {
                    let mut checked = value == "true";
                    if ui.checkbox(
                        &mut checked, 
                        egui::RichText::new("Enabled")
                            .size(Typography::BASE)
                    ).changed() {
                        *value = checked.to_string();
                    }
                }
            }

            ui.add_space(Spacing::XXXS);
        }
    }
}
