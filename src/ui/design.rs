use egui::{Color32, FontFamily, FontId, Rounding, Shadow, Stroke, Visuals};

pub struct Colors;

impl Colors {
    pub const BACKGROUND: Color32 = Color32::from_rgb(17, 17, 19);
    pub const SURFACE: Color32 = Color32::from_rgb(24, 24, 27);
    pub const SURFACE_ELEVATED: Color32 = Color32::from_rgb(36, 36, 41);
    pub const FOREGROUND: Color32 = Color32::from_rgb(245, 245, 247);
    pub const ACCENT: Color32 = Color32::from_rgb(59, 130, 246);
    pub const ACCENT_HOVER: Color32 = Color32::from_rgb(37, 99, 235);
    pub const ACCENT_FOREGROUND: Color32 = Color32::WHITE;
    pub const MUTED_FOREGROUND: Color32 = Color32::from_rgb(163, 163, 170);
    pub const BORDER: Color32 = Color32::from_rgb(63, 63, 70);
    pub const BORDER_HOVER: Color32 = Color32::from_rgb(82, 82, 91);
    pub const INPUT_BG: Color32 = Color32::from_rgb(30, 30, 34);
    pub const SUCCESS: Color32 = Color32::from_rgb(16, 185, 129);
    pub const ERROR: Color32 = Color32::from_rgb(239, 68, 68);
    pub const WARNING: Color32 = Color32::from_rgb(245, 158, 11);
}

pub struct Typography;

impl Typography {
    pub const XS: f32 = 11.0;
    pub const SM: f32 = 12.0;
    pub const BASE: f32 = 14.0;
    pub const LG: f32 = 16.0;
    pub const XL: f32 = 20.0;
    pub const XXL: f32 = 24.0;
    pub const XXXL: f32 = 32.0;
    pub const XXXXL: f32 = 40.0;
    pub const XXXXXL: f32 = 48.0;
    pub const HERO: f32 = 56.0;
    pub const DISPLAY: f32 = 64.0;
}

pub struct Typefaces;

impl Typefaces {
    pub fn display(size: f32) -> FontId {
        FontId::new(size, FontFamily::Monospace)
    }

    pub fn section(size: f32) -> FontId {
        FontId::new(size, FontFamily::Proportional)
    }

    pub fn body(size: f32) -> FontId {
        FontId::new(size, FontFamily::Proportional)
    }

    pub fn meta(size: f32) -> FontId {
        FontId::new(size, FontFamily::Monospace)
    }
}

pub struct Spacing;

impl Spacing {
    pub const NONE: f32 = 0.0;
    pub const XXXS: f32 = 2.0;
    pub const XXS: f32 = 4.0;
    pub const XS: f32 = 4.0;
    pub const SM: f32 = 8.0;
    pub const BASE: f32 = 12.0;
    pub const LG: f32 = 16.0;
    pub const XL: f32 = 20.0;
    pub const XXL: f32 = 24.0;
    pub const XXXL: f32 = 32.0;
    pub const HUGE: f32 = 40.0;
}

pub struct InputSize;

impl InputSize {
    pub const HEIGHT: f32 = 48.0;
    pub const HEIGHT_SM: f32 = 40.0;
    pub const HEIGHT_LG: f32 = 56.0;
    pub const PADDING_X: f32 = 16.0;
    pub const PADDING_Y: f32 = 10.0;
}

pub struct Radius;

impl Radius {
    pub const SM: f32 = 6.0;
    pub const BASE: f32 = 6.0;
    pub const MD: f32 = 8.0;
    pub const LG: f32 = 8.0;
    pub const XL: f32 = 8.0;
}

pub struct Borders;

impl Borders {
    pub fn hairline() -> Stroke {
        Stroke::new(1.0, Colors::BORDER)
    }
    pub fn thin() -> Stroke {
        Stroke::new(2.0, Colors::BORDER)
    }
    pub fn medium() -> Stroke {
        Stroke::new(2.0, Colors::BORDER)
    }
    pub fn thick() -> Stroke {
        Stroke::new(4.0, Colors::BORDER)
    }
    pub fn hover() -> Stroke {
        Stroke::new(2.0, Colors::BORDER_HOVER)
    }
    pub fn accent() -> Stroke {
        Stroke::new(2.0, Colors::ACCENT)
    }
}

pub struct Shadows;

impl Shadows {
    pub fn card() -> Shadow {
        Shadow::NONE
    }
    pub fn elevated() -> Shadow {
        Shadow::NONE
    }
    pub fn popup() -> Shadow {
        Shadow::NONE
    }
}

