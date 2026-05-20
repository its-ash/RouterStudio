use crate::ui::{Colors, InputSize, Spacing, Typefaces, Typography};
use egui::{self, PopupCloseBehavior};
use std::collections::HashMap;

pub fn searchable_dropdown(
    ui: &mut egui::Ui,
    id_source: &str,
    selected: &mut String,
    options: &[String],
    search_queries: &mut HashMap<String, String>,
) {
    let selected_gold = egui::Color32::from_rgb(245, 158, 11);
    let selected_gold_bg = egui::Color32::from_rgb(180, 120, 20);
    let search_key = format!("dropdown_search_{}", id_source);
    let search_query = search_queries.entry(search_key).or_default();

    let popup_id = ui.make_persistent_id(format!("{}_popup", id_source));
    let trigger = egui::Button::new(
        egui::RichText::new(selected.as_str())
            .font(Typefaces::body(Typography::BASE))
            .color(selected_gold),
    )
    .fill(Colors::INPUT_BG)
    .stroke(egui::Stroke::new(1.0, Colors::BORDER))
    .rounding(egui::Rounding::same(6.0));

    let trigger_response = ui.add_sized(egui::vec2(ui.available_width(), InputSize::HEIGHT_SM), trigger);

    if trigger_response.clicked() {
        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
    }

    egui::popup_below_widget(
        ui,
        popup_id,
        &trigger_response,
        PopupCloseBehavior::IgnoreClicks,
        |ui| {
            ui.set_min_width(trigger_response.rect.width());

            egui::Frame::none()
                .fill(Colors::SURFACE)
                .stroke(egui::Stroke::NONE)
                .inner_margin(Spacing::XXS)
                .show(ui, |ui| {
                    ui.scope(|ui| {
                        ui.style_mut().visuals.widgets.inactive.bg_stroke =
                            egui::Stroke::new(1.0, Colors::BORDER);
                        ui.style_mut().visuals.widgets.hovered.bg_stroke =
                            egui::Stroke::new(1.0, Colors::BORDER_HOVER);
                        ui.style_mut().visuals.widgets.active.bg_stroke =
                            egui::Stroke::new(2.0, Colors::ACCENT);
                        ui.add(
                            egui::TextEdit::singleline(search_query)
                                .hint_text("Search...")
                                .desired_width(ui.available_width())
                                .font(egui::TextStyle::Body)
                                .margin(egui::vec2(InputSize::PADDING_X * 0.8, InputSize::PADDING_Y * 0.7)),
                        );
                    });

                    ui.add_space(Spacing::XXS);
                    ui.separator();
                    ui.add_space(Spacing::XXS);

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

                    if filtered_options.is_empty() {
                        ui.label(
                            egui::RichText::new("No matches found")
                                .font(Typefaces::meta(Typography::SM))
                                .color(Colors::MUTED_FOREGROUND),
                        );
                    } else {
                        egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                            for option in filtered_options {
                                let is_selected = option.as_str() == selected.as_str();
                                let (rect, response) = ui.allocate_exact_size(
                                    egui::vec2(ui.available_width(), InputSize::HEIGHT_SM - 6.0),
                                    egui::Sense::click(),
                                );

                                let bg_color = if is_selected {
                                    selected_gold_bg
                                } else if response.hovered() {
                                    Colors::SURFACE_ELEVATED
                                } else {
                                    egui::Color32::TRANSPARENT
                                };

                                ui.painter().rect_filled(rect, egui::Rounding::same(4.0), bg_color);

                                ui.painter().text(
                                    egui::pos2(rect.left() + Spacing::SM, rect.center().y),
                                    egui::Align2::LEFT_CENTER,
                                    option,
                                    Typefaces::body(Typography::BASE),
                                    Colors::FOREGROUND,
                                );

                                if response.clicked() {
                                    *selected = option.clone();
                                    search_query.clear();
                                    ui.memory_mut(|mem| mem.close_popup());
                                }
                            }
                        });
                    }
                });
        },
    );
}
