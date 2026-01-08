//! Game settings and configuration

/// Game settings
#[derive(Clone)]
pub struct Settings {
    /// Movement speed multiplier
    pub move_speed: f32,
    /// Dash cooldown in seconds
    pub dash_cooldown: f32,
    /// Dash duration in seconds
    pub dash_duration: f32,
    /// Dash speed multiplier
    pub dash_speed_mult: f32,
    /// Camera smoothing factor (0-1, lower = smoother)
    pub camera_smoothing: f32,
    /// Number of bots in game
    pub bot_count: usize,
    /// Round duration in seconds
    pub round_duration: f32,
    /// Show FPS counter
    pub show_fps: bool,
    /// Theme index (0 = default, 1 = dark, 2 = neon)
    pub theme_index: usize,
    /// Screen shake intensity (0-1)
    pub screen_shake_intensity: f32,
    /// Particle density (0-1)
    pub particle_density: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            move_speed: 200.0,
            dash_cooldown: 3.0,
            dash_duration: 0.3,
            dash_speed_mult: 2.5,
            camera_smoothing: 0.1,
            bot_count: 5,
            round_duration: 120.0, // 2 minutes
            show_fps: false,
            theme_index: 0,
            screen_shake_intensity: 0.5,
            particle_density: 1.0,
        }
    }
}

impl Settings {
    /// Get effective movement speed based on hole size
    pub fn get_move_speed(&self, hole_radius: f32) -> f32 {
        // Larger holes move slightly slower
        let size_penalty = (hole_radius / 50.0).min(1.5);
        self.move_speed / (1.0 + size_penalty * 0.3)
    }
}
