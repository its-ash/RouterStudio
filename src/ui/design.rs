/// Modern Professional Design System
/// Polished, premium UX inspired by Linear, Raycast, Cursor, Notion

use egui::{Color32, FontFamily, FontId, Rounding, Shadow, Stroke, Visuals};

/// Refined dark color palette - professional and elegant
pub struct Colors;

impl Colors {
    /// Deep charcoal background
    pub const BACKGROUND: Color32 = Color32::from_rgb(17, 17, 19);
    
    /// Surface panels (slightly lighter)
    pub const SURFACE: Color32 = Color32::from_rgb(24, 24, 27);
    
    /// Elevated surfaces
    pub const SURFACE_ELEVATED: Color32 = Color32::from_rgb(32, 32, 36);
    
    /// Primary foreground text
    pub const FOREGROUND: Color32 = Color32::from_rgb(245, 245, 247);
    
    /// Elegant blue-purple accent
    pub const ACCENT: Color32 = Color32::from_rgb(99, 102, 241); // Indigo-500
    
    /// Accent hover state
    pub const ACCENT_HOVER: Color32 = Color32::from_rgb(129, 140, 248); // Indigo-400
    
    /// White text on accent
    pub const ACCENT_FOREGROUND: Color32 = Color32::WHITE;
    
    /// Muted text (descriptions, labels)
    pub const MUTED_FOREGROUND: Color32 = Color32::from_rgb(163, 163, 163);
    
    /// Subtle borders
    pub const BORDER: Color32 = Color32::from_rgb(45, 45, 48);
    
    /// Border on hover
    pub const BORDER_HOVER: Color32 = Color32::from_rgb(63, 63, 70);
    
    /// Input background
    pub const INPUT_BG: Color32 = Color32::from_rgb(24, 24, 27);
    
    /// Success green
    pub const SUCCESS: Color32 = Color32::from_rgb(34, 197, 94);
    
    /// Error red
    pub const ERROR: Color32 = Color32::from_rgb(239, 68, 68);
    
    /// Warning amber
    pub const WARNING: Color32 = Color32::from_rgb(251, 146, 60);
}

/// Professional typography scale - balanced and readable
pub struct Typography;

impl Typography {
    /// Small labels (11px)
    pub const XS: f32 = 11.0;
    
    /// Labels and metadata (13px)
    pub const SM: f32 = 13.0;
    
    /// Base body text (14px)
    pub const BASE: f32 = 14.0;
    
    /// Large body text (16px)
    pub const LG: f32 = 16.0;
    
    /// Section headers (18px)
    pub const XL: f32 = 18.0;
    
    /// Page titles (24px)
    pub const XXL: f32 = 24.0;
    
    /// Large headings (32px)
    pub const XXXL: f32 = 32.0;
    
    /// Hero text (40px)
    pub const XXXXL: f32 = 40.0;
    
    /// Display text (48px)
    pub const XXXXXL: f32 = 48.0;
    
    /// Massive display (56px)
    pub const HERO: f32 = 56.0;
    
    /// Ultra large (64px)
    pub const DISPLAY: f32 = 64.0;
}

/// Spacing scale - compact efficient
pub struct Spacing;

impl Spacing {
    pub const NONE: f32 = 0.0;
    pub const XXXS: f32 = 1.0;
    pub const XXS: f32 = 2.0;
    pub const XS: f32 = 4.0;
    pub const SM: f32 = 6.0;
    pub const BASE: f32 = 8.0;
    pub const LG: f32 = 10.0;
    pub const XL: f32 = 12.0;
    pub const XXL: f32 = 16.0;
    pub const XXXL: f32 = 20.0;
    pub const HUGE: f32 = 24.0;
}

/// Input dimensions - comfortable professional sizing
pub struct InputSize;

impl InputSize {
    /// Standard input height
    pub const HEIGHT: f32 = 38.0;
    
    /// Small input height
    pub const HEIGHT_SM: f32 = 32.0;
    
    /// Large input height
    pub const HEIGHT_LG: f32 = 44.0;
    
    /// Input horizontal padding
    pub const PADDING_X: f32 = 10.0;
    
    /// Input vertical padding
    pub const PADDING_Y: f32 = 8.0;
}

/// Border radii - smooth and modern
pub struct Radius;

impl Radius {
    /// Small radius (6px)
    pub const SM: f32 = 6.0;
    
    /// Base radius (10px)
    pub const BASE: f32 = 10.0;
    
    /// Medium radius (12px)
    pub const MD: f32 = 12.0;
    
    /// Large radius (16px)
    pub const LG: f32 = 16.0;
    
    /// Extra large radius (20px)
    pub const XL: f32 = 20.0;
}

