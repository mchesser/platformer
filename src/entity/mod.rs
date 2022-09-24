use macroquad::prelude::{Rect, Vec2};

use crate::{controller::Controller, map::Map};

pub mod blocks;
pub mod creature;

static PIXEL_SCALE: f32 = 52.0;
static GRAVITY: f32 = 9.8;

/// Structure for an entity. An entity consists of a object and a controller.
/// The controller can be used to support AI as well as user input control
pub struct Entity<A, B> {
    pub object: A,
    pub controller: B,
}

impl<A: Object, B: Controller<A>> Entity<A, B> {
    pub fn update(&mut self, map: &Map, secs: f32) {
        // Update the controller
        self.controller.update(&mut self.object, secs);
        // Update the object
        self.object.update(map, secs);
    }

    pub fn draw(&self, camera: Vec2) {
        self.object.draw(camera);
    }
}

/// Objects are things that can be drawn to the screen
pub trait Object {
    /// Gets the object's position
    /// # Return
    /// Returns the position of the object as a vector
    fn position(&self) -> Vec2;

    /// Sets the object's position
    /// # Arguments
    /// `new_pos` - The position to set
    fn set_position(&mut self, new_pos: Vec2);

    /// Gets the object's bounds
    /// # Return
    /// Returns a rectangle representing the objects physical bounds
    fn bounds(&self) -> Rect;

    /// Updates the object
    /// # Arguments
    /// `map` - The map where the object is currently
    /// `secs` - The total seconds elapsed since the last update
    fn update(&mut self, map: &Map, secs: f32);

    /// Draws the object on the screen
    /// # Arguments
    /// `camera` - The offset due to the camera position
    fn draw(&self, camera: Vec2);
}

/// Defines objects that physics can be applied on. Note that default implementations are provided
/// for setting and getting acceleration for convenience as acceleration is not modified by the
/// physics of the game.
pub trait Physics: Object {
    /// Gets the object's acceleration
    /// # Return
    /// Returns the acceleration of the object as a vector
    fn acceleration(&self) -> Vec2 {
        // If no implementation is provided, then assume the object has zero acceleration
        Vec2::ZERO
    }

    /// Sets the object's acceleration
    /// # Arguments
    /// `new_accel` - The acceleration to set
    fn set_acceleration(&mut self, _new_accel: Vec2) {
        // If no implementation is provided, then setting the acceleration does nothing
    }

    /// Gets the object's velocity
    /// # Return
    /// Returns the velocity of the object as a vector
    fn velocity(&self) -> Vec2;

    /// Sets the object's velocity
    /// # Arguments
    /// `new_vel` - The velocity to set
    fn set_velocity(&mut self, new_vel: Vec2);

    /// Checks if the object is on the ground
    /// # Return
    /// Returns true if the object is on the ground, or false if it is in the air
    fn is_on_ground(&self) -> bool;

    /// Sets that object is on the ground or in the air
    /// # Arguments
    /// `value` - true => object is on the ground, false => in air
    fn set_on_ground(&mut self, _value: bool) {
        // Do nothing if an implementation is not provided
    }

    /// Sets that object is hitting a wall or not
    /// # Arguments
    /// `value` - true => object is hitting a wall, false => not hitting a wall
    fn set_hit_wall(&mut self, _value: bool) {
        // Do nothing if an implementation is not provided
    }

    /// Gets the objects physical properties
    /// # Return
    /// Returns the object's physical properties
    fn get_properties(&self) -> PhysicalProperties;
}

#[derive(Copy, Clone)]
/// A structure that contains important values that are used for physics calculations
pub struct PhysicalProperties {
    // The drag coefficent of the object
    pub c_drag: f32,
    // The mass of the object (kg)
    pub mass: f32,
    // The cross-sectional area of the object (m^2)
    pub cross_area: f32,
    // The maximum horizontal velocity of the object (m/s)
    pub max_vel_x: f32,
    // A friction modifier used for extra control on getting the object to stop
    pub stop_bonus: f32,
}

