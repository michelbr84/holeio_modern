//! World objects - buildings, cars, trees, people, etc.

use macroquad::prelude::*;
use ::rand::prelude::*;

/// Types of objects in the world
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ObjectType {
    Building,
    Car,
    Tree,
    Person,
    Lamppost,
    Hydrant,
    TrashCan,
    Bench,
}

impl ObjectType {
    /// Get base size for this object type
    pub fn base_size(&self) -> f32 {
        match self {
            ObjectType::Building => 60.0,
            ObjectType::Car => 18.0,
            ObjectType::Tree => 12.0,
            ObjectType::Person => 5.0,
            ObjectType::Lamppost => 6.0,
            ObjectType::Hydrant => 4.0,
            ObjectType::TrashCan => 5.0,
            ObjectType::Bench => 8.0,
        }
    }

    /// Get color for this object type
    pub fn color(&self) -> Color {
        match self {
            ObjectType::Building => Color::new(0.45, 0.45, 0.55, 1.0),
            ObjectType::Car => Color::new(0.8, 0.2, 0.2, 1.0),
            ObjectType::Tree => Color::new(0.2, 0.6, 0.2, 1.0),
            ObjectType::Person => Color::new(0.9, 0.7, 0.5, 1.0),
            ObjectType::Lamppost => Color::new(0.3, 0.3, 0.3, 1.0),
            ObjectType::Hydrant => Color::new(0.9, 0.1, 0.1, 1.0),
            ObjectType::TrashCan => Color::new(0.3, 0.5, 0.3, 1.0),
            ObjectType::Bench => Color::new(0.5, 0.35, 0.2, 1.0),
        }
    }
}

/// Object state during capture
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ObjectState {
    /// Normal, on the ground
    Normal,
    /// Being captured (falling into hole)
    Falling {
        progress: f32,      // 0.0 to 1.0
        target_x: f32,
        target_y: f32,
        rotation: f32,
    },
    /// Already consumed
    Consumed,
}

/// A world object that can be swallowed
#[derive(Clone)]
pub struct WorldObject {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub size: f32,       // Effective size for swallowing (circumference equivalent)
    pub mass: f32,       // Value when consumed
    pub obj_type: ObjectType,
    pub state: ObjectState,
    pub consumed: bool,
    pub color: Color,
    pub rotation: f32,
}

static mut NEXT_ID: u32 = 0;

fn get_next_id() -> u32 {
    unsafe {
        let id = NEXT_ID;
        NEXT_ID += 1;
        id
    }
}

impl WorldObject {
    /// Create a new world object
    pub fn new(x: f32, y: f32, obj_type: ObjectType, rng: &mut impl Rng) -> Self {
        let base_size = obj_type.base_size();
        let size_variation = rng.gen_range(0.8..1.2);
        let size = base_size * size_variation;
        
        // Apply color variation
        let base_color = obj_type.color();
        let color_var = rng.gen_range(-0.1..0.1);
        let color = Color::new(
            (base_color.r + color_var).clamp(0.0, 1.0),
            (base_color.g + color_var).clamp(0.0, 1.0),
            (base_color.b + color_var).clamp(0.0, 1.0),
            1.0,
        );

        Self {
            id: get_next_id(),
            x, y,
            width: size,
            height: size,
            size,
            mass: size * size * 0.1, // Mass proportional to area
            obj_type,
            state: ObjectState::Normal,
            consumed: false,
            color,
            rotation: rng.gen::<f32>() * std::f32::consts::TAU,
        }
    }

    /// Create a building with specific dimensions
    pub fn new_building(x: f32, y: f32, width: f32, height: f32, rng: &mut impl Rng) -> Self {
        let size = (width + height) / 2.0;
        
        // Building colors with variation
        let gray = rng.gen_range(0.35..0.65);
        let color = Color::new(gray, gray, gray + 0.05, 1.0);

        Self {
            id: get_next_id(),
            x, y,
            width,
            height,
            size,
            mass: width * height * 0.5, // Buildings are heavy
            obj_type: ObjectType::Building,
            state: ObjectState::Normal,
            consumed: false,
            color,
            rotation: 0.0,
        }
    }

    /// Check if this object can be swallowed by a hole of given radius
    pub fn can_be_swallowed(&self, hole_radius: f32) -> bool {
        const K_FIT: f32 = 0.92;
        self.size <= hole_radius * K_FIT
    }

    /// Start falling animation toward the hole
    pub fn start_falling(&mut self, hole_x: f32, hole_y: f32) {
        self.state = ObjectState::Falling {
            progress: 0.0,
            target_x: hole_x,
            target_y: hole_y,
            rotation: 0.0,
        };
    }

    /// Update falling animation, returns true when complete
    pub fn update_falling(&mut self, dt: f32) -> bool {
        if let ObjectState::Falling { progress, target_x, target_y, rotation } = &mut self.state {
            let fall_speed = 3.0; // Complete in ~0.33 seconds
            *progress += dt * fall_speed;
            *rotation += dt * 15.0; // Spin while falling
            
            // Lerp position toward target
            let t = (*progress).min(1.0);
            let ease_t = t * t; // Ease-in for acceleration effect
            self.x = self.x + ((*target_x) - self.x) * ease_t * 0.3;
            self.y = self.y + ((*target_y) - self.y) * ease_t * 0.3;
            self.rotation = *rotation;

            if *progress >= 1.0 {
                self.state = ObjectState::Consumed;
                self.consumed = true;
                return true;
            }
        }
        false
    }

    /// Get visual scale based on state
    pub fn get_visual_scale(&self) -> f32 {
        match &self.state {
            ObjectState::Normal => 1.0,
            ObjectState::Falling { progress, .. } => {
                let t = (*progress).min(1.0);
                1.0 - t * 0.8 // Shrink to 20% while falling
            }
            ObjectState::Consumed => 0.0,
        }
    }

    /// Get visual alpha based on state
    pub fn get_visual_alpha(&self) -> f32 {
        match &self.state {
            ObjectState::Normal => 1.0,
            ObjectState::Falling { progress, .. } => {
                let t = (*progress).min(1.0);
                1.0 - t * 0.7 // Fade to 30% alpha
            }
            ObjectState::Consumed => 0.0,
        }
    }

    /// Get bounding rect for collision
    pub fn get_rect(&self) -> Rect {
        Rect::new(
            self.x - self.width / 2.0,
            self.y - self.height / 2.0,
            self.width,
            self.height,
        )
    }
}