pub fn create_monochrome_visuals() -> Visuals {
    Visuals {
        dark_mode: true,
        override_text_color: None,
        panel_fill: Colors::BACKGROUND,
        window_fill: Colors::SURFACE,
        window_stroke: Borders::thin(),
        window_rounding: Rounding::same(Radius::MD),
        window_shadow: Shadows::elevated(),
        popup_shadow: Shadows::popup(),
        widgets: egui::style::Widgets {
            noninteractive: egui::style::WidgetVisuals {
                bg_fill: Colors::SURFACE,
                weak_bg_fill: Colors::SURFACE,
                bg_stroke: Stroke::NONE,
                rounding: Rounding::same(Radius::BASE),
                fg_stroke: Stroke::new(1.0, Colors::FOREGROUND),
                expansion: 0.0,
            },
            inactive: egui::style::WidgetVisuals {
                bg_fill: Colors::INPUT_BG,
                weak_bg_fill: Colors::SURFACE,
                bg_stroke: Stroke::NONE,
                rounding: Rounding::same(Radius::BASE),
                fg_stroke: Stroke::new(1.0, Colors::FOREGROUND),
                expansion: 0.0,
            },
            hovered: egui::style::WidgetVisuals {
                bg_fill: Colors::SURFACE_ELEVATED,
                weak_bg_fill: Colors::SURFACE_ELEVATED,
                bg_stroke: Stroke::NONE,
                rounding: Rounding::same(Radius::BASE),
                fg_stroke: Stroke::new(1.0, Colors::FOREGROUND),
                expansion: 0.0,
            },
            active: egui::style::WidgetVisuals {
                bg_fill: Colors::INPUT_BG,
                weak_bg_fill: Colors::INPUT_BG,
                bg_stroke: Borders::accent(),
                rounding: Rounding::same(Radius::BASE),
                fg_stroke: Stroke::new(1.0, Colors::FOREGROUND),
                expansion: 0.0,
            },
            open: egui::style::WidgetVisuals {
                bg_fill: Colors::SURFACE,
                weak_bg_fill: Colors::SURFACE,
                bg_stroke: Borders::accent(),
                rounding: Rounding::same(Radius::BASE),
                fg_stroke: Stroke::new(1.0, Colors::FOREGROUND),
                expansion: 0.0,
            },
        },
        selection: egui::style::Selection {
            bg_fill: Color32::from_rgba_premultiplied(59, 130, 246, 44),
            stroke: Stroke::new(2.0, Colors::ACCENT),
        },
        hyperlink_color: Colors::ACCENT,
        faint_bg_color: Colors::SURFACE,
        extreme_bg_color: Colors::BACKGROUND,
        code_bg_color: Colors::SURFACE,
        warn_fg_color: Colors::WARNING,
        error_fg_color: Colors::ERROR,
        window_highlight_topmost: true,
        menu_rounding: Rounding::same(Radius::BASE),
        striped: false,
        slider_trailing_fill: false,
        handle_shape: egui::style::HandleShape::Rect { aspect_ratio: 0.8 },
        interact_cursor: Some(egui::CursorIcon::PointingHand),
        image_loading_spinners: true,
        numeric_color_space: egui::style::NumericColorSpace::GammaByte,
        ..Default::default()
    }
}

pub fn configure_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    style.spacing.item_spacing = egui::vec2(Spacing::SM, Spacing::SM);
    style.spacing.window_margin = egui::Margin::same(Spacing::BASE);
    style.spacing.button_padding = egui::vec2(InputSize::PADDING_X, InputSize::PADDING_Y);
    style.spacing.indent = Spacing::BASE;
    style.spacing.slider_width = 220.0;
    style.spacing.combo_width = 220.0;
    style.spacing.combo_height = InputSize::HEIGHT;

    style.interaction.selectable_labels = true;
    style.interaction.show_tooltips_only_when_still = false;

    let mut text_styles = std::collections::BTreeMap::new();

    text_styles.insert(
        egui::TextStyle::Small,
        Typefaces::meta(Typography::SM),
    );

    text_styles.insert(
        egui::TextStyle::Body,
        Typefaces::body(Typography::BASE),
    );

    text_styles.insert(
        egui::TextStyle::Button,
        Typefaces::meta(Typography::BASE),
    );

    text_styles.insert(
        egui::TextStyle::Heading,
        Typefaces::section(Typography::XXL),
    );

    text_styles.insert(
        egui::TextStyle::Monospace,
        FontId::new(Typography::BASE, FontFamily::Monospace),
    );

    style.text_styles = text_styles;

    style.visuals = create_monochrome_visuals();

    style.animation_time = 0.12;

    ctx.set_style(style);
}

pub fn horizontal_rule(ui: &mut egui::Ui, thickness: f32) {
    ui.add_space(Spacing::XXS);
    let rect = ui.available_rect_before_wrap();
    ui.painter().hline(
        rect.x_range(),
        ui.cursor().min.y,
        Stroke::new(thickness, Colors::BORDER),
    );
    ui.add_space(Spacing::XXS);
}

pub fn vertical_rule(ui: &mut egui::Ui, thickness: f32, height: f32) {
    let rect = ui.available_rect_before_wrap();
    ui.painter().vline(
        rect.left(),
        egui::Rangef::new(rect.top(), rect.top() + height),
        Stroke::new(thickness, Colors::BORDER),
    );
}

pub fn card_frame(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::none()
        .fill(Colors::SURFACE)
        .stroke(Stroke::NONE)
        .inner_margin(Spacing::BASE)
        .rounding(Rounding::same(Radius::MD))
        .shadow(Shadow::NONE)
        .show(ui, add_contents);
}

pub fn inverted_card_frame(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::none()
        .fill(Colors::ACCENT)
        .stroke(Stroke::NONE)
        .inner_margin(Spacing::BASE)
        .rounding(Rounding::same(Radius::MD))
        .shadow(Shadow::NONE)
        .show(ui, |ui| {
            ui.visuals_mut().override_text_color = Some(Colors::ACCENT_FOREGROUND);
            add_contents(ui);
        });
}
