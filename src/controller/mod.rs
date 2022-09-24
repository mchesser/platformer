use std::marker::PhantomData;

use macroquad::prelude::{is_key_down, KeyCode, Vec2};

use rand::{rngs::ThreadRng, Rng};

use crate::entity::creature::Creature;
use crate::entity::{Object, Physics};

pub trait Controller<A> {
    /// Update the controller
    /// # Arguments
    /// `object` - The object to control
    /// `secs` - The time elapsed sinced last update
    fn update(&mut self, _object: &mut A, _secs: f32) {}
}

pub struct NoneController<A>(PhantomData<A>);

impl<A: Object> NoneController<A> {
    pub fn new() -> NoneController<A> {
        Self(PhantomData)
    }
}

impl<A: Object> Controller<A> for NoneController<A> {
    // Just use default trait implementations
}

/// A controller that controls objects using the keyboard
pub struct KeyboardController;

impl KeyboardController {
    pub fn new() -> KeyboardController {
        KeyboardController
    }
}

impl Controller<Creature> for KeyboardController {
    fn update(&mut self, object: &mut Creature, _: f32) {
        let move_accel = object.move_accel;
        let x_accel = if is_key_down(KeyCode::Left) {
            -move_accel * if object.is_on_ground() { 1.0 } else { 0.6 }
        }
        else if is_key_down(KeyCode::Right) {
            move_accel * if object.is_on_ground() { 1.0 } else { 0.6 }
        }
        else {
            0.0
        };
        let new_accel = Vec2::new(x_accel, object.acceleration().y);
        object.set_acceleration(new_accel);

        let jump_accel = object.jump_accel;
        if object.is_on_ground() && is_key_down(KeyCode::Up) {
            let new_velocity = object.velocity() + Vec2::new(0.0, -jump_accel);
            object.set_velocity(new_velocity);
        }
    }
}

/// A controller that controls objects using randomness
pub struct RandomController {
    rng: ThreadRng,
    move_time: f32,
    wait_time: f32,
}

impl RandomController {
    pub fn new(move_time: f32) -> RandomController {
        RandomController { rng: rand::thread_rng(), move_time, wait_time: 0.0 }
    }
}

impl Controller<Creature> for RandomController {
    fn update(&mut self, object: &mut Creature, secs: f32) {
        self.wait_time += secs;
        if self.wait_time > self.move_time {
            let move_accel = object.move_accel;
            let x_accel = match self.rng.gen::<f32>() {
                dir if dir < 0.5 => 0.0,
                dir if dir < 0.75 => move_accel,
                _ => -move_accel,
            };
            let new_accel = Vec2::new(x_accel, object.acceleration().y);
            object.set_acceleration(new_accel);
            self.wait_time -= self.move_time;
        }
    }
}
