//! Swallow/capture logic and animations

use crate::gameplay::hole::Hole;
use crate::world::objects::{WorldObject, ObjectState};
use crate::world::spatial::SpatialGrid;
use crate::render::vfx::{VfxSystem, VfxType};

/// Growth multiplier for consumed objects
pub const GROWTH_MULTIPLIER: f32 = 0.15;

/// Process swallowing for a hole
pub fn process_swallow(
    hole: &mut Hole,
    objects: &mut [WorldObject],
    spatial: &SpatialGrid,
    vfx: &mut VfxSystem,
) -> Vec<u32> {
    if !hole.is_alive {
        return vec![];
    }

    let mut consumed_ids = Vec::new();
    
    // Query nearby objects
    let nearby = spatial.query_radius(hole.x, hole.y, hole.radius * 2.0);
    
    for idx in nearby {
        let obj = &mut objects[idx];
        
        // Skip already consumed or falling objects
        if obj.consumed || matches!(obj.state, ObjectState::Falling { .. }) {
            continue;
        }
        
        // Check if can capture
        if hole.can_capture_at(obj.x, obj.y, obj.size) {
            // Start falling animation
            obj.start_falling(hole.x, hole.y);
            consumed_ids.push(obj.id);
            
            // Spawn particles
            let particle_count = (obj.size / 5.0).ceil() as usize;
            vfx.spawn(VfxType::SwallowParticles {
                x: obj.x,
                y: obj.y,
                color: obj.color,
                count: particle_count.min(20),
            });
            
            // Spawn ripple
            vfx.spawn(VfxType::Ripple {
                x: hole.x,
                y: hole.y,
                radius: hole.radius,
                color: hole.color,
            });
        }
    }
    
    consumed_ids
}

/// Update falling objects and apply growth
pub fn update_falling_objects(
    hole: &mut Hole,
    objects: &mut [WorldObject],
    dt: f32,
) {
    for obj in objects.iter_mut() {
        if matches!(obj.state, ObjectState::Falling { .. }) {
            if obj.update_falling(dt) {
                // Object finished falling, apply growth
                hole.grow(obj.mass, GROWTH_MULTIPLIER);
            }
        }
    }
}

/// Process hole vs hole combat
pub fn process_hole_combat(
    holes: &mut [Hole],
    player_idx: usize,
    vfx: &mut VfxSystem,
    allow_respawn: bool,
    respawn_time: f32,
) -> Option<usize> {
    let mut eliminations: Vec<(usize, usize)> = Vec::new(); // (winner, loser)
    
    // Check all pairs
    for i in 0..holes.len() {
        for j in (i + 1)..holes.len() {
            if !holes[i].is_alive || !holes[j].is_alive {
                continue;
            }
            
            if !holes[i].overlaps_hole(&holes[j]) {
                continue;
            }
            
            if holes[i].can_consume_hole(&holes[j]) {
                eliminations.push((i, j));
            } else if holes[j].can_consume_hole(&holes[i]) {
                eliminations.push((j, i));
            }
        }
    }
    
    let mut player_eliminated = None;
    
    // Process eliminations
    for (winner, loser) in eliminations {
        // Spawn big VFX
        let loser_hole = &holes[loser];
        vfx.spawn(VfxType::SwallowParticles {
            x: loser_hole.x,
            y: loser_hole.y,
            color: loser_hole.color,
            count: 30,
        });
        
        vfx.spawn(VfxType::Ripple {
            x: holes[winner].x,
            y: holes[winner].y,
            radius: holes[winner].radius * 1.5,
            color: holes[winner].color,
        });
        
        // Apply consumption
        let loser_area = holes[loser].area;
        holes[winner].area += loser_area * 0.5;
        holes[winner].radius = (holes[winner].area / std::f32::consts::PI).sqrt();
        holes[winner].eliminations += 1;
        
        if allow_respawn {
            holes[loser].die(respawn_time);
        } else {
            holes[loser].is_alive = false;
        }
        
        if loser == player_idx {
            player_eliminated = Some(winner);
        }
    }
    
    player_eliminated
}
