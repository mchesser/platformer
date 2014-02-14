use std::iter::range_step_inclusive;
use std::num::{min, max, abs, clamp, Round};

use gmath::vectors::Vec2;
use gmath::shapes::Rect;
use game::map::Map;

pub mod player;

static PIXEL_SCALE: f32 = 52.0;
static GRAVITY: f32 = 9.8;

pub struct PhysicalProperties {
    // Air resistance properties
    c_drag        : f32,
    mass          : f32, // (kg)
    acting_area   : f32, // (m^2)
    // Movement properties
    movement_accel: f32, // (m/s^2)
    max_velocity  : f32, // (m/s)
    jump_accel    : f32, // (m/s^2)
    jump_time     : f32, // (secs)
    stopping_bonus: f32,
}

pub trait Entity {
    /// Retuns the acceleration of an entity
    fn acceleration(&self) -> Vec2<f32>;
    /// Sets the acceleration of an entity
    fn set_acceleration(&mut self, new_accel: Vec2<f32>);
    /// Returns the velocity of an entity
    fn velocity(&self) -> Vec2<f32>;
    /// Sets the velocity of an entity
    fn set_velocity(&mut self, new_vel: Vec2<f32>);
    /// Gets the position of an entity
    fn position(&self) -> Vec2<f32>;
    /// Sets the position of an entity
    fn set_position(&mut self, new_pos: Vec2<f32>);
    /// Gets the entites physical properties
    fn physical_properties<'a>(&'a self) -> &'a PhysicalProperties;
    // Returns true if friction should be applied, and false otherwise
    fn apply_friction(&self) -> bool {
        abs(self.acceleration().x) == 0.0 || (self.velocity().x != 0.0 &&
                self.velocity().x.signum() != self.acceleration().x.signum())
    }
    /// Returns true if the entity is on the ground
    fn is_on_ground(&self) -> bool;
    /// Notify the entity if it has hit something in the y direction
    fn hit_y(&mut self, value: bool);
    /// Returns the entity's bounding rectangle
    fn bounds(&self) -> Rect;
}

/// Update entity based on physics
/// # Arguments
/// `entity` - the entity the apply physics to
/// `map` - the map where the entity is
/// `secs` - the number of seconds since the last update
pub fn physics<T: Entity>(entity: &mut T, map: &Map, secs: f32) {
    let mut new_velocity = entity.velocity();
    // Increase velocity due to acceleration
    new_velocity = new_velocity + entity.acceleration().mul(secs);
    // Decrease velocity due to air resistance
    new_velocity = new_velocity + air_resistance(entity).mul(secs);
    // Decrease X velocity due to friction
    if entity.is_on_ground() && entity.apply_friction() {
        let friction = 0.9 * GRAVITY * entity.physical_properties().stopping_bonus * secs;
        new_velocity.x =
            if new_velocity.x < 0.0 {
                min(new_velocity.x + friction, 0.0)
            }
            else {
                max(new_velocity.x - friction, 0.0)
            };
    }

    // Ensure that the velocity doesn't get too hight
    let max_vel = entity.physical_properties().max_velocity;
    new_velocity.x = clamp(new_velocity.x, -max_vel, max_vel);
    entity.set_velocity(new_velocity);

    let mut new_position = entity.position();
    // Calculate the new x position
    let move_x = entity.velocity().x * secs * PIXEL_SCALE;
    let collision_x = map_collision_x(entity, map, move_x);
    if abs(move_x) > abs(collision_x) {
        new_position.x += collision_x;
        new_velocity.x = 0.0;
    }
    else {
        new_position.x += move_x;
    }
    entity.set_position(new_position);

    // Calculate the new y position
    let move_y = new_velocity.y * secs * PIXEL_SCALE;
    let collision_y = map_collision_y(entity, map, move_y);
    if abs(move_y) > abs(collision_y) {
        new_position.y += collision_y;
        new_velocity.y = 0.0;
        entity.hit_y(true);
    }
    else {
        new_position.y += move_y;
        entity.hit_y(false);
    }
    entity.set_position(new_position);

    entity.set_velocity(new_velocity);
}

