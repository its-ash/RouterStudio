use crate::ui::{Colors, Radius, Typography};
use egui::{self, Rounding, Stroke};

#[derive(Clone, Copy)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Success,
    Danger,
}

pub fn app_button(ui: &mut egui::Ui, text: &str, variant: ButtonVariant) -> egui::Response {
    let (base_fill, hover_fill, active_fill, text_color) = match variant {
        ButtonVariant::Primary => (
            Colors::ACCENT,
            Colors::ACCENT_HOVER,
            egui::Color32::from_rgb(29, 78, 216),
            Colors::ACCENT_FOREGROUND,
        ),
        ButtonVariant::Secondary => (
            Colors::SURFACE,
            Colors::SURFACE_ELEVATED,
            Colors::BORDER_HOVER,
            Colors::FOREGROUND,
        ),
        ButtonVariant::Success => (
            Colors::SUCCESS,
            egui::Color32::from_rgb(5, 150, 105),
            egui::Color32::from_rgb(4, 120, 87),
            Colors::ACCENT_FOREGROUND,
        ),
        ButtonVariant::Danger => (
            Colors::ERROR,
            egui::Color32::from_rgb(220, 38, 38),
            egui::Color32::from_rgb(185, 28, 28),
            Colors::ACCENT_FOREGROUND,
        ),
    };

    let response = ui
        .scope(|ui| {
            ui.style_mut().visuals.widgets.inactive.bg_fill = base_fill;
            ui.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::NONE;
            ui.style_mut().visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, text_color);
            ui.style_mut().visuals.widgets.inactive.expansion = 0.0;

            ui.style_mut().visuals.widgets.hovered.bg_fill = hover_fill;
            ui.style_mut().visuals.widgets.hovered.bg_stroke = Stroke::NONE;
            ui.style_mut().visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, text_color);
            ui.style_mut().visuals.widgets.hovered.expansion = 1.0;

            ui.style_mut().visuals.widgets.active.bg_fill = active_fill;
            ui.style_mut().visuals.widgets.active.bg_stroke = Stroke::NONE;
            ui.style_mut().visuals.widgets.active.fg_stroke = Stroke::new(1.0, text_color);
            ui.style_mut().visuals.widgets.active.expansion = 0.0;

            ui.style_mut().visuals.widgets.open = ui.style().visuals.widgets.hovered;

            let button = egui::Button::new(
                egui::RichText::new(text)
                    .size(Typography::BASE)
                    .color(text_color),
            )
            .fill(base_fill)
            .stroke(Stroke::NONE)
            .rounding(Rounding::same(Radius::BASE));

            ui.add(button)
        })
        .inner;

    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    if response.hovered() {
        ui.painter().rect_stroke(
            response.rect,
            Rounding::same(Radius::BASE),
            Stroke::new(1.5, Colors::ACCENT),
        );
    }

    if response.hovered() && !response.is_pointer_button_down_on() {
        ui.painter().rect_filled(
            response.rect,
            Rounding::same(Radius::BASE),
            egui::Color32::from_white_alpha(14),
        );
    }

    if response.is_pointer_button_down_on() {
        ui.painter().rect_filled(
            response.rect,
            Rounding::same(Radius::BASE),
            egui::Color32::from_black_alpha(24),
        );
    }

    response
}