/// A simple utility function for determining if friction should be applied on a particular object.
/// The rules for applying friction on an object are as follows:
///     * The horizontal acceleration is equal to zero (to prevent friction from affecting control)
/// or
///     * The velocity is not equal to zero, and
///     * The object's acceleration is in the opposite direction to its velocity
///
/// # Arguments
/// `object` - The object to check
/// # Return
/// Returns true if friction should be applied to the object, or false if it friction should not be
/// applied.
fn apply_friction<T: Physics>(object: &T) -> bool {
    object.acceleration().x.abs() == 0.0
        || (object.velocity().x != 0.0
            && object.velocity().x.signum() != object.acceleration().x.signum())
}

fn clamp(value: f32, min: f32, max: f32) -> f32 {
    value.min(max).max(min)
}

/// Update object based on physics
/// # Arguments
/// `object` - the object the apply physics to
/// `map` - the map where the object is
/// `secs` - the number of seconds since the last update
pub fn physics<T: Physics>(object: &mut T, map: &Map, dt: f32) {
    let mut new_velocity = object.velocity();
    // Increase velocity due to acceleration
    new_velocity = new_velocity + object.acceleration() * dt;
    // Decrease velocity due to air resistance
    new_velocity = new_velocity + air_resistance(object) * dt;
    // Decrease X velocity due to friction
    if object.is_on_ground() && apply_friction(object) {
        let friction = 0.9 * GRAVITY * object.get_properties().stop_bonus * dt;
        new_velocity.x = if new_velocity.x < 0.0 {
            (new_velocity.x + friction).min(0.0)
        }
        else {
            (new_velocity.x - friction).max(0.0)
        };
    }

    // Ensure that the velocity doesn't get too hight
    let max_vel = object.get_properties().max_vel_x;
    new_velocity.x = clamp(new_velocity.x, -max_vel, max_vel);
    object.set_velocity(new_velocity);

    let mut new_position = object.position();
    // Calculate the new x position
    let move_x = object.velocity().x * dt * PIXEL_SCALE;
    let collision_x = map_collision_x(object, map, move_x);
    if move_x.abs() > collision_x.abs() {
        new_position.x += collision_x;
        new_velocity.x = 0.0;
        object.set_hit_wall(true);
    }
    else {
        new_position.x += move_x;
        object.set_hit_wall(false);
    }
    object.set_position(new_position);

    // Calculate the new y position
    let move_y = new_velocity.y * dt * PIXEL_SCALE;
    let collision_y = map_collision_y(object, map, move_y);
    if move_y.abs() > collision_y.abs() {
        new_position.y += collision_y;
        new_velocity.y = 0.0;
        object.set_on_ground(true);
    }
    else {
        new_position.y += move_y;
        object.set_on_ground(false);
    }
    object.set_position(new_position);

    object.set_velocity(new_velocity);
}

/// Calculates the air resistance acting on an object
/// # Arguments
/// `object` - The object to calculate the air resistance acting on it
/// # Return
/// Returns a vector representing the air resistance acting on the object, in the direction opposite
/// to the object's velocity
fn air_resistance<T: Physics>(object: &T) -> Vec2 {
    static AIR_DENSITY: f32 = 1.2; // (kg/m^3)
    let speed_squared = object.velocity().length_squared();
    if speed_squared > 0.1 {
        let properties = object.get_properties();
        // Calulate the force of the air resistance
        // (see: http://en.wikipedia.org/wiki/Drag_(physics))
        let force = 0.5 * AIR_DENSITY * speed_squared * properties.c_drag * properties.cross_area;
        // Return a vector of the acceleration due to air resistance
        object.velocity().normalize() * (-force / properties.mass)
    }
    else {
        // Below the threshold velocity the air resistance is 0
        Vec2::ZERO
    }
}

