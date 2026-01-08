//! UI rendering - HUD, menus, overlays

use macroquad::prelude::*;
use crate::render::theme::{Theme, draw_rounded_rect, draw_rounded_rect_shadow, ease_out_back};
use crate::gameplay::scoring::LeaderboardEntry;
use crate::gameplay::modes::GameMode;

/// Draw the main menu
pub fn draw_menu(theme: &Theme, selection: usize, animation_time: f32) {
    let sw = screen_width();
    let sh = screen_height();

    draw_rectangle(0.0, 0.0, sw, sh, theme.palette.background);
    draw_grid_background(theme, animation_time);

    // Title
    let title = "HOLE.IO";
    let title_size = theme.font_size_title;
    let title_dims = measure_text(title, None, title_size as u16, 1.0);
    let title_x = sw / 2.0 - title_dims.width / 2.0;
    let title_y = sh * 0.25;
    draw_text(title, title_x + 4.0, title_y + 4.0, title_size, Color::new(0.0, 0.0, 0.0, 0.5));
    draw_text(title, title_x, title_y, title_size, theme.palette.ui_accent);

    let items = ["PLAY", "SETTINGS", "QUIT"];
    let item_height = 60.0;
    let start_y = sh * 0.45;

    for (i, item) in items.iter().enumerate() {
        let y = start_y + i as f32 * item_height;
        draw_menu_item(theme, item, sw / 2.0, y, i == selection, animation_time);
    }

    let hint = "Use ARROW KEYS to navigate, ENTER to select";
    let hint_dims = measure_text(hint, None, theme.font_size_small as u16, 1.0);
    draw_text(hint, sw / 2.0 - hint_dims.width / 2.0, sh - 40.0, theme.font_size_small, theme.palette.ui_text_secondary);
}

fn draw_grid_background(theme: &Theme, time: f32) {
    let sw = screen_width();
    let sh = screen_height();
    let grid_size = 50.0;
    let line_color = Color::new(1.0, 1.0, 1.0, 0.05);
    let offset_x = (time * 10.0) % grid_size;
    let offset_y = (time * 5.0) % grid_size;
    let mut x = -offset_x;
    while x < sw { draw_line(x, 0.0, x, sh, 1.0, line_color); x += grid_size; }
    let mut y = -offset_y;
    while y < sh { draw_line(0.0, y, sw, y, 1.0, line_color); y += grid_size; }
}

fn draw_menu_item(theme: &Theme, text: &str, x: f32, y: f32, selected: bool, time: f32) {
    let font_size = theme.font_size_large;
    let text_dims = measure_text(text, None, font_size as u16, 1.0);
    let bg_width = text_dims.width + 60.0;
    let bg_height = 50.0;

    if selected {
        let pulse = 1.0 + (time * 5.0).sin() * 0.02;
        let scale_w = bg_width * pulse;
        let scale_h = bg_height * pulse;
        draw_rounded_rect_shadow(x - scale_w / 2.0, y - scale_h / 2.0, scale_w, scale_h, theme.corner_radius, theme.palette.ui_accent, Color::new(0.0, 0.0, 0.0, 0.3), 4.0);
        draw_text(text, x - text_dims.width / 2.0, y + text_dims.height / 3.0, font_size, WHITE);
    } else {
        draw_rounded_rect(x - bg_width / 2.0, y - bg_height / 2.0, bg_width, bg_height, theme.corner_radius, theme.palette.ui_fg);
        draw_text(text, x - text_dims.width / 2.0, y + text_dims.height / 3.0, font_size, theme.palette.ui_text_secondary);
    }
}

/// Draw mode selection screen
pub fn draw_mode_select(theme: &Theme, selection: usize, animation_time: f32) {
    let sw = screen_width();
    let sh = screen_height();
    draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.7));

    let title = "SELECT MODE";
    let title_dims = measure_text(title, None, theme.font_size_large as u16, 1.0);
    draw_text(title, sw / 2.0 - title_dims.width / 2.0, sh * 0.2, theme.font_size_large, theme.palette.ui_text);

    let modes = [("CLASSIC", "2 min, biggest wins!"), ("BATTLE", "Last standing!"), ("SOLO", "100% city!")];
    let card_width = 200.0;
    let total_width = card_width * 3.0 + 40.0;
    let start_x = sw / 2.0 - total_width / 2.0;
    let card_y = sh * 0.4;

    for (i, (name, desc)) in modes.iter().enumerate() {
        let x = start_x + i as f32 * (card_width + 20.0);
        let is_selected = i == selection;
        let bg_color = if is_selected { theme.palette.ui_accent } else { theme.palette.ui_fg };
        draw_rounded_rect_shadow(x, card_y, card_width, 120.0, theme.corner_radius, bg_color, Color::new(0.0, 0.0, 0.0, 0.4), 6.0);
        let name_dims = measure_text(name, None, theme.font_size_medium as u16, 1.0);
        draw_text(name, x + card_width / 2.0 - name_dims.width / 2.0, card_y + 50.0, theme.font_size_medium, WHITE);
        let desc_dims = measure_text(desc, None, theme.font_size_small as u16, 1.0);
        draw_text(desc, x + card_width / 2.0 - desc_dims.width / 2.0, card_y + 90.0, theme.font_size_small, Color::new(1.0, 1.0, 1.0, 0.7));
    }

    let hint = "Press ESC to go back";
    let hint_dims = measure_text(hint, None, theme.font_size_small as u16, 1.0);
    draw_text(hint, sw / 2.0 - hint_dims.width / 2.0, sh - 40.0, theme.font_size_small, theme.palette.ui_text_secondary);
}

