//! Hole rendering - gradients, borders, effects

use macroquad::prelude::*;
use crate::gameplay::hole::Hole;
use crate::render::theme::Theme;

/// Draw all holes
pub fn draw_holes(
    holes: &[Hole],
    theme: &Theme,
    camera_x: f32,
    camera_y: f32,
    zoom: f32,
) {
    // Sort by size so larger holes are drawn behind
    let mut sorted: Vec<&Hole> = holes.iter().filter(|h| h.is_alive).collect();
    sorted.sort_by(|a, b| b.radius.partial_cmp(&a.radius).unwrap());

    for hole in sorted {
        draw_hole(hole, theme, camera_x, camera_y, zoom);
    }
}

/// Draw a single hole
pub fn draw_hole(
    hole: &Hole,
    theme: &Theme,
    camera_x: f32,
    camera_y: f32,
    zoom: f32,
) {
    if !hole.is_alive {
        return;
    }

    let x = (hole.x - camera_x) * zoom;
    let y = (hole.y - camera_y) * zoom;
    let r = hole.radius * zoom;

    // Invincibility effect
    let alpha = if hole.invincible > 0.0 {
        0.5 + (hole.invincible * 10.0).sin() * 0.3
    } else {
        1.0
    };

    // Draw hole layers (fake depth)
    draw_hole_depth(x, y, r, hole.color, alpha);
    
    // Draw border with potential pattern
    draw_hole_border(x, y, r, hole.color, hole.skin_pattern, alpha, hole.pulse_timer);

    // Draw name label
    draw_hole_label(x, y, r, &hole.name, hole.is_player, theme);
    
    // Draw dash indicator
    if hole.dash_active > 0.0 {
        draw_dash_trail(x, y, r, hole.color, hole.velocity);
    }
}

fn draw_hole_depth(x: f32, y: f32, r: f32, color: Color, alpha: f32) {
    // Outer dark ring (shadow)
    let shadow = Color::new(0.0, 0.0, 0.0, 0.8 * alpha);
    draw_circle(x, y, r * 1.02, shadow);

    // Main hole body - dark center
    let center_dark = Color::new(0.02, 0.02, 0.05, alpha);
    draw_circle(x, y, r, center_dark);

    // Inner gradient rings (fake radial gradient)
    let rings = 5;
    for i in (0..rings).rev() {
        let t = i as f32 / rings as f32;
        let ring_r = r * (0.3 + t * 0.6);
        let darkness = 0.05 + t * 0.1;
        let ring_color = Color::new(darkness, darkness, darkness + 0.02, alpha);
        draw_circle(x, y, ring_r, ring_color);
    }

    // Center abyss
    draw_circle(x, y, r * 0.2, Color::new(0.0, 0.0, 0.0, alpha));
}

fn draw_hole_border(x: f32, y: f32, r: f32, color: Color, pattern: u8, alpha: f32, pulse_timer: f32) {
    let border_width = (r * 0.08).max(3.0);
    
    // Pulse effect
    let pulse = 1.0 + (pulse_timer * 3.0).sin() * 0.02;
    let effective_r = r * pulse;
    
    // Main border
    let segments = 64;
    for i in 0..segments {
        let angle1 = (i as f32 / segments as f32) * std::f32::consts::TAU;
        let angle2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;
        
        let x1 = x + effective_r * angle1.cos();
        let y1 = y + effective_r * angle1.sin();
        let x2 = x + effective_r * angle2.cos();
        let y2 = y + effective_r * angle2.sin();
        
        // Pattern variation
        let pattern_alpha = match pattern {
            1 => {
                // Stripes
                if i % 4 < 2 { 1.0 } else { 0.5 }
            }
            2 => {
                // Wave
                0.7 + (angle1 * 8.0 + pulse_timer * 5.0).sin() * 0.3
            }
            3 => {
                // Dots (every other segment brighter)
                if i % 2 == 0 { 1.0 } else { 0.4 }
            }
            _ => 1.0, // Solid
        };
        
        let seg_color = Color::new(
            color.r * pattern_alpha,
            color.g * pattern_alpha,
            color.b * pattern_alpha,
            alpha,
        );
        
        draw_line(x1, y1, x2, y2, border_width, seg_color);
    }

    // Glow effect
    let glow_color = Color::new(color.r, color.g, color.b, 0.2 * alpha);
    draw_circle_lines(x, y, effective_r + border_width, border_width * 0.5, glow_color);
}

fn draw_hole_label(x: f32, y: f32, r: f32, name: &str, is_player: bool, theme: &Theme) {
    let font_size = theme.font_size_small;
    let text_dims = measure_text(name, None, font_size as u16, 1.0);
    
    let label_x = x - text_dims.width / 2.0;
    let label_y = y - r - 15.0;
    
    // Background
    let bg_padding = 4.0;
    let bg_color = if is_player {
        Color::new(0.2, 0.5, 1.0, 0.8)
    } else {
        Color::new(0.0, 0.0, 0.0, 0.6)
    };
    
    draw_rectangle(
        label_x - bg_padding,
        label_y - text_dims.height - bg_padding,
        text_dims.width + bg_padding * 2.0,
        text_dims.height + bg_padding * 2.0,
        bg_color,
    );
    
    draw_text(name, label_x, label_y, font_size, WHITE);
}

fn draw_dash_trail(x: f32, y: f32, r: f32, color: Color, velocity: Vec2) {
    if velocity.length() < 0.01 {
        return;
    }
    
    let dir = velocity.normalize();
    let trail_length = r * 0.8;
    
    // Draw fading circles behind
    for i in 1..=3 {
        let t = i as f32 * 0.3;
        let trail_x = x - dir.x * trail_length * t;
        let trail_y = y - dir.y * trail_length * t;
        let trail_r = r * (1.0 - t * 0.3);
        let trail_alpha = 0.3 * (1.0 - t);
        
        let trail_color = Color::new(color.r, color.g, color.b, trail_alpha);
        draw_circle(trail_x, trail_y, trail_r * 0.5, trail_color);
    }
}

/// Draw respawn indicator for dead hole
pub fn draw_respawn_indicator(
    hole: &Hole,
    theme: &Theme,
    camera_x: f32,
    camera_y: f32,
    zoom: f32,
) {
    if hole.is_alive || hole.respawn_timer <= 0.0 {
        return;
    }

    let x = (hole.x - camera_x) * zoom;
    let y = (hole.y - camera_y) * zoom;
    
    // Respawn bubble
    let bubble_r = 30.0 * zoom;
    let pulse = (hole.respawn_timer * 5.0).sin() * 0.2 + 0.8;
    
    let bubble_color = Color::new(hole.color.r, hole.color.g, hole.color.b, 0.3 * pulse);
    draw_circle(x, y, bubble_r * pulse, bubble_color);
    draw_circle_lines(x, y, bubble_r, 2.0, hole.color);
    
    // Timer text
    let timer_text = format!("{:.1}", hole.respawn_timer);
    let text_dims = measure_text(&timer_text, None, 20, 1.0);
    draw_text(&timer_text, x - text_dims.width / 2.0, y + 6.0, 20.0, WHITE);
}
