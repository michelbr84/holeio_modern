//! Hole entity - the player and bots

use macroquad::prelude::*;

/// Hole entity (player or bot)
#[derive(Clone)]
pub struct Hole {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub velocity: Vec2,
    pub name: String,
    pub color: Color,
    pub is_player: bool,
    pub is_alive: bool,
    
    // Growth
    pub area: f32,           // Current area
    pub score: i32,          // Objects consumed
    pub eliminations: i32,   // Holes consumed
    
    // Dash
    pub dash_cooldown: f32,  // Remaining cooldown
    pub dash_active: f32,    // Remaining dash time
    
    // Respawn
    pub respawn_timer: f32,  // Time until respawn
    pub invincible: f32,     // Invincibility after respawn
    
    // Visual
    pub skin_pattern: u8,    // 0-3 for different patterns
    pub border_style: u8,    // 0-2 for different borders
    pub pulse_timer: f32,    // For pulsing animation
}

static mut HOLE_NEXT_ID: u32 = 0;

fn get_hole_id() -> u32 {
    unsafe {
        let id = HOLE_NEXT_ID;
        HOLE_NEXT_ID += 1;
        id
    }
}

impl Hole {
    /// Starting radius
    pub const INITIAL_RADIUS: f32 = 25.0;
    /// Maximum radius before capping growth
    pub const MAX_RADIUS: f32 = 200.0;
    
    /// Create a new hole
    pub fn new(x: f32, y: f32, name: String, color: Color, is_player: bool) -> Self {
        let area = std::f32::consts::PI * Self::INITIAL_RADIUS * Self::INITIAL_RADIUS;
        Self {
            id: get_hole_id(),
            x, y,
            radius: Self::INITIAL_RADIUS,
            velocity: Vec2::ZERO,
            name,
            color,
            is_player,
            is_alive: true,
            area,
            score: 0,
            eliminations: 0,
            dash_cooldown: 0.0,
            dash_active: 0.0,
            respawn_timer: 0.0,
            invincible: 0.0,
            skin_pattern: 0,
            border_style: 0,
            pulse_timer: 0.0,
        }
    }

    /// Create player hole
    pub fn new_player(x: f32, y: f32, name: String) -> Self {
        Self::new(x, y, name, Color::new(0.2, 0.6, 1.0, 1.0), true)
    }

    /// Create bot hole
    pub fn new_bot(x: f32, y: f32, name: String, color: Color) -> Self {
        Self::new(x, y, name, color, false)
    }

    /// Update hole state
    pub fn update(&mut self, dt: f32, world_width: f32, world_height: f32, move_speed: f32) {
        // Update timers
        if self.dash_cooldown > 0.0 {
            self.dash_cooldown -= dt;
        }
        if self.dash_active > 0.0 {
            self.dash_active -= dt;
        }
        if self.invincible > 0.0 {
            self.invincible -= dt;
        }
        if self.respawn_timer > 0.0 {
            self.respawn_timer -= dt;
            if self.respawn_timer <= 0.0 {
                self.is_alive = true;
                self.invincible = 1.0; // 1 second invincibility
            }
            return;
        }
        
        self.pulse_timer += dt;

        // Apply velocity with speed adjustment for size
        let size_penalty = (self.radius / 50.0).min(1.5);
        let effective_speed = move_speed / (1.0 + size_penalty * 0.3);
        let dash_mult = if self.dash_active > 0.0 { 2.5 } else { 1.0 };
        
        self.x += self.velocity.x * effective_speed * dash_mult * dt;
        self.y += self.velocity.y * effective_speed * dash_mult * dt;

        // Clamp to world bounds
        self.x = self.x.clamp(self.radius, world_width - self.radius);
        self.y = self.y.clamp(self.radius, world_height - self.radius);
    }

    /// Set movement direction (normalized)
    pub fn set_velocity(&mut self, vel: Vec2) {
        if vel.length() > 0.01 {
            self.velocity = vel.normalize();
        } else {
            self.velocity = Vec2::ZERO;
        }
    }

    /// Attempt to dash
    pub fn try_dash(&mut self, dash_cooldown: f32, dash_duration: f32) -> bool {
        if self.dash_cooldown <= 0.0 && self.velocity.length() > 0.01 {
            self.dash_cooldown = dash_cooldown;
            self.dash_active = dash_duration;
            return true;
        }
        false
    }

    /// Grow by consuming an object
    pub fn grow(&mut self, mass: f32, growth_multiplier: f32) {
        self.area += mass * growth_multiplier;
        self.radius = (self.area / std::f32::consts::PI).sqrt();
        self.radius = self.radius.min(Self::MAX_RADIUS);
        self.score += 1;
    }

    /// Consume another hole
    pub fn consume_hole(&mut self, other: &Hole) {
        self.area += other.area * 0.5; // Get half the area
        self.radius = (self.area / std::f32::consts::PI).sqrt();
        self.radius = self.radius.min(Self::MAX_RADIUS);
        self.eliminations += 1;
    }

    /// Check if can consume another hole
    pub fn can_consume_hole(&self, other: &Hole) -> bool {
        const MARGIN: f32 = 1.2; // Need to be 20% larger
        if !self.is_alive || !other.is_alive {
            return false;
        }
        if other.invincible > 0.0 {
            return false;
        }
        self.radius > other.radius * MARGIN
    }

    /// Check if overlapping with another hole
    pub fn overlaps_hole(&self, other: &Hole) -> bool {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dist = (dx * dx + dy * dy).sqrt();
        dist < (self.radius + other.radius) * 0.6 // Need to be fairly close
    }

    /// Kill this hole (prepare for respawn)
    pub fn die(&mut self, respawn_time: f32) {
        self.is_alive = false;
        self.respawn_timer = respawn_time;
        // Reset to initial size
        self.area = std::f32::consts::PI * Self::INITIAL_RADIUS * Self::INITIAL_RADIUS;
        self.radius = Self::INITIAL_RADIUS;
    }

    /// Respawn at a new position
    pub fn respawn(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
        self.is_alive = true;
        self.invincible = 1.0;
    }

    /// Get position as Vec2
    pub fn position(&self) -> Vec2 {
        vec2(self.x, self.y)
    }

    /// Check capture condition for an object
    pub fn can_capture_at(&self, obj_x: f32, obj_y: f32, obj_size: f32) -> bool {
        const K_FIT: f32 = 0.92;
        const K_CAPTURE: f32 = 1.05;
        
        // Check if object fits
        if obj_size > self.radius * K_FIT {
            return false;
        }
        
        // Check if center is in capture zone
        let dx = obj_x - self.x;
        let dy = obj_y - self.y;
        let dist = (dx * dx + dy * dy).sqrt();
        dist <= self.radius * K_CAPTURE
    }
}
