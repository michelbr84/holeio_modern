//! Procedural city generation

use macroquad::prelude::*;
use ::rand::prelude::*;
use ::rand::rngs::StdRng;
use ::rand::SeedableRng;
use crate::world::objects::{WorldObject, ObjectType};

/// World configuration
pub const WORLD_WIDTH: f32 = 2000.0;
pub const WORLD_HEIGHT: f32 = 2000.0;
pub const STREET_WIDTH: f32 = 40.0;
pub const BLOCK_SIZE: f32 = 200.0;
pub const AVENUE_INTERVAL: usize = 3; // Every 3rd street is an avenue (wider)

/// Street segment
#[derive(Clone)]
pub struct Street {
    pub rect: Rect,
    pub is_avenue: bool,
}

/// City block (area between streets)
#[derive(Clone)]
pub struct Block {
    pub rect: Rect,
    pub is_park: bool,
}

/// Complete generated world
pub struct World {
    pub streets: Vec<Street>,
    pub blocks: Vec<Block>,
    pub objects: Vec<WorldObject>,
    pub width: f32,
    pub height: f32,
}

impl World {
    /// Generate a new procedural city
    pub fn generate(seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut streets = Vec::new();
        let mut blocks = Vec::new();
        let mut objects = Vec::new();

        let num_blocks_x = (WORLD_WIDTH / BLOCK_SIZE) as usize;
        let num_blocks_y = (WORLD_HEIGHT / BLOCK_SIZE) as usize;

        // Generate horizontal streets
        for i in 0..=num_blocks_y {
            let y = i as f32 * BLOCK_SIZE;
            let is_avenue = i % AVENUE_INTERVAL == 0;
            let width = if is_avenue { STREET_WIDTH * 1.5 } else { STREET_WIDTH };
            streets.push(Street {
                rect: Rect::new(0.0, y - width / 2.0, WORLD_WIDTH, width),
                is_avenue,
            });
        }

        // Generate vertical streets
        for i in 0..=num_blocks_x {
            let x = i as f32 * BLOCK_SIZE;
            let is_avenue = i % AVENUE_INTERVAL == 0;
            let width = if is_avenue { STREET_WIDTH * 1.5 } else { STREET_WIDTH };
            streets.push(Street {
                rect: Rect::new(x - width / 2.0, 0.0, width, WORLD_HEIGHT),
                is_avenue,
            });
        }

        // Generate blocks between streets
        for by in 0..num_blocks_y {
            for bx in 0..num_blocks_x {
                let x = bx as f32 * BLOCK_SIZE + STREET_WIDTH / 2.0;
                let y = by as f32 * BLOCK_SIZE + STREET_WIDTH / 2.0;
                let w = BLOCK_SIZE - STREET_WIDTH;
                let h = BLOCK_SIZE - STREET_WIDTH;
                
                let is_park = rng.gen::<f32>() < 0.15; // 15% chance of park
                
                blocks.push(Block {
                    rect: Rect::new(x, y, w, h),
                    is_park,
                });

                // Generate objects within the block
                if is_park {
                    // Parks have trees
                    let tree_count = rng.gen_range(5..12);
                    for _ in 0..tree_count {
                        let ox = x + rng.gen::<f32>() * w;
                        let oy = y + rng.gen::<f32>() * h;
                        objects.push(WorldObject::new(
                            ox, oy,
                            ObjectType::Tree,
                            &mut rng,
                        ));
                    }
                    // Some benches
                    let bench_count = rng.gen_range(2..5);
                    for _ in 0..bench_count {
                        let ox = x + rng.gen::<f32>() * w;
                        let oy = y + rng.gen::<f32>() * h;
                        objects.push(WorldObject::new(
                            ox, oy,
                            ObjectType::Bench,
                            &mut rng,
                        ));
                    }
                } else {
                    // City blocks have buildings
                    let building_count = rng.gen_range(1..4);
                    let building_padding = 15.0;
                    
                    for i in 0..building_count {
                        let bw = w / building_count as f32 - building_padding;
                        let bh = h - building_padding * 2.0;
                        let ox = x + building_padding / 2.0 + i as f32 * (bw + building_padding);
                        let oy = y + building_padding;
                        
                        objects.push(WorldObject::new_building(
                            ox + bw / 2.0, 
                            oy + bh / 2.0,
                            bw, bh,
                            &mut rng,
                        ));
                    }
                }
            }
        }

        // Add street objects (lampposts, cars, people)
        for street in &streets {
            // Lampposts along streets
            let lamp_spacing = 80.0;
            if street.rect.w > street.rect.h {
                // Horizontal street
                let mut x = street.rect.x + lamp_spacing / 2.0;
                while x < street.rect.x + street.rect.w {
                    if rng.gen::<f32>() < 0.7 {
                        objects.push(WorldObject::new(
                            x, street.rect.y + 5.0,
                            ObjectType::Lamppost,
                            &mut rng,
                        ));
                    }
                    x += lamp_spacing;
                }
            } else {
                // Vertical street
                let mut y = street.rect.y + lamp_spacing / 2.0;
                while y < street.rect.y + street.rect.h {
                    if rng.gen::<f32>() < 0.7 {
                        objects.push(WorldObject::new(
                            street.rect.x + 5.0, y,
                            ObjectType::Lamppost,
                            &mut rng,
                        ));
                    }
                    y += lamp_spacing;
                }
            }

            // Cars and people on streets
            if street.is_avenue {
                let car_count = rng.gen_range(3..8);
                for _ in 0..car_count {
                    let (cx, cy) = if street.rect.w > street.rect.h {
                        (
                            street.rect.x + rng.gen::<f32>() * street.rect.w,
                            street.rect.y + street.rect.h / 2.0,
                        )
                    } else {
                        (
                            street.rect.x + street.rect.w / 2.0,
                            street.rect.y + rng.gen::<f32>() * street.rect.h,
                        )
                    };
                    objects.push(WorldObject::new(cx, cy, ObjectType::Car, &mut rng));
                }
            }

            // People
            let people_count = rng.gen_range(2..6);
            for _ in 0..people_count {
                let (px, py) = if street.rect.w > street.rect.h {
                    (
                        street.rect.x + rng.gen::<f32>() * street.rect.w,
                        street.rect.y + rng.gen::<f32>() * street.rect.h,
                    )
                } else {
                    (
                        street.rect.x + rng.gen::<f32>() * street.rect.w,
                        street.rect.y + rng.gen::<f32>() * street.rect.h,
                    )
                };
                objects.push(WorldObject::new(px, py, ObjectType::Person, &mut rng));
            }
        }

        // Add some hydrants and trash cans
        let misc_count = 50;
        for _ in 0..misc_count {
            let x = rng.gen::<f32>() * WORLD_WIDTH;
            let y = rng.gen::<f32>() * WORLD_HEIGHT;
            let obj_type = if rng.gen::<f32>() < 0.5 {
                ObjectType::Hydrant
            } else {
                ObjectType::TrashCan
            };
            objects.push(WorldObject::new(x, y, obj_type, &mut rng));
        }

        Self {
            streets,
            blocks,
            objects,
            width: WORLD_WIDTH,
            height: WORLD_HEIGHT,
        }
    }

    /// Get spawn position for a player/bot (on a street, avoiding objects)
    pub fn get_spawn_position(&self, rng: &mut impl Rng) -> Vec2 {
        let street = &self.streets[rng.gen_range(0..self.streets.len())];
        vec2(
            street.rect.x + rng.gen::<f32>() * street.rect.w,
            street.rect.y + rng.gen::<f32>() * street.rect.h,
        )
    }

    /// Calculate percentage of city consumed
    pub fn get_consumption_percentage(&self) -> f32 {
        let consumed = self.objects.iter().filter(|o| o.consumed).count();
        let total = self.objects.len();
        if total == 0 { 0.0 } else { consumed as f32 / total as f32 * 100.0 }
    }
}