/// Elegant border styles with subtle emphasis
pub struct Borders;

impl Borders {
    /// Hairline border (0.5px)
    pub fn hairline() -> Stroke {
        Stroke::new(0.5, Colors::BORDER)
    }
    
    /// Standard thin border (1px)
    pub fn thin() -> Stroke {
        Stroke::new(1.0, Colors::BORDER)
    }
    
    /// Medium emphasis (1.5px)
    pub fn medium() -> Stroke {
        Stroke::new(1.5, Colors::BORDER)
    }
    
    /// Thick border (2px)
    pub fn thick() -> Stroke {
        Stroke::new(2.0, Colors::BORDER)
    }
    
    /// Hover state border
    pub fn hover() -> Stroke {
        Stroke::new(1.0, Colors::BORDER_HOVER)
    }
    
    /// Accent border
    pub fn accent() -> Stroke {
        Stroke::new(1.5, Colors::ACCENT)
    }
}

/// Elegant shadows for depth
pub struct Shadows;

impl Shadows {
    /// Subtle card shadow
    pub fn card() -> Shadow {
        Shadow {
            offset: egui::vec2(0.0, 1.0),
            blur: 4.0,
            spread: 0.0,
            color: Color32::from_black_alpha(16),
        }
    }
    
    /// Elevated panel shadow
    pub fn elevated() -> Shadow {
        Shadow {
            offset: egui::vec2(0.0, 4.0),
            blur: 16.0,
            spread: 0.0,
            color: Color32::from_black_alpha(40),
        }
    }
    
    /// Popup/modal shadow
    pub fn popup() -> Shadow {
        Shadow {
            offset: egui::vec2(0.0, 8.0),
            blur: 24.0,
            spread: 0.0,
            color: Color32::from_black_alpha(50),
        }
    }
}

/// Create modern professional visual style
pub fn create_monochrome_visuals() -> Visuals {
    Visuals {
        dark_mode: true,
        
        // No text color override
        override_text_color: None,
        
        // Deep charcoal background
        panel_fill: Colors::BACKGROUND,
        
        // Window styling with elegant shadows
        window_fill: Colors::SURFACE,
        window_stroke: Borders::thin(),
        window_rounding: Rounding::same(Radius::MD),
        window_shadow: Shadows::elevated(),
        
        // Popup styling with shadow
        popup_shadow: Shadows::popup(),
        
        // Modern widget styling
        widgets: egui::style::Widgets {
            noninteractive: egui::style::WidgetVisuals {
                bg_fill: Colors::INPUT_BG,
                weak_bg_fill: Colors::SURFACE,
                bg_stroke: Borders::thin(),
                rounding: Rounding::same(Radius::BASE),
                fg_stroke: Stroke::new(1.0, Colors::FOREGROUND),
                expansion: 0.0,
            },
            inactive: egui::style::WidgetVisuals {
                bg_fill: Colors::INPUT_BG,
                weak_bg_fill: Colors::SURFACE,
                bg_stroke: Borders::thin(),
                rounding: Rounding::same(Radius::BASE),
                fg_stroke: Stroke::new(1.0, Colors::FOREGROUND),
                expansion: 0.0,
            },
            hovered: egui::style::WidgetVisuals {
                bg_fill: Colors::SURFACE_ELEVATED,
                weak_bg_fill: Colors::SURFACE_ELEVATED,
                bg_stroke: Borders::hover(),
                rounding: Rounding::same(Radius::BASE),
                fg_stroke: Stroke::new(1.0, Colors::FOREGROUND),
                expansion: 1.0,
            },
            active: egui::style::WidgetVisuals {
                bg_fill: Colors::SURFACE_ELEVATED,
                weak_bg_fill: Colors::SURFACE_ELEVATED,
                bg_stroke: Borders::accent(),
                rounding: Rounding::same(Radius::BASE),
                fg_stroke: Stroke::new(1.0, Colors::ACCENT),
                expansion: 1.0,
            },
            open: egui::style::WidgetVisuals {
                bg_fill: Colors::SURFACE_ELEVATED,
                weak_bg_fill: Colors::SURFACE_ELEVATED,
                bg_stroke: Borders::accent(),
                rounding: Rounding::same(Radius::BASE),
                fg_stroke: Stroke::new(1.0, Colors::FOREGROUND),
                expansion: 0.0,
            },
        },
        
        // Elegant accent selection
        selection: egui::style::Selection {
            bg_fill: Color32::from_rgba_premultiplied(99, 102, 241, 40),
            stroke: Stroke::new(1.5, Colors::ACCENT),
        },
        
        // Hyperlinks in accent color
        hyperlink_color: Colors::ACCENT,
        
        // Subtle backgrounds
        faint_bg_color: Colors::SURFACE,
        extreme_bg_color: Colors::BACKGROUND,
        
        // Code highlighting
        code_bg_color: Colors::SURFACE_ELEVATED,
        
        // Status colors
        warn_fg_color: Colors::WARNING,
        error_fg_color: Colors::ERROR,
        
        // Window settings
        window_highlight_topmost: true,
        
        // Smooth rounded menus
        menu_rounding: Rounding::same(Radius::BASE),
        
        // Visual effects
        striped: false,
        slider_trailing_fill: true,
        handle_shape: egui::style::HandleShape::Circle,
        interact_cursor: Some(egui::CursorIcon::PointingHand),
        image_loading_spinners: true,
        numeric_color_space: egui::style::NumericColorSpace::GammaByte,
        
        ..Default::default()
    }
}

