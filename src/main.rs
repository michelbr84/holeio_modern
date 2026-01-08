//! Hole.io Clone - Main entry point
//! A modern Hole.io clone with procedural graphics

use macroquad::prelude::*;
use ::rand::prelude::*;
use ::rand::rngs::StdRng;
use ::rand::SeedableRng;

mod app;
mod world;
mod gameplay;
mod render;
mod time;

use app::state::{AppState, GameState};
use app::settings::Settings;
use world::gen::World;
use world::spatial::SpatialGrid;
use gameplay::hole::Hole;
use gameplay::modes::{GameMode, ModeRules};
use gameplay::bots::{BotController, BOT_NAMES, get_bot_color};
use gameplay::scoring::Leaderboard;
use gameplay::swallow;
use render::theme::Theme;
use render::vfx::VfxSystem;
use time::clock::GameClock;

/// Camera state
struct Camera {
    x: f32,
    y: f32,
    zoom: f32,
    target_zoom: f32,
}

impl Camera {
    fn new() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0, target_zoom: 1.0 }
    }

    fn follow(&mut self, target_x: f32, target_y: f32, hole_radius: f32, _dt: f32, smoothing: f32) {
        let sw = screen_width();
        let sh = screen_height();
        
        // Target camera position (center on hole)
        let target_cx = target_x - sw / (2.0 * self.zoom);
        let target_cy = target_y - sh / (2.0 * self.zoom);
        
        // Smooth follow
        self.x += (target_cx - self.x) * smoothing.min(1.0);
        self.y += (target_cy - self.y) * smoothing.min(1.0);
        
        // Dynamic zoom based on hole size
        self.target_zoom = (50.0 / hole_radius).clamp(0.4, 1.2);
        self.zoom += (self.target_zoom - self.zoom) * 0.02;
    }
}

/// Complete game session
struct GameSession {
    world: World,
    spatial: SpatialGrid,
    holes: Vec<Hole>,
    bot_controllers: Vec<BotController>,
    player_idx: usize,
    clock: GameClock,
    leaderboard: Leaderboard,
    mode_rules: ModeRules,
    vfx: VfxSystem,
    camera: Camera,
    game_over: bool,
    results_time: f32,
}

