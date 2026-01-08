//! World rendering - city, streets, buildings

use macroquad::prelude::*;
use crate::world::gen::{World, Street, Block};
use crate::world::objects::{WorldObject, ObjectType, ObjectState};
use crate::render::theme::{Theme, draw_rounded_rect};

/// Draw the entire world
pub fn draw_world(
    world: &World,
    theme: &Theme,
    camera_x: f32,
    camera_y: f32,
    zoom: f32,
) {
    // Draw base ground
    draw_rectangle(
        -camera_x * zoom,
        -camera_y * zoom,
        world.width * zoom,
        world.height * zoom,
        theme.palette.background,
    );

    // Draw streets
    for street in &world.streets {
        draw_street(street, theme, camera_x, camera_y, zoom);
    }

    // Draw blocks (parks)
    for block in &world.blocks {
        if block.is_park {
            draw_park(block, theme, camera_x, camera_y, zoom);
        }
    }

    // Draw objects (sorted by type for proper layering)
    let mut objects_to_draw: Vec<&WorldObject> = world.objects.iter()
        .filter(|o| !o.consumed && !matches!(o.state, ObjectState::Consumed))
        .collect();
    
    // Sort by Y + object type for layering
    objects_to_draw.sort_by(|a, b| {
        let a_layer = get_object_layer(a);
        let b_layer = get_object_layer(b);
        if a_layer != b_layer {
            a_layer.cmp(&b_layer)
        } else {
            a.y.partial_cmp(&b.y).unwrap()
        }
    });

    for obj in objects_to_draw {
        draw_object(obj, theme, camera_x, camera_y, zoom);
    }
}

fn get_object_layer(obj: &WorldObject) -> i32 {
    match obj.obj_type {
        ObjectType::Person => 0,
        ObjectType::Hydrant | ObjectType::TrashCan => 1,
        ObjectType::Lamppost | ObjectType::Bench => 2,
        ObjectType::Car => 3,
        ObjectType::Tree => 4,
        ObjectType::Building => 5,
    }
}

fn draw_street(street: &Street, theme: &Theme, camera_x: f32, camera_y: f32, zoom: f32) {
    let x = (street.rect.x - camera_x) * zoom;
    let y = (street.rect.y - camera_y) * zoom;
    let w = street.rect.w * zoom;
    let h = street.rect.h * zoom;

    // Street surface
    draw_rectangle(x, y, w, h, theme.palette.street);

    // Center line for avenues
    if street.is_avenue {
        let line_color = theme.palette.street_line;
        if w > h {
            // Horizontal street
            let cy = y + h / 2.0;
            let dash_width = 20.0 * zoom;
            let gap = 10.0 * zoom;
            let mut lx = x;
            while lx < x + w {
                draw_rectangle(lx, cy - 1.0 * zoom, dash_width, 2.0 * zoom, line_color);
                lx += dash_width + gap;
            }
        } else {
            // Vertical street
            let cx = x + w / 2.0;
            let dash_height = 20.0 * zoom;
            let gap = 10.0 * zoom;
            let mut ly = y;
            while ly < y + h {
                draw_rectangle(cx - 1.0 * zoom, ly, 2.0 * zoom, dash_height, line_color);
                ly += dash_height + gap;
            }
        }
    }
}

fn draw_park(block: &Block, theme: &Theme, camera_x: f32, camera_y: f32, zoom: f32) {
    let x = (block.rect.x - camera_x) * zoom;
    let y = (block.rect.y - camera_y) * zoom;
    let w = block.rect.w * zoom;
    let h = block.rect.h * zoom;

    draw_rounded_rect(x, y, w, h, 8.0 * zoom, theme.palette.grass);
}