/// Configure professional app-wide styling
pub fn configure_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    
    // Compact spacing
    style.spacing.item_spacing = egui::vec2(Spacing::XXS, Spacing::XXS);
    style.spacing.window_margin = egui::Margin::same(Spacing::XS);
    style.spacing.button_padding = egui::vec2(InputSize::PADDING_X, InputSize::PADDING_Y);
    style.spacing.indent = Spacing::XS;
    style.spacing.slider_width = 140.0;
    style.spacing.combo_width = 140.0;
    style.spacing.combo_height = InputSize::HEIGHT;
    
    // Smooth interactions
    style.interaction.selectable_labels = true;
    style.interaction.show_tooltips_only_when_still = false;
    
    // Professional typography
    let mut text_styles = std::collections::BTreeMap::new();
    
    text_styles.insert(
        egui::TextStyle::Small,
        FontId::new(Typography::SM, FontFamily::Proportional),
    );
    
    text_styles.insert(
        egui::TextStyle::Body,
        FontId::new(Typography::BASE, FontFamily::Proportional),
    );
    
    text_styles.insert(
        egui::TextStyle::Button,
        FontId::new(Typography::SM, FontFamily::Proportional),
    );
    
    text_styles.insert(
        egui::TextStyle::Heading,
        FontId::new(Typography::XXL, FontFamily::Proportional),
    );
    
    text_styles.insert(
        egui::TextStyle::Monospace,
        FontId::new(Typography::BASE, FontFamily::Monospace),
    );
    
    style.text_styles = text_styles;
    
    // Apply modern visuals
    style.visuals = create_monochrome_visuals();
    
    // Enable animations
    style.animation_time = 0.15; // 150ms smooth transitions
    
    ctx.set_style(style);
}

/// Elegant horizontal divider
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

/// Vertical divider
pub fn vertical_rule(ui: &mut egui::Ui, thickness: f32, height: f32) {
    let rect = ui.available_rect_before_wrap();
    ui.painter().vline(
        rect.left(),
        egui::Rangef::new(rect.top(), rect.top() + height),
        Stroke::new(thickness, Colors::BORDER),
    );
}

/// Modern primary button with smooth hover
pub fn primary_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    let button = egui::Button::new(
        egui::RichText::new(text)
            .size(Typography::BASE)
            .color(Colors::ACCENT_FOREGROUND)
    )
    .fill(Colors::ACCENT)
    .stroke(Stroke::NONE)
    .rounding(Rounding::same(Radius::BASE));
    
    let response = ui.add(button);
    
    // Smooth hover effect
    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    
    response
}

/// Secondary button with elegant outline
pub fn secondary_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    let button = egui::Button::new(
        egui::RichText::new(text)
            .size(Typography::BASE)
            .color(Colors::FOREGROUND)
    )
    .fill(Color32::TRANSPARENT)
    .stroke(Borders::thin())
    .rounding(Rounding::same(Radius::BASE));
    
    let response = ui.add(button);
    
    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    
    response
}

/// Elegant card frame with shadow
pub fn card_frame(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::none()
        .fill(Colors::SURFACE)
        .stroke(Borders::thin())
    .inner_margin(Spacing::XS)
        .rounding(Rounding::same(Radius::MD))
    .shadow(Shadow::NONE)
        .show(ui, add_contents);
}

/// Accent card for highlights
pub fn inverted_card_frame(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::none()
        .fill(Colors::ACCENT)
        .stroke(Stroke::NONE)
        .inner_margin(Spacing::XS)
        .rounding(Rounding::same(Radius::MD))
        .shadow(Shadow::NONE)
        .show(ui, |ui| {
            ui.visuals_mut().override_text_color = Some(Colors::ACCENT_FOREGROUND);
            add_contents(ui);
        });
}