impl GameSession {
    fn new(mode: GameMode, player_name: &str, seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let world = World::generate(seed);
        let mode_rules = ModeRules::new(mode);
        
        // Create player
        let player_pos = world.get_spawn_position(&mut rng);
        let player = Hole::new_player(player_pos.x, player_pos.y, player_name.to_string());
        
        let mut holes = vec![player];
        let mut bot_controllers = vec![BotController::default()]; // Placeholder for player
        
        // Create bots
        for i in 0..mode_rules.bot_count {
            let pos = world.get_spawn_position(&mut rng);
            let name = BOT_NAMES[i % BOT_NAMES.len()].to_string();
            let color = get_bot_color(i);
            holes.push(Hole::new_bot(pos.x, pos.y, name, color));
            bot_controllers.push(BotController::default());
        }
        
        let mut spatial = SpatialGrid::new();
        spatial.build(&world.objects);
        
        let clock = GameClock::new(mode.round_duration());
        
        Self {
            world,
            spatial,
            holes,
            bot_controllers,
            player_idx: 0,
            clock,
            leaderboard: Leaderboard::new(),
            mode_rules,
            vfx: VfxSystem::new(),
            camera: Camera::new(),
            game_over: false,
            results_time: 0.0,
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Hole.io".to_owned(),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app_state = AppState::default();
    let mut settings = Settings::default();
    let mut theme = Theme::default();
    let mut session: Option<GameSession> = None;
    let mut animation_time = 0.0f32;
    let mut rng = ::rand::thread_rng();

    loop {
        let dt = get_frame_time();
        animation_time += dt;

        match app_state.game_state {
            GameState::Menu => {
                handle_menu_input(&mut app_state);
                render::draw_ui::draw_menu(&theme, app_state.menu_selection, animation_time);
            }
            GameState::ModeSelect => {
                handle_mode_select_input(&mut app_state, &mut session, &settings, &mut rng);
                render::draw_ui::draw_mode_select(&theme, app_state.mode_selection, animation_time);
            }
            GameState::Playing => {
                if let Some(ref mut sess) = session {
                    update_game(sess, &mut app_state, &settings, dt, &mut rng);
                    render_game(sess, &theme, &settings);
                }
            }
            GameState::Pause => {
                if let Some(ref sess) = session {
                    render_game(sess, &theme, &settings);
                }
                render::draw_ui::draw_pause_overlay(&theme, app_state.pause_selection, animation_time);
                handle_pause_input(&mut app_state, &mut session, &settings, &mut rng);
            }
            GameState::Results => {
                if let Some(ref mut sess) = session {
                    sess.results_time += dt;
                    render_game(sess, &theme, &settings);
                    let pr = sess.leaderboard.get_player_rank().unwrap_or(sess.holes.len());
                    let ps = sess.holes[sess.player_idx].radius;
                    let cc = sess.world.get_consumption_percentage();
                    render::draw_ui::draw_results(&theme, sess.mode_rules.mode, pr, ps, sess.holes.len(), cc, app_state.results_selection, sess.results_time);
                }
                handle_results_input(&mut app_state, &mut session, &settings, &mut rng);
            }
        }

        if settings.show_fps {
            render::draw_ui::draw_fps(&theme);
        }

        next_frame().await
    }
}

fn handle_menu_input(app_state: &mut AppState) {
    if is_key_pressed(KeyCode::Up) { app_state.menu_selection = app_state.menu_selection.saturating_sub(1); }
    if is_key_pressed(KeyCode::Down) { app_state.menu_selection = (app_state.menu_selection + 1).min(2); }
    if is_key_pressed(KeyCode::Enter) {
        match app_state.menu_selection {
            0 => app_state.transition_to(GameState::ModeSelect),
            1 => {} // Settings TODO
            2 => std::process::exit(0),
            _ => {}
        }
    }
}

fn handle_mode_select_input(app_state: &mut AppState, session: &mut Option<GameSession>, _settings: &Settings, rng: &mut impl Rng) {
    if is_key_pressed(KeyCode::Left) { app_state.mode_selection = app_state.mode_selection.saturating_sub(1); }
    if is_key_pressed(KeyCode::Right) { app_state.mode_selection = (app_state.mode_selection + 1).min(2); }
    if is_key_pressed(KeyCode::Escape) { app_state.transition_to(GameState::Menu); }
    if is_key_pressed(KeyCode::Enter) {
        let mode = match app_state.mode_selection {
            0 => GameMode::Classic,
            1 => GameMode::Battle,
            _ => GameMode::Solo,
        };
        *session = Some(GameSession::new(mode, &app_state.player_name, rng.gen()));
        if let Some(ref mut s) = session { s.clock.start(); }
        app_state.start_game(mode);
    }
}

fn handle_pause_input(app_state: &mut AppState, session: &mut Option<GameSession>, _settings: &Settings, rng: &mut impl Rng) {
    if is_key_pressed(KeyCode::Up) { app_state.pause_selection = app_state.pause_selection.saturating_sub(1); }
    if is_key_pressed(KeyCode::Down) { app_state.pause_selection = (app_state.pause_selection + 1).min(2); }
    if is_key_pressed(KeyCode::Escape) {
        if let Some(ref mut s) = session { s.clock.resume(); }
        app_state.transition_to(GameState::Playing);
    }
    if is_key_pressed(KeyCode::Enter) {
        match app_state.pause_selection {
            0 => { if let Some(ref mut s) = session { s.clock.resume(); } app_state.transition_to(GameState::Playing); }
            1 => { *session = Some(GameSession::new(app_state.selected_mode, &app_state.player_name, rng.gen())); if let Some(ref mut s) = session { s.clock.start(); } app_state.transition_to(GameState::Playing); }
            2 => { *session = None; app_state.transition_to(GameState::Menu); }
            _ => {}
        }
    }
}

fn handle_results_input(app_state: &mut AppState, session: &mut Option<GameSession>, _settings: &Settings, rng: &mut impl Rng) {
    if is_key_pressed(KeyCode::Up) { app_state.results_selection = app_state.results_selection.saturating_sub(1); }
    if is_key_pressed(KeyCode::Down) { app_state.results_selection = (app_state.results_selection + 1).min(2); }
    if is_key_pressed(KeyCode::Enter) {
        match app_state.results_selection {
            0 => { *session = Some(GameSession::new(app_state.selected_mode, &app_state.player_name, rng.gen())); if let Some(ref mut s) = session { s.clock.start(); } app_state.transition_to(GameState::Playing); }
            1 => { *session = None; app_state.transition_to(GameState::ModeSelect); }
            2 => { *session = None; app_state.transition_to(GameState::Menu); }
            _ => {}
        }
    }
}

fn update_game(sess: &mut GameSession, app_state: &mut AppState, settings: &Settings, dt: f32, rng: &mut impl Rng) {
    if sess.game_over { return; }
    
    // Pause check
    if is_key_pressed(KeyCode::Escape) {
        sess.clock.pause();
        app_state.transition_to(GameState::Pause);
        return;
    }

    // Update clock
    let time_up = sess.clock.update(dt);
    
    // Player input
    let player = &mut sess.holes[sess.player_idx];
    if player.is_alive {
        let mut vel = Vec2::ZERO;
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) { vel.y -= 1.0; }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) { vel.y += 1.0; }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) { vel.x -= 1.0; }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) { vel.x += 1.0; }
        player.set_velocity(vel);
        
        if is_key_pressed(KeyCode::LeftShift) || is_key_pressed(KeyCode::RightShift) {
            player.try_dash(settings.dash_cooldown, settings.dash_duration);
        }
    }

    // Update bot AI
    for i in 1..sess.holes.len() {
        let hole = sess.holes[i].clone();
        if hole.is_alive {
            let vel = sess.bot_controllers[i].update(&hole, &sess.holes, &sess.world.objects, &sess.spatial, dt, rng);
            sess.holes[i].set_velocity(vel);
        }
    }

    // Update all holes
    for hole in &mut sess.holes {
        hole.update(dt, sess.world.width, sess.world.height, settings.move_speed);
    }

    // Rebuild spatial grid
    sess.spatial.build(&sess.world.objects);

    // Process swallowing for each hole
    for i in 0..sess.holes.len() {
        let hole = &mut sess.holes[i];
        if hole.is_alive {
            swallow::process_swallow(hole, &mut sess.world.objects, &sess.spatial, &mut sess.vfx);
        }
    }

    // Update falling objects
    for i in 0..sess.holes.len() {
        let hole = &mut sess.holes[i];
        swallow::update_falling_objects(hole, &mut sess.world.objects, dt);
    }

    // Hole vs hole combat
    swallow::process_hole_combat(&mut sess.holes, sess.player_idx, &mut sess.vfx, sess.mode_rules.mode.allows_respawn(), sess.mode_rules.respawn_time);

    // Respawn dead holes at new positions
    for hole in &mut sess.holes {
        if !hole.is_alive && hole.respawn_timer <= 0.0 && sess.mode_rules.mode.allows_respawn() {
            let pos = sess.world.get_spawn_position(rng);
            hole.respawn(pos.x, pos.y);
        }
    }

    // Update VFX
    sess.vfx.update(dt);

    // Update camera
    let player = &sess.holes[sess.player_idx];
    sess.camera.follow(player.x, player.y, player.radius, dt, settings.camera_smoothing);

    // Update leaderboard
    sess.leaderboard.update(&sess.holes);

    // Check victory conditions
    let alive_count = sess.holes.iter().filter(|h| h.is_alive).count();
    let player_alive = sess.holes[sess.player_idx].is_alive;
    let city_consumed = sess.world.get_consumption_percentage();

    if time_up || (sess.mode_rules.mode == GameMode::Battle && alive_count <= 1) || (sess.mode_rules.mode == GameMode::Solo && city_consumed >= 100.0) {
        sess.game_over = true;
        app_state.transition_to(GameState::Results);
    }

    if !player_alive && !sess.mode_rules.mode.allows_respawn() {
        sess.game_over = true;
        app_state.transition_to(GameState::Results);
    }
}

