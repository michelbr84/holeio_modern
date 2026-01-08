//! Spatial partitioning for efficient collision detection

use crate::world::objects::WorldObject;
use macroquad::prelude::*;
use std::collections::HashMap;

/// Cell size for spatial grid
pub const CELL_SIZE: f32 = 100.0;

/// Grid cell coordinate
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct CellCoord {
    pub x: i32,
    pub y: i32,
}

impl CellCoord {
    pub fn from_position(x: f32, y: f32) -> Self {
        Self {
            x: (x / CELL_SIZE).floor() as i32,
            y: (y / CELL_SIZE).floor() as i32,
        }
    }
}

/// Spatial grid for efficient lookups
pub struct SpatialGrid {
    /// Maps cell coordinates to object indices
    cells: HashMap<CellCoord, Vec<usize>>,
}

impl SpatialGrid {
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
        }
    }

    /// Build spatial grid from objects
    pub fn build(&mut self, objects: &[WorldObject]) {
        self.cells.clear();
        
        for (idx, obj) in objects.iter().enumerate() {
            if obj.consumed {
                continue;
            }
            
            // Get cells that this object overlaps
            let half_w = obj.width / 2.0;
            let half_h = obj.height / 2.0;
            
            let min_cell = CellCoord::from_position(obj.x - half_w, obj.y - half_h);
            let max_cell = CellCoord::from_position(obj.x + half_w, obj.y + half_h);

            for cx in min_cell.x..=max_cell.x {
                for cy in min_cell.y..=max_cell.y {
                    let coord = CellCoord { x: cx, y: cy };
                    self.cells.entry(coord).or_insert_with(Vec::new).push(idx);
                }
            }
        }
    }

    /// Get indices of objects near a position
    pub fn query_radius(&self, x: f32, y: f32, radius: f32) -> Vec<usize> {
        let mut result = Vec::new();
        let mut seen = std::collections::HashSet::new();

        let min_cell = CellCoord::from_position(x - radius, y - radius);
        let max_cell = CellCoord::from_position(x + radius, y + radius);

        for cx in min_cell.x..=max_cell.x {
            for cy in min_cell.y..=max_cell.y {
                let coord = CellCoord { x: cx, y: cy };
                if let Some(indices) = self.cells.get(&coord) {
                    for &idx in indices {
                        if seen.insert(idx) {
                            result.push(idx);
                        }
                    }
                }
            }
        }

        result
    }

    /// Get indices of objects in a rectangle
    pub fn query_rect(&self, rect: &Rect) -> Vec<usize> {
        let mut result = Vec::new();
        let mut seen = std::collections::HashSet::new();

        let min_cell = CellCoord::from_position(rect.x, rect.y);
        let max_cell = CellCoord::from_position(rect.x + rect.w, rect.y + rect.h);

        for cx in min_cell.x..=max_cell.x {
            for cy in min_cell.y..=max_cell.y {
                let coord = CellCoord { x: cx, y: cy };
                if let Some(indices) = self.cells.get(&coord) {
                    for &idx in indices {
                        if seen.insert(idx) {
                            result.push(idx);
                        }
                    }
                }
            }
        }

        result
    }
}

impl Default for SpatialGrid {
    fn default() -> Self {
        Self::new()
    }
}