fn draw_object(obj: &WorldObject, theme: &Theme, camera_x: f32, camera_y: f32, zoom: f32) {
    let scale = obj.get_visual_scale();
    if scale < 0.01 {
        return;
    }

    let alpha = obj.get_visual_alpha();
    let x = (obj.x - camera_x) * zoom;
    let y = (obj.y - camera_y) * zoom;
    let w = obj.width * zoom * scale;
    let h = obj.height * zoom * scale;

    let color = Color::new(obj.color.r, obj.color.g, obj.color.b, alpha);
    let shadow = Color::new(0.0, 0.0, 0.0, 0.3 * alpha);

    match obj.obj_type {
        ObjectType::Building => {
            // Shadow
            let shadow_offset = theme.shadow_offset * zoom;
            draw_rectangle(
                x - w / 2.0 + shadow_offset,
                y - h / 2.0 + shadow_offset,
                w, h,
                shadow,
            );
            // Building
            draw_rectangle(x - w / 2.0, y - h / 2.0, w, h, color);
            // Windows (simple grid pattern)
            let window_color = Color::new(0.9, 0.9, 0.6, 0.8 * alpha);
            let window_size = 4.0 * zoom * scale;
            let window_gap = 8.0 * zoom * scale;
            let mut wy = y - h / 2.0 + window_gap;
            while wy < y + h / 2.0 - window_gap {
                let mut wx = x - w / 2.0 + window_gap;
                while wx < x + w / 2.0 - window_gap {
                    draw_rectangle(wx, wy, window_size, window_size, window_color);
                    wx += window_gap + window_size;
                }
                wy += window_gap + window_size;
            }
            // Highlight
            draw_rectangle(
                x - w / 2.0,
                y - h / 2.0,
                w,
                3.0 * zoom * scale,
                theme.palette.highlight,
            );
        }
        ObjectType::Car => {
            // Shadow
            draw_ellipse(x + 2.0 * zoom, y + 2.0 * zoom, w / 2.0, h / 3.0, 0.0, shadow);
            // Car body
            draw_rectangle(x - w / 2.0, y - h / 3.0, w, h * 0.6, color);
            // Roof
            let roof_color = Color::new(color.r * 0.8, color.g * 0.8, color.b * 0.8, alpha);
            draw_rectangle(x - w / 3.0, y - h / 3.0 - h * 0.2, w * 0.6, h * 0.25, roof_color);
            // Wheels
            let wheel_color = Color::new(0.1, 0.1, 0.1, alpha);
            draw_circle(x - w / 3.0, y + h / 4.0, 3.0 * zoom * scale, wheel_color);
            draw_circle(x + w / 3.0, y + h / 4.0, 3.0 * zoom * scale, wheel_color);
        }
        ObjectType::Tree => {
            // Trunk
            let trunk_color = Color::new(0.4, 0.25, 0.1, alpha);
            draw_rectangle(x - 2.0 * zoom * scale, y, 4.0 * zoom * scale, h / 2.0, trunk_color);
            // Foliage (circles)
            draw_circle(x, y - h / 4.0, w / 2.0, color);
            draw_circle(x - w / 3.0, y, w / 3.0, color);
            draw_circle(x + w / 3.0, y, w / 3.0, color);
        }
        ObjectType::Person => {
            // Simple person shape
            let head_radius = w / 3.0;
            draw_circle(x, y - h / 3.0, head_radius, color);
            // Body
            draw_rectangle(x - w / 4.0, y - h / 6.0, w / 2.0, h / 2.0, color);
        }
        ObjectType::Lamppost => {
            // Pole
            let pole_color = Color::new(0.3, 0.3, 0.3, alpha);
            draw_rectangle(x - 1.5 * zoom * scale, y - h, 3.0 * zoom * scale, h, pole_color);
            // Lamp
            let lamp_color = Color::new(1.0, 0.9, 0.5, alpha);
            draw_circle(x, y - h, 4.0 * zoom * scale, lamp_color);
        }
        ObjectType::Hydrant => {
            // Body
            draw_rectangle(x - w / 2.0, y - h / 2.0, w, h, color);
            // Cap
            let cap_color = Color::new(color.r * 0.8, color.g * 0.1, color.b * 0.1, alpha);
            draw_circle(x, y - h / 2.0, w / 2.0, cap_color);
        }
        ObjectType::TrashCan => {
            // Can body
            draw_rectangle(x - w / 2.0, y - h / 2.0, w, h, color);
            // Lid
            let lid_color = Color::new(color.r * 1.2, color.g * 1.2, color.b * 1.2, alpha);
            draw_rectangle(x - w / 2.0 - 1.0 * zoom, y - h / 2.0, w + 2.0 * zoom, 3.0 * zoom * scale, lid_color);
        }
        ObjectType::Bench => {
            // Seat
            draw_rectangle(x - w / 2.0, y - 2.0 * zoom * scale, w, 4.0 * zoom * scale, color);
            // Legs
            let leg_color = Color::new(0.2, 0.2, 0.2, alpha);
            draw_rectangle(x - w / 2.0 + 2.0 * zoom, y, 2.0 * zoom * scale, 4.0 * zoom * scale, leg_color);
            draw_rectangle(x + w / 2.0 - 4.0 * zoom, y, 2.0 * zoom * scale, 4.0 * zoom * scale, leg_color);
        }
    }
}

/// Draw world bounds indicator
pub fn draw_world_bounds(world: &World, theme: &Theme, camera_x: f32, camera_y: f32, zoom: f32) {
    let x = -camera_x * zoom;
    let y = -camera_y * zoom;
    let w = world.width * zoom;
    let h = world.height * zoom;
    
    let border_color = Color::new(1.0, 0.3, 0.3, 0.5);
    let thickness = 4.0;
    
    draw_line(x, y, x + w, y, thickness, border_color);
    draw_line(x, y + h, x + w, y + h, thickness, border_color);
    draw_line(x, y, x, y + h, thickness, border_color);
    draw_line(x + w, y, x + w, y + h, thickness, border_color);
}