/// Draw the HUD during gameplay
pub fn draw_hud(theme: &Theme, timer: f32, leaderboard: &[LeaderboardEntry], player_rank: Option<usize>, player_size: f32, mode: GameMode, city_consumed: f32, dash_cooldown: f32, dash_cooldown_max: f32) {
    let sw = screen_width();
    let sh = screen_height();

    if mode.has_timer() { draw_timer(theme, sw / 2.0, 30.0, timer); }
    draw_leaderboard(theme, sw - 20.0, 20.0, leaderboard, player_rank);
    draw_player_stats(theme, 20.0, sh - 80.0, player_size, player_rank, mode, city_consumed);
    draw_dash_indicator(theme, sw / 2.0, sh - 40.0, dash_cooldown, dash_cooldown_max);
}

fn draw_timer(theme: &Theme, x: f32, y: f32, time_remaining: f32) {
    let mins = (time_remaining / 60.0).floor() as i32;
    let secs = (time_remaining % 60.0).floor() as i32;
    let timer_text = format!("{:02}:{:02}", mins, secs);
    let font_size = theme.font_size_large;
    let text_dims = measure_text(&timer_text, None, font_size as u16, 1.0);
    draw_rounded_rect(x - text_dims.width / 2.0 - 15.0, y - 5.0, text_dims.width + 30.0, text_dims.height + 20.0, theme.corner_radius, theme.palette.ui_bg);
    let color = if time_remaining < 30.0 { Color::new(1.0, 0.3, 0.3, 1.0) } else { theme.palette.ui_text };
    draw_text(&timer_text, x - text_dims.width / 2.0, y + text_dims.height, font_size, color);
}

fn draw_leaderboard(theme: &Theme, x: f32, y: f32, entries: &[LeaderboardEntry], _player_rank: Option<usize>) {
    let card_w = 200.0;
    let entry_h = 28.0;
    let visible = entries.len().min(5);
    let card_h = 30.0 + visible as f32 * entry_h + 20.0;
    draw_rounded_rect(x - card_w, y, card_w, card_h, theme.corner_radius, theme.palette.ui_bg);
    draw_text("LEADERBOARD", x - card_w + 10.0, y + 25.0, theme.font_size_small, theme.palette.ui_accent);

    for (i, entry) in entries.iter().take(5).enumerate() {
        let ey = y + 30.0 + i as f32 * entry_h + 20.0;
        if entry.is_player { draw_rectangle(x - card_w + 5.0, ey - entry_h + 8.0, card_w - 10.0, entry_h - 2.0, Color::new(0.3, 0.7, 1.0, 0.3)); }
        let tc = if entry.is_player { theme.palette.ui_accent } else { theme.palette.ui_text };
        draw_text(&format!("{}.", i + 1), x - card_w + 10.0, ey, theme.font_size_small, tc);
        let name: String = entry.name.chars().take(8).collect();
        draw_text(&name, x - card_w + 35.0, ey, theme.font_size_small, tc);
        draw_text(&format!("{:.0}", entry.size), x - 50.0, ey, theme.font_size_small, tc);
    }
}

fn draw_player_stats(theme: &Theme, x: f32, y: f32, size: f32, rank: Option<usize>, mode: GameMode, city_consumed: f32) {
    draw_rounded_rect(x, y, 180.0, 70.0, theme.corner_radius, theme.palette.ui_bg);
    draw_text(&format!("Size: {:.0}", size), x + 10.0, y + 25.0, theme.font_size_small, theme.palette.ui_text);
    match mode {
        GameMode::Solo => {
            draw_text(&format!("City: {:.1}%", city_consumed), x + 10.0, y + 50.0, theme.font_size_small, theme.palette.ui_accent);
        }
        _ => {
            if let Some(r) = rank {
                draw_text(&format!("Rank: #{}", r), x + 10.0, y + 50.0, theme.font_size_small, theme.palette.ui_accent);
            }
        }
    }
}

