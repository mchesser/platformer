use std::num::abs;

use game::entity;
use game::entity::{Physics, Object, PhysicalProperties, GRAVITY};
use game::map::Map;
use game::sprite::{Animation, AnimationPlayer};

use gmath::vectors::Vec2;
use gmath::shapes::Rect;

use sdl2::render::Renderer;

pub struct Creature {
    accel      : Vec2<f32>,
    vel        : Vec2<f32>,
    pos        : Vec2<f32>,
    base_bounds: Rect,
    base_hitbox: Rect,
    on_ground  : bool,
    properties : PhysicalProperties,
    move_accel : f32,
    jump_accel : f32,
    animations : CreatureAnimations,
    animation_player: AnimationPlayer
}

pub struct CreatureAnimations {
    idle: Animation,
    walk: Animation,
    jump: Animation,
    fall: Animation,
}

impl Physics for Creature {
    fn acceleration(&self) -> Vec2<f32> { self.accel }
    fn set_acceleration(&mut self, new_accel: Vec2<f32>) { self.accel = new_accel; }
    fn velocity(&self) -> Vec2<f32> { self.vel }
    fn set_velocity(&mut self, new_vel: Vec2<f32>) { self.vel = new_vel; }
    fn position(&self) -> Vec2<f32> { self.pos }
    fn set_position(&mut self, new_pos: Vec2<f32>) { self.pos = new_pos }
    fn bounds(&self) -> Rect { self.base_bounds.move_vec(self.pos) }
    fn is_on_ground(&self) -> bool { self.on_ground }
    fn set_on_ground(&mut self, value: bool) { self.on_ground = value }
    fn get_properties(&self) -> PhysicalProperties { self.properties }
}

impl Object for Creature {
    fn update(&mut self, map: &Map, secs: f32) {
        entity::physics(self, map, secs);

        if abs(self.acceleration().x) != 0.0 {
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
                    self.animation_player.speed_up = 1.0 / abs(self.velocity().x);
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

    fn draw(&self, camera: Vec2<i32>, renderer: &Renderer) {
        let pos = Vec2::new(self.pos.x as i32, self.pos.y as i32) - camera;
        self.animation_player.draw(pos, renderer);
    }
}

impl Creature {
    pub fn new(position: Vec2<f32>, base_bounds: Rect, base_hitbox: Rect,
               properties: PhysicalProperties, move_accel: f32, jump_accel: f32,
               animations: CreatureAnimations) -> Creature {
        Creature {
            accel: Vec2::new(0.0, GRAVITY),
            vel: Vec2::zero(),
            pos: position,
            base_bounds: base_bounds,
            base_hitbox: base_hitbox,
            on_ground: false,
            properties: properties,
            move_accel: move_accel,
            jump_accel: jump_accel,
            animation_player: AnimationPlayer::new(animations.idle.clone()),
            animations: animations,
        }
    }

    pub fn center(&self) -> Vec2<i32> {
        let actual_center = self.bounds().center();
        Vec2::new(actual_center.x as i32, actual_center.y as i32)
    }

    #[cfg(debug)]
    pub fn draw_bounding_rect(&self, camera: Vec2<i32>, renderer: &Renderer)  {
        renderer.draw_rect(sdl::rect::Rect::new(
                self.bounds().x as i32 - camera.x, self.bounds().y as i32 - camera.y,
                self.bounds().width as i32, self.bounds().height() as i32));
    }
}