fn air_resistance<T: Entity>(entity: &T) -> Vec2<f32> {
    static AIR_DENSITY: f32 = 1.2; // (kg/m^3)
    if entity.velocity().length_sqr() > 0.1 {
        let properties = entity.physical_properties();
        // Calulate the force of the air resistance
        // (see: http://en.wikipedia.org/wiki/Drag_(physics))
        let force = 0.5 * AIR_DENSITY * entity.velocity().length_sqr() * properties.c_drag *
                properties.acting_area;
        // Return a vector of the acceleration due to air resistance
        entity.velocity().unit().mul(-force / properties.mass)
    }
    else {
        // Below the threshold velocity the air resistance is 0
        Vec2::<f32>::zero()
    }
}

/// Calculates the maximum distance the entity can travel in the x direction
fn map_collision_x<T: Entity>(entity: &T, map: &Map, max_dist: f32) -> f32 {
    if max_dist != 0.0 {
        let tile_size = map.tile_size() as f32;
        // Map bounds
        let map_top = 0;
        let map_bottom = (map.height() - 1) as int;
        let map_left = 0;
        let map_right = (map.width() - 1) as int;
        // Calculate range of x values to check
        let (start_x, end_x, dir_x) =
            if entity.velocity().x < 0.0 {
                ((entity.bounds().left() / tile_size).floor() as int, map_left, -1)
            }
            else {
                ((entity.bounds().right() / tile_size).ceil() as int, map_right, 1)
            };
        // Calculate the range of y values to check
        let start_y = max(map_top, (entity.bounds().top() / tile_size).floor() as int);
        let end_y = min(map_bottom, (entity.bounds().bottom() / tile_size).ceil() as int);
        // Check the tiles for collision
        let tile = scan_tiles_x(map, start_x, end_x, dir_x, start_y, end_y) as f32;
        // Calculate the new maximum distance
        if entity.velocity().x < 0.0 {
            max((tile + 1.0) * tile_size - entity.bounds().left(), max_dist)
        }
        else {
            min(tile * tile_size - entity.bounds().right(), max_dist)
        }
    }
    else {
        // The entity will not move at all
        0.0
    }
}

/// Scan for solid tiles in the x direction
fn scan_tiles_x(map: &Map, start_x: int, end_x: int, dir_x: int, start_y: int, end_y: int) -> int {
    for x in range_step_inclusive(start_x, end_x, dir_x) {
        for y in range(start_y, end_y) {
            if map.tile_info_at(x as uint, y as uint).solid {
                return x;
            }
        }
    }
    end_x
}

/// Calculates the maximum distance the entity can travel in the x direction
fn map_collision_y<T: Entity>(entity: &T, map: &Map, max_dist: f32) -> f32 {
    if max_dist != 0.0 {
        let tile_size = map.tile_size() as f32;
        // Map bounds
        let map_top = 0;
        let map_bottom = (map.height() - 1) as int;
        let map_left = 0;
        let map_right = (map.width() - 1) as int;
        // Calculate range of y values to check
        let (start_y, end_y, dir_y) =
            if entity.velocity().y < 0.0 {
                ((entity.bounds().top() / tile_size).floor() as int, map_top, -1)
            }
            else {
                ((entity.bounds().bottom() / tile_size).ceil() as int, map_bottom, 1)
            };
        // Calculate the range of y values to check
        let start_x = max(map_left, (entity.bounds().left() / tile_size).floor() as int);
        let end_x = min(map_right, (entity.bounds().right() / tile_size).ceil() as int);
        // Check the tiles for collision
        let tile = scan_tiles_y(map, start_y, end_y, dir_y, start_x, end_x) as f32;
        // Calculate the new maximum distance
        if entity.velocity().y < 0.0 {
            (tile + 1.0) * tile_size - entity.bounds().top()
        }
        else {
            tile * tile_size - entity.bounds().bottom()
        }
    }
    else {
        // The entity will not move at all
        0.0
    }
}

/// Scan for solid tiles in the y direction
fn scan_tiles_y(map: &Map, start_y: int, end_y: int, dir_y: int, start_x: int, end_x: int) -> int {
    for x in range(start_x, end_x) {
        for y in range_step_inclusive(start_y, end_y, dir_y) {
            if map.tile_info_at(x as uint, y as uint).solid {
                return y;
            }
        }
    }
    end_y
}