fn draw_dash_indicator(theme: &Theme, x: f32, y: f32, cooldown: f32, max_cd: f32) {
    let bar_w = 100.0;
    draw_rounded_rect(x - bar_w / 2.0, y, bar_w, 8.0, 4.0, theme.palette.ui_fg);
    let fill = if max_cd > 0.0 { 1.0 - (cooldown / max_cd) } else { 1.0 };
    let fill_color = if fill >= 1.0 { theme.palette.ui_accent } else { theme.palette.ui_text_secondary };
    if fill > 0.0 { draw_rounded_rect(x - bar_w / 2.0, y, bar_w * fill, 8.0, 4.0, fill_color); }
    let label = if fill >= 1.0 { "DASH READY" } else { "DASH" };
    let lbl_dims = measure_text(label, None, 12, 1.0);
    draw_text(label, x - lbl_dims.width / 2.0, y - 5.0, 12.0, theme.palette.ui_text_secondary);
}

/// Draw pause overlay
pub fn draw_pause_overlay(theme: &Theme, selection: usize, animation_time: f32) {
    let sw = screen_width();
    let sh = screen_height();
    draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.7));
    let card_w = 300.0;
    let card_h = 250.0;
    draw_rounded_rect_shadow(sw / 2.0 - card_w / 2.0, sh / 2.0 - card_h / 2.0, card_w, card_h, theme.corner_radius * 2.0, theme.palette.ui_bg, Color::new(0.0, 0.0, 0.0, 0.5), 8.0);
    let title_dims = measure_text("PAUSED", None, theme.font_size_large as u16, 1.0);
    draw_text("PAUSED", sw / 2.0 - title_dims.width / 2.0, sh / 2.0 - card_h / 2.0 + 50.0, theme.font_size_large, theme.palette.ui_accent);
    let options = ["RESUME", "RESTART", "EXIT"];
    for (i, opt) in options.iter().enumerate() {
        draw_menu_item(theme, opt, sw / 2.0, sh / 2.0 - card_h / 2.0 + 100.0 + i as f32 * 45.0, i == selection, animation_time);
    }
}

/// Draw results screen
pub fn draw_results(theme: &Theme, mode: GameMode, player_rank: usize, player_size: f32, total_players: usize, city_consumed: f32, selection: usize, animation_time: f32) {
    let sw = screen_width();
    let sh = screen_height();
    draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.8));
    let card_w = 400.0;
    let card_h = 350.0;
    let card_y = sh / 2.0 - card_h / 2.0;
    let entrance_t = (animation_time * 2.0).min(1.0);
    let animated_y = card_y + 50.0 * (1.0 - ease_out_back(entrance_t));
    draw_rounded_rect_shadow(sw / 2.0 - card_w / 2.0, animated_y, card_w, card_h, theme.corner_radius * 2.0, theme.palette.ui_bg, Color::new(0.0, 0.0, 0.0, 0.5), 8.0);

    let title = if mode == GameMode::Solo { if city_consumed >= 100.0 { "PERFECT!" } else { "GAME OVER" } } else { if player_rank == 1 { "VICTORY!" } else { "GAME OVER" } };
    let title_dims = measure_text(title, None, theme.font_size_large as u16, 1.0);
    draw_text(title, sw / 2.0 - title_dims.width / 2.0, animated_y + 50.0, theme.font_size_large, theme.palette.ui_accent);

    match mode {
        GameMode::Solo => {
            let txt = format!("City: {:.1}%", city_consumed);
            let dims = measure_text(&txt, None, theme.font_size_medium as u16, 1.0);
            draw_text(&txt, sw / 2.0 - dims.width / 2.0, animated_y + 100.0, theme.font_size_medium, theme.palette.ui_text);
        }
        _ => {
            let txt = format!("Rank: #{} / {}", player_rank, total_players);
            let dims = measure_text(&txt, None, theme.font_size_medium as u16, 1.0);
            draw_text(&txt, sw / 2.0 - dims.width / 2.0, animated_y + 100.0, theme.font_size_medium, theme.palette.ui_text);
            let stxt = format!("Size: {:.0}", player_size);
            let sdims = measure_text(&stxt, None, theme.font_size_medium as u16, 1.0);
            draw_text(&stxt, sw / 2.0 - sdims.width / 2.0, animated_y + 140.0, theme.font_size_medium, theme.palette.ui_text);
        }
    }

    let options = ["PLAY AGAIN", "CHANGE MODE", "MAIN MENU"];
    for (i, opt) in options.iter().enumerate() {
        draw_menu_item(theme, opt, sw / 2.0, animated_y + 200.0 + i as f32 * 45.0, i == selection, animation_time);
    }
}

/// Draw FPS counter
pub fn draw_fps(theme: &Theme) {
    draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, theme.font_size_small, theme.palette.ui_text_secondary);
}