/// Calculates the maximum distance the object can travel in the x direction
fn map_collision_x<T: Physics>(object: &T, map: &Map, max_dist: f32) -> f32 {
    if max_dist != 0.0 {
        let tile_size = map.tile_size() as f32;
        // Map bounds
        let map_top = 0;
        let map_bottom = (map.height - 1) as u32;
        let map_left = 0;
        let map_right = (map.width - 1) as u32;
        // Calculate range of x values to check
        let (start_x, end_x, dir_x) = if object.velocity().x < 0.0 {
            ((object.bounds().left() / tile_size).floor() as u32, map_left, -1)
        }
        else {
            ((object.bounds().right() / tile_size).ceil() as u32, map_right, 1)
        };
        // Calculate the range of y values to check
        let start_y = std::cmp::max(map_top, (object.bounds().top() / tile_size).floor() as u32);
        let end_y = std::cmp::min(map_bottom, (object.bounds().bottom() / tile_size).ceil() as u32);
        // Check the tiles for collision
        let tile = scan_tiles_x(map, start_x, end_x, dir_x, start_y, end_y) as f32;
        // Calculate the new maximum distance
        if object.velocity().x < 0.0 {
            (tile + 1.0) * tile_size - object.bounds().left()
        }
        else {
            tile * tile_size - object.bounds().right()
        }
    }
    else {
        // The object will not move at all
        0.0
    }
}

/// Scan for solid tiles in the x direction
fn scan_tiles_x(map: &Map, start_x: u32, end_x: u32, dir_x: i32, start_y: u32, end_y: u32) -> u32 {
    if dir_x > 0 {
        for x in start_x..=end_x {
            for y in start_y..end_y {
                if map.tile_info_at(x as usize, y as usize).solid {
                    return x;
                }
            }
        }
    }
    else {
        for x in (start_x..=end_x).rev() {
            for y in start_y..end_y {
                if map.tile_info_at(x as usize, y as usize).solid {
                    return x;
                }
            }
        }
    }

    end_x
}

/// Calculates the maximum distance the object can travel in the x direction
fn map_collision_y<T: Physics>(object: &T, map: &Map, max_dist: f32) -> f32 {
    if max_dist != 0.0 {
        let tile_size = map.tile_size() as f32;
        // Map bounds
        let map_top = 0;
        let map_bottom = (map.height - 1) as u32;
        let map_left = 0;
        let map_right = (map.width - 1) as u32;
        // Calculate range of y values to check
        let (start_y, end_y, dir_y) = if object.velocity().y < 0.0 {
            ((object.bounds().top() / tile_size).floor() as u32, map_top, -1)
        }
        else {
            ((object.bounds().bottom() / tile_size).ceil() as u32, map_bottom, 1)
        };
        // Calculate the range of y values to check
        let start_x = std::cmp::max(map_left, (object.bounds().left() / tile_size).floor() as u32);
        let end_x = std::cmp::min(map_right, (object.bounds().right() / tile_size).ceil() as u32);
        // Check the tiles for collision
        let tile = scan_tiles_y(map, start_y, end_y, dir_y, start_x, end_x) as f32;
        // Calculate the new maximum distance
        if object.velocity().y < 0.0 {
            (tile + 1.0) * tile_size - object.bounds().top()
        }
        else {
            tile * tile_size - object.bounds().bottom()
        }
    }
    else {
        // The object will not move at all
        0.0
    }
}

/// Scan for solid tiles in the y direction
fn scan_tiles_y(map: &Map, start_y: u32, end_y: u32, dir_y: i32, start_x: u32, end_x: u32) -> u32 {
    if dir_y > 0 {
        for y in start_y..=end_y {
            for x in start_x..end_x {
                if map.tile_info_at(x as usize, y as usize).solid {
                    return y;
                }
            }
        }
    }
    else {
        for y in (start_y..=end_y).rev() {
            for x in start_x..end_x {
                if map.tile_info_at(x as usize, y as usize).solid {
                    return y;
                }
            }
        }
    }
    end_y
}
