//! Bot AI - steering behaviors

use macroquad::prelude::*;
use ::rand::prelude::*;
use crate::gameplay::hole::Hole;
use crate::world::objects::WorldObject;
use crate::world::spatial::SpatialGrid;

/// Bot behavior state
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BotState {
    /// Looking for objects to consume
    Farming,
    /// Hunting smaller holes
    Hunting,
    /// Fleeing from larger holes
    Fleeing,
    /// Wandering randomly
    Wandering,
}

/// Bot controller
pub struct BotController {
    pub state: BotState,
    pub target: Option<Vec2>,
    pub state_timer: f32,
    pub wander_angle: f32,
    pub decision_cooldown: f32,
}

impl Default for BotController {
    fn default() -> Self {
        Self {
            state: BotState::Farming,
            target: None,
            state_timer: 0.0,
            wander_angle: 0.0,
            decision_cooldown: 0.0,
        }
    }
}

impl BotController {
    /// Update bot decision making
    pub fn update(
        &mut self,
        hole: &Hole,
        holes: &[Hole],
        objects: &[WorldObject],
        spatial: &SpatialGrid,
        dt: f32,
        rng: &mut impl Rng,
    ) -> Vec2 {
        self.state_timer += dt;
        self.decision_cooldown -= dt;

        // Make decisions periodically
        if self.decision_cooldown <= 0.0 {
            self.make_decision(hole, holes, objects, spatial, rng);
            self.decision_cooldown = 0.3 + rng.gen::<f32>() * 0.3; // 0.3-0.6s between decisions
        }

        // Execute current behavior
        match self.state {
            BotState::Farming => self.execute_farming(hole, objects, spatial, rng),
            BotState::Hunting => self.execute_hunting(hole, holes),
            BotState::Fleeing => self.execute_fleeing(hole, holes),
            BotState::Wandering => self.execute_wandering(hole, dt, rng),
        }
    }

    fn make_decision(
        &mut self,
        hole: &Hole,
        holes: &[Hole],
        objects: &[WorldObject],
        spatial: &SpatialGrid,
        rng: &mut impl Rng,
    ) {
        // Check for threats (larger holes nearby)
        let threat = self.find_threat(hole, holes);
        if let Some(threat_pos) = threat {
            self.state = BotState::Fleeing;
            self.target = Some(threat_pos);
            return;
        }

        // If large, hunt smaller holes
        if hole.radius > 50.0 {
            if let Some(prey) = self.find_prey(hole, holes) {
                if rng.gen::<f32>() < 0.6 { // 60% chance to hunt
                    self.state = BotState::Hunting;
                    self.target = Some(prey);
                    return;
                }
            }
        }

        // Otherwise, farm objects
        if let Some(target) = self.find_best_object(hole, objects, spatial) {
            self.state = BotState::Farming;
            self.target = Some(target);
        } else {
            self.state = BotState::Wandering;
            self.target = None;
        }
    }

    fn find_threat(&self, hole: &Hole, holes: &[Hole]) -> Option<Vec2> {
        const THREAT_RANGE: f32 = 200.0;
        const THREAT_MARGIN: f32 = 1.3; // Threat if 30% larger
        
        let mut closest_threat: Option<(f32, Vec2)> = None;
        
        for other in holes {
            if other.id == hole.id || !other.is_alive {
                continue;
            }
            
            let dx = other.x - hole.x;
            let dy = other.y - hole.y;
            let dist = (dx * dx + dy * dy).sqrt();
            
            if dist < THREAT_RANGE && other.radius > hole.radius * THREAT_MARGIN {
                if closest_threat.is_none() || dist < closest_threat.unwrap().0 {
                    closest_threat = Some((dist, vec2(other.x, other.y)));
                }
            }
        }
        
        closest_threat.map(|(_, pos)| pos)
    }

