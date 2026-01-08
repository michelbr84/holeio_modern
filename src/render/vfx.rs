//! Visual effects - particles, ripples, trails, screen shake

use macroquad::prelude::*;
use ::rand::prelude::*;
use ::rand::rngs::StdRng;
use ::rand::SeedableRng;

/// VFX event types
pub enum VfxType {
    SwallowParticles { x: f32, y: f32, color: Color, count: usize },
    Ripple { x: f32, y: f32, radius: f32, color: Color },
    Trail { x: f32, y: f32, color: Color },
}

/// Single particle
#[derive(Clone)]
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    size: f32,
    color: Color,
    lifetime: f32,
    max_lifetime: f32,
}

/// Single ripple effect
#[derive(Clone)]
struct Ripple {
    x: f32,
    y: f32,
    start_radius: f32,
    current_radius: f32,
    max_radius: f32,
    color: Color,
    lifetime: f32,
    max_lifetime: f32,
}

/// VFX system managing all effects
#[derive(Clone)]
pub struct VfxSystem {
    particles: Vec<Particle>,
    ripples: Vec<Ripple>,
    screen_shake: f32,
    shake_intensity: f32,
    rng: StdRng,
}

impl Default for VfxSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl VfxSystem {
    pub fn new() -> Self {
        Self {
            particles: Vec::with_capacity(500),
            ripples: Vec::with_capacity(20),
            screen_shake: 0.0,
            shake_intensity: 0.5,
            rng: StdRng::from_entropy(),
        }
    }

    /// Spawn a VFX event
    pub fn spawn(&mut self, vfx: VfxType) {
        match vfx {
            VfxType::SwallowParticles { x, y, color, count } => {
                for _ in 0..count.min(30) {
                    let angle = self.rng.gen::<f32>() * std::f32::consts::TAU;
                    let speed = self.rng.gen_range(50.0..150.0);
                    self.particles.push(Particle {
                        x, y,
                        vx: angle.cos() * speed,
                        vy: angle.sin() * speed,
                        size: self.rng.gen_range(2.0..6.0),
                        color,
                        lifetime: self.rng.gen_range(0.3..0.6),
                        max_lifetime: 0.5,
                    });
                }
                // Add screen shake for bigger swallows
                if count > 10 {
                    self.screen_shake = 0.1;
                }
            }
            VfxType::Ripple { x, y, radius, color } => {
                self.ripples.push(Ripple {
                    x, y,
                    start_radius: radius * 0.8,
                    current_radius: radius * 0.8,
                    max_radius: radius * 1.5,
                    color: Color::new(color.r, color.g, color.b, 0.5),
                    lifetime: 0.4,
                    max_lifetime: 0.4,
                });
            }
            VfxType::Trail { x, y, color } => {
                self.particles.push(Particle {
                    x, y,
                    vx: 0.0,
                    vy: 0.0,
                    size: 4.0,
                    color: Color::new(color.r, color.g, color.b, 0.3),
                    lifetime: 0.2,
                    max_lifetime: 0.2,
                });
            }
        }
    }

    /// Add screen shake
    pub fn add_shake(&mut self, amount: f32) {
        self.screen_shake = (self.screen_shake + amount).min(0.3);
    }

    /// Update all effects
    pub fn update(&mut self, dt: f32) {
        // Update particles
        self.particles.retain_mut(|p| {
            p.lifetime -= dt;
            if p.lifetime <= 0.0 { return false; }
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.vx *= 0.95; // Friction
            p.vy *= 0.95;
            true
        });

        // Update ripples
        self.ripples.retain_mut(|r| {
            r.lifetime -= dt;
            if r.lifetime <= 0.0 { return false; }
            let t = 1.0 - r.lifetime / r.max_lifetime;
            r.current_radius = r.start_radius + (r.max_radius - r.start_radius) * t;
            true
        });

        // Decay screen shake
        if self.screen_shake > 0.0 {
            self.screen_shake -= dt * 2.0;
            if self.screen_shake < 0.0 { self.screen_shake = 0.0; }
        }
    }

    /// Render all effects
    pub fn draw(&self, camera_x: f32, camera_y: f32, zoom: f32) {
        // Draw ripples first (behind particles)
        for r in &self.ripples {
            let x = (r.x - camera_x) * zoom;
            let y = (r.y - camera_y) * zoom;
            let radius = r.current_radius * zoom;
            let alpha = r.lifetime / r.max_lifetime * r.color.a;
            let color = Color::new(r.color.r, r.color.g, r.color.b, alpha);
            draw_circle_lines(x, y, radius, 2.0, color);
        }

        // Draw particles
        for p in &self.particles {
            let x = (p.x - camera_x) * zoom;
            let y = (p.y - camera_y) * zoom;
            let size = p.size * zoom;
            let alpha = p.lifetime / p.max_lifetime * p.color.a;
            let color = Color::new(p.color.r, p.color.g, p.color.b, alpha);
            draw_circle(x, y, size, color);
        }
    }

    /// Get screen shake offset
    pub fn get_shake_offset(&mut self) -> (f32, f32) {
        if self.screen_shake > 0.0 {
            let intensity = self.screen_shake * self.shake_intensity * 10.0;
            let x = (self.rng.gen::<f32>() - 0.5) * intensity;
            let y = (self.rng.gen::<f32>() - 0.5) * intensity;
            (x, y)
        } else {
            (0.0, 0.0)
        }
    }

    /// Clear all effects
    pub fn clear(&mut self) {
        self.particles.clear();
        self.ripples.clear();
        self.screen_shake = 0.0;
    }
}