fn render_game(sess: &GameSession, theme: &Theme, settings: &Settings) {
    clear_background(theme.palette.background);

    let (shake_x, shake_y) = if settings.screen_shake_intensity > 0.0 {
        let mut vfx_copy = sess.vfx.clone();
        vfx_copy.get_shake_offset()
    } else { (0.0, 0.0) };

    let cam_x = sess.camera.x + shake_x;
    let cam_y = sess.camera.y + shake_y;
    let zoom = sess.camera.zoom;

    // Draw world
    render::draw_world::draw_world(&sess.world, theme, cam_x, cam_y, zoom);
    render::draw_world::draw_world_bounds(&sess.world, theme, cam_x, cam_y, zoom);

    // Draw VFX (behind holes)
    sess.vfx.draw(cam_x, cam_y, zoom);

    // Draw holes
    render::draw_holes::draw_holes(&sess.holes, theme, cam_x, cam_y, zoom);

    // Draw respawn indicators
    for hole in &sess.holes {
        render::draw_holes::draw_respawn_indicator(hole, theme, cam_x, cam_y, zoom);
    }

    // Draw HUD
    let player = &sess.holes[sess.player_idx];
    render::draw_ui::draw_hud(
        theme,
        sess.clock.remaining,
        sess.leaderboard.top(5),
        sess.leaderboard.get_player_rank(),
        player.radius,
        sess.mode_rules.mode,
        sess.world.get_consumption_percentage(),
        player.dash_cooldown,
        settings.dash_cooldown,
    );
}