    fn find_prey(&self, hole: &Hole, holes: &[Hole]) -> Option<Vec2> {
        const HUNT_RANGE: f32 = 300.0;
        
        let mut best_prey: Option<(f32, Vec2)> = None;
        
        for other in holes {
            if other.id == hole.id || !other.is_alive {
                continue;
            }
            
            // Can consume if significantly larger
            if !hole.can_consume_hole(other) {
                continue;
            }
            
            let dx = other.x - hole.x;
            let dy = other.y - hole.y;
            let dist = (dx * dx + dy * dy).sqrt();
            
            if dist < HUNT_RANGE {
                // Prefer closer prey
                let score = dist;
                if best_prey.is_none() || score < best_prey.unwrap().0 {
                    best_prey = Some((score, vec2(other.x, other.y)));
                }
            }
        }
        
        best_prey.map(|(_, pos)| pos)
    }

    fn find_best_object(
        &self,
        hole: &Hole,
        objects: &[WorldObject],
        spatial: &SpatialGrid,
    ) -> Option<Vec2> {
        let nearby = spatial.query_radius(hole.x, hole.y, hole.radius * 4.0);
        
        let mut best: Option<(f32, Vec2)> = None;
        
        for idx in nearby {
            let obj = &objects[idx];
            
            if obj.consumed || !obj.can_be_swallowed(hole.radius) {
                continue;
            }
            
            let dx = obj.x - hole.x;
            let dy = obj.y - hole.y;
            let dist = (dx * dx + dy * dy).sqrt();
            
            // Score: prefer closer, larger objects
            let score = dist - obj.mass * 0.1;
            
            if best.is_none() || score < best.unwrap().0 {
                best = Some((score, vec2(obj.x, obj.y)));
            }
        }
        
        best.map(|(_, pos)| pos)
    }

    fn execute_farming(&self, hole: &Hole, objects: &[WorldObject], spatial: &SpatialGrid, rng: &mut impl Rng) -> Vec2 {
        if let Some(target) = self.target {
            let dir = target - vec2(hole.x, hole.y);
            if dir.length() > 1.0 {
                return dir.normalize();
            }
        }
        
        // No target, try to find one on the fly
        if let Some(new_target) = self.find_best_object(hole, objects, spatial) {
            let dir = new_target - vec2(hole.x, hole.y);
            if dir.length() > 1.0 {
                return dir.normalize();
            }
        }
        
        Vec2::ZERO
    }

    fn execute_hunting(&self, hole: &Hole, holes: &[Hole]) -> Vec2 {
        if let Some(target) = self.target {
            let dir = target - vec2(hole.x, hole.y);
            if dir.length() > 1.0 {
                return dir.normalize();
            }
        }
        Vec2::ZERO
    }

    fn execute_fleeing(&self, hole: &Hole, holes: &[Hole]) -> Vec2 {
        if let Some(threat_pos) = self.target {
            // Move away from threat
            let dir = vec2(hole.x, hole.y) - threat_pos;
            if dir.length() > 1.0 {
                return dir.normalize();
            }
        }
        Vec2::ZERO
    }

    fn execute_wandering(&mut self, hole: &Hole, dt: f32, rng: &mut impl Rng) -> Vec2 {
        // Slowly change wander direction
        self.wander_angle += (rng.gen::<f32>() - 0.5) * 2.0 * dt;
        
        vec2(self.wander_angle.cos(), self.wander_angle.sin())
    }
}

/// Bot names pool
pub const BOT_NAMES: [&str; 20] = [
    "Shadow", "Nova", "Blaze", "Storm", "Vortex",
    "Eclipse", "Thunder", "Frost", "Phoenix", "Titan",
    "Apex", "Nebula", "Raven", "Hunter", "Striker",
    "Ghost", "Raptor", "Omega", "Pulse", "Zero",
];

/// Bot colors pool
pub fn get_bot_color(index: usize) -> Color {
    let colors = [
        Color::new(1.0, 0.3, 0.3, 1.0),  // Red
        Color::new(0.3, 1.0, 0.3, 1.0),  // Green
        Color::new(1.0, 1.0, 0.3, 1.0),  // Yellow
        Color::new(1.0, 0.3, 1.0, 1.0),  // Magenta
        Color::new(0.3, 1.0, 1.0, 1.0),  // Cyan
        Color::new(1.0, 0.6, 0.2, 1.0),  // Orange
        Color::new(0.6, 0.3, 1.0, 1.0),  // Purple
        Color::new(0.3, 0.8, 0.5, 1.0),  // Teal
    ];
    colors[index % colors.len()]
}
