use std::rc::Rc;
use std::cell::RefCell;
use std::num::abs;

use game::entity;
use game::entity::Entity;
use game::entity::PhysicalProperties;
use game::map::Map;
use game::sprite::{Sprite, Animation, AnimationPlayer};

use gmath::vectors::Vec2;
use gmath::shapes::Rect;

use keyboard::KeyboardState;

use sdl2::keycode;
use sdl2::render::Renderer;
use sdl2::render::Texture;

pub struct Player {
    accel      : Vec2<f32>,
    vel        : Vec2<f32>,
    pos        : Vec2<f32>,
    base_bounds: Rect,
    base_hitbox: Rect,
    keyboard   : Rc<RefCell<KeyboardState>>,
    on_ground  : bool,
    properties : PhysicalProperties,
    animations : EntityAnimations,
    animation_player: AnimationPlayer
}

pub struct EntityAnimations {
    idle: Animation,
    walk: Animation,
    jump: Animation,
    fall: Animation,
}

impl Entity for Player {
    fn acceleration(&self) -> Vec2<f32> { self.accel }
    fn set_acceleration(&mut self, new_accel: Vec2<f32>) { self.accel = new_accel; }
    fn velocity(&self) -> Vec2<f32> { self.vel }
    fn set_velocity(&mut self, new_vel: Vec2<f32>) { self.vel = new_vel; }
    fn position(&self) -> Vec2<f32> { self.pos }
    fn set_position(&mut self, new_pos: Vec2<f32>) { self.pos = new_pos }
    fn physical_properties<'a>(&'a self) -> &'a PhysicalProperties { &self.properties }
    fn bounds(&self) -> Rect { self.base_bounds.move_vec(self.pos) }
    fn is_on_ground(&self) -> bool { self.on_ground }
    fn hit_y(&mut self, value: bool) { self.on_ground = value }
}

impl Player {
    pub fn rounded_position(&self) -> Vec2<i32> {
        Vec2::new(self.pos.x as i32, self.pos.y as i32)
    }

    pub fn center(&self) -> Vec2<i32> {
        let actual_center = self.bounds().center();
        Vec2::new(actual_center.x as i32, actual_center.y as i32)
    }

    pub fn new(position: Vec2<f32>, spritesheet: Rc<~Texture>,
            keyboard: Rc<RefCell<KeyboardState>>) -> Player {
        let stand_sprite = Sprite {
            spritesheet: spritesheet.clone(),
            offset: Vec2::new(0, 0),
            frame_width: 64,
            frame_height: 128,
            num_frames_x: 1,
            num_frames_y: 1,
        };
        let stand_animation = Animation { sprite: stand_sprite, repeat: true, frame_time: 0.0 };

        let walk_sprite = Sprite {
            spritesheet: spritesheet.clone(),
            offset: Vec2::new(1*64, 0*128),
            frame_width: 64,
            frame_height: 128,
            num_frames_x: 6,
            num_frames_y: 1,
        };
        let walk_animation = Animation { sprite: walk_sprite, repeat: true, frame_time: 0.7 };

        let jump_sprite = Sprite {
            spritesheet: spritesheet.clone(),
            offset: Vec2::new(7*64, 1*128),
            frame_width: 64,
            frame_height: 128,
            num_frames_x: 1,
            num_frames_y: 1,
        };
        let jump_animation = Animation { sprite: jump_sprite, repeat: true, frame_time: 0.6 };

        let falling_sprite = Sprite {
            spritesheet: spritesheet.clone(),
            offset: Vec2::new(8*64, 1*128),
            frame_width: 64,
            frame_height: 128,
            num_frames_x: 2,
            num_frames_y: 1,
        };
        let falling_animation = Animation { sprite: falling_sprite, repeat: true, frame_time: 0.5 };

        Player {
            accel: Vec2::new(0.0, 9.8),
            vel: Vec2::zero(),
            pos: position,
            base_bounds: Rect::new(14.0, 36.0, 32.0, 92.0),
            base_hitbox: Rect::new(0.0, 0.0, 32.0, 32.0),
            keyboard: keyboard,
            on_ground: false,
            properties: PhysicalProperties {
                c_drag        : 0.470,
                mass          : 70.00, // (kg)
                acting_area   : 0.760, // (m^2)
                movement_accel: 6.000,
                max_velocity  : 7.000, // (m/s)
                jump_accel    : 5.000, // (m/s)
                jump_time     : 0.000, // (secs)
                stopping_bonus: 6.000,
            },
            animations: EntityAnimations {
                idle: stand_animation.clone(),
                walk: walk_animation,
                jump: jump_animation,
                fall: falling_animation
            },
            animation_player: AnimationPlayer::new(stand_animation.clone()),
        }
    }

    pub fn update(&mut self, map: &Map, secs: f32) {
        self.handle_input();
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
        }

        self.animation_player.update(secs);
    }

    fn handle_input(&mut self) {
        let keyboard = self.keyboard.borrow().borrow();

        self.accel.x =
            if keyboard.get().is_keydown(keycode::LeftKey) {
                -self.properties.movement_accel * if self.on_ground { 1.0 } else { 0.6 }
            }
            else if keyboard.get().is_keydown(keycode::RightKey) {
                self.properties.movement_accel * if self.on_ground { 1.0 } else { 0.6 }
            }
            else {
                0.0
            };

        if self.on_ground && keyboard.get().is_new_keypress(keycode::UpKey) {
            self.vel = self.vel + Vec2::new(0.0, -self.properties.jump_accel);
            self.animation_player.play(self.animations.jump.clone());
            self.animation_player.reset();
        }
    }

    pub fn draw(&self, camera: Vec2<i32>, renderer: &Renderer) {
        let pos = Vec2::new(self.pos.x as i32, self.pos.y as i32) - camera;
        self.animation_player.draw(pos, renderer);
    }

    #[cfg(debug)]
    pub fn draw_bounding_rect(&self, camera: Vec2<i32>, renderer: &Renderer)  {
        renderer.draw_rect(sdl::rect::Rect::new(
                self.bounds().x as i32 - camera.x, self.bounds().y as i32 - camera.y,
                self.bounds().width as i32, self.bounds().height() as i32));
    }
}
