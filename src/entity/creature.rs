use macroquad::prelude::{Rect, Vec2};

use crate::{
    entity::{self, Object, PhysicalProperties, Physics, GRAVITY},
    map::Map,
    sprite::{Animation, AnimationPlayer},
};

pub struct Creature {
    pub accel: Vec2,
    pub vel: Vec2,
    pub pos: Vec2,
    pub base_bounds: Rect,
    pub base_hitbox: Rect,
    pub on_ground: bool,
    pub properties: PhysicalProperties,
    pub move_accel: f32,
    pub jump_accel: f32,
    pub animations: CreatureAnimations,
    pub animation_player: AnimationPlayer,
}

pub struct CreatureAnimations {
    pub idle: Animation,
    pub walk: Animation,
    pub jump: Animation,
    pub fall: Animation,
}

impl Physics for Creature {
    fn acceleration(&self) -> Vec2 {
        self.accel
    }
    fn set_acceleration(&mut self, new_accel: Vec2) {
        self.accel = new_accel;
    }
    fn velocity(&self) -> Vec2 {
        self.vel
    }
    fn set_velocity(&mut self, new_vel: Vec2) {
        self.vel = new_vel;
    }
    fn is_on_ground(&self) -> bool {
        self.on_ground
    }
    fn set_on_ground(&mut self, value: bool) {
        self.on_ground = value
    }
    fn get_properties(&self) -> PhysicalProperties {
        self.properties
    }
}

impl Object for Creature {
    fn position(&self) -> Vec2 {
        self.pos
    }

    fn set_position(&mut self, new_pos: Vec2) {
        self.pos = new_pos
    }

    fn bounds(&self) -> Rect {
        self.base_bounds.offset(self.pos)
    }

    fn update(&mut self, map: &Map, secs: f32) {
        entity::physics(self, map, secs);

        if self.acceleration().x.abs() != 0.0 {
            let flip = self.acceleration().x < 0.0;
            self.animation_player.flip_horizontal(flip);
        }

        // If the entity is on the ground, then it must be standing or walking
        if self.on_ground {
            if self.acceleration().x == 0.0 && self.velocity().x == 0.0 {
                self.animation_player.play(self.animations.idle.clone());
            }
            else {
                self.animation_player.play(self.animations.walk.clone());
                if self.velocity().x != 0.0 {
                    self.animation_player.speed_up = 1.0 / self.velocity().x.abs();
                }
                else {
                    self.animation_player.speed_up = 1.0;
                }
            }
        }
        // The entity is in the air, so it must be jumping or falling
        else {
            if self.velocity().y > 0.0 {
                self.animation_player.play(self.animations.fall.clone());
            }
            else {
                self.animation_player.play(self.animations.jump.clone());
            }
        }

        self.animation_player.update(secs);
    }

    fn draw(&self, camera: Vec2) {
        self.animation_player.draw(self.pos - camera);
    }
}

impl Creature {
    pub fn new(
        position: Vec2,
        base_bounds: Rect,
        base_hitbox: Rect,
        properties: PhysicalProperties,
        move_accel: f32,
        jump_accel: f32,
        animations: CreatureAnimations,
    ) -> Self {
        Self {
            accel: Vec2::new(0.0, GRAVITY),
            vel: Vec2::ZERO,
            pos: position,
            base_bounds,
            base_hitbox,
            on_ground: false,
            properties,
            move_accel,
            jump_accel,
            animation_player: AnimationPlayer::new(animations.idle.clone()),
            animations,
        }
    }

    pub fn center(&self) -> Vec2 {
        self.bounds().center()
    }
}
