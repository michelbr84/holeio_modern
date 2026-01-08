//! Theme system - colors, styles, palettes

use macroquad::prelude::*;

/// Color palette
#[derive(Clone)]
pub struct Palette {
    pub background: Color,
    pub street: Color,
    pub street_line: Color,
    pub grass: Color,
    pub building_base: Color,
    pub shadow: Color,
    pub highlight: Color,
    pub ui_bg: Color,
    pub ui_fg: Color,
    pub ui_accent: Color,
    pub ui_text: Color,
    pub ui_text_secondary: Color,
}

impl Palette {
    /// Default city theme
    pub fn city() -> Self {
        Self {
            background: Color::new(0.15, 0.18, 0.22, 1.0),
            street: Color::new(0.25, 0.28, 0.32, 1.0),
            street_line: Color::new(0.9, 0.9, 0.3, 1.0),
            grass: Color::new(0.2, 0.5, 0.25, 1.0),
            building_base: Color::new(0.45, 0.48, 0.55, 1.0),
            shadow: Color::new(0.0, 0.0, 0.0, 0.3),
            highlight: Color::new(1.0, 1.0, 1.0, 0.15),
            ui_bg: Color::new(0.1, 0.12, 0.15, 0.95),
            ui_fg: Color::new(0.18, 0.2, 0.25, 1.0),
            ui_accent: Color::new(0.3, 0.7, 1.0, 1.0),
            ui_text: Color::new(1.0, 1.0, 1.0, 1.0),
            ui_text_secondary: Color::new(0.7, 0.7, 0.7, 1.0),
        }
    }

    /// Dark neon theme
    pub fn neon() -> Self {
        Self {
            background: Color::new(0.05, 0.05, 0.1, 1.0),
            street: Color::new(0.1, 0.1, 0.15, 1.0),
            street_line: Color::new(0.0, 1.0, 0.8, 1.0),
            grass: Color::new(0.0, 0.3, 0.2, 1.0),
            building_base: Color::new(0.15, 0.1, 0.25, 1.0),
            shadow: Color::new(0.0, 0.0, 0.0, 0.5),
            highlight: Color::new(1.0, 0.0, 1.0, 0.2),
            ui_bg: Color::new(0.05, 0.05, 0.1, 0.95),
            ui_fg: Color::new(0.1, 0.1, 0.2, 1.0),
            ui_accent: Color::new(1.0, 0.0, 0.8, 1.0),
            ui_text: Color::new(1.0, 1.0, 1.0, 1.0),
            ui_text_secondary: Color::new(0.6, 0.6, 0.8, 1.0),
        }
    }

    /// Sunset theme
    pub fn sunset() -> Self {
        Self {
            background: Color::new(0.2, 0.1, 0.15, 1.0),
            street: Color::new(0.3, 0.2, 0.25, 1.0),
            street_line: Color::new(1.0, 0.8, 0.3, 1.0),
            grass: Color::new(0.3, 0.4, 0.2, 1.0),
            building_base: Color::new(0.4, 0.3, 0.35, 1.0),
            shadow: Color::new(0.0, 0.0, 0.0, 0.4),
            highlight: Color::new(1.0, 0.8, 0.5, 0.2),
            ui_bg: Color::new(0.15, 0.08, 0.1, 0.95),
            ui_fg: Color::new(0.25, 0.15, 0.2, 1.0),
            ui_accent: Color::new(1.0, 0.5, 0.3, 1.0),
            ui_text: Color::new(1.0, 0.95, 0.9, 1.0),
            ui_text_secondary: Color::new(0.8, 0.7, 0.6, 1.0),
        }
    }
}

/// Current theme
pub struct Theme {
    pub palette: Palette,
    pub font_size_small: f32,
    pub font_size_medium: f32,
    pub font_size_large: f32,
    pub font_size_title: f32,
    pub corner_radius: f32,
    pub shadow_offset: f32,
    pub animation_speed: f32,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            palette: Palette::city(),
            font_size_small: 16.0,
            font_size_medium: 24.0,
            font_size_large: 36.0,
            font_size_title: 64.0,
            corner_radius: 8.0,
            shadow_offset: 4.0,
            animation_speed: 1.0,
        }
    }
}

impl Theme {
    pub fn set_palette_index(&mut self, index: usize) {
        self.palette = match index {
            0 => Palette::city(),
            1 => Palette::neon(),
            2 => Palette::sunset(),
            _ => Palette::city(),
        };
    }
}

/// Draw a rounded rectangle
pub fn draw_rounded_rect(x: f32, y: f32, w: f32, h: f32, radius: f32, color: Color) {
    // Simple approximation: draw main rect + circles at corners
    let r = radius.min(w / 2.0).min(h / 2.0);
    
    // Main rectangle (clamped to safe area)
    draw_rectangle(x + r, y, w - r * 2.0, h, color);
    draw_rectangle(x, y + r, w, h - r * 2.0, color);
    
    // Corner circles
    draw_circle(x + r, y + r, r, color);
    draw_circle(x + w - r, y + r, r, color);
    draw_circle(x + r, y + h - r, r, color);
    draw_circle(x + w - r, y + h - r, r, color);
}

/// Draw a rounded rectangle with shadow
pub fn draw_rounded_rect_shadow(
    x: f32, y: f32, w: f32, h: f32, 
    radius: f32, color: Color, shadow_color: Color, shadow_offset: f32
) {
    draw_rounded_rect(x + shadow_offset, y + shadow_offset, w, h, radius, shadow_color);
    draw_rounded_rect(x, y, w, h, radius, color);
}

/// Lerp between two colors
pub fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    Color::new(
        a.r + (b.r - a.r) * t,
        a.g + (b.g - a.g) * t,
        a.b + (b.b - a.b) * t,
        a.a + (b.a - a.a) * t,
    )
}

/// Ease functions
pub fn ease_in_out(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        -1.0 + (4.0 - 2.0 * t) * t
    }
}

pub fn ease_out_back(t: f32) -> f32 {
    let c1 = 1.70158;
    let c3 = c1 + 1.0;
    1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
}
