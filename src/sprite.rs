use std::rc::Rc;
use sdl2::render::{Texture, Renderer};
use sdl2::render::{RendererFlip, FlipNone, FlipHorizontal};
use sdl2::rect::Rect;

use gmath::vectors::Vec2;

#[deriving(Clone, Eq)]
pub struct Sprite {
    spritesheet: Rc<~Texture>,
    offset: Vec2<i32>,
    frame_width: i32,
    frame_height: i32,
    num_frames_x: i32,
    num_frames_y: i32,
}

impl Sprite {
    pub fn draw(&self, frame: Vec2<i32>, pos: Vec2<i32>,
            flip_state: RendererFlip, renderer: &Renderer) {
        assert!(frame.x < self.num_frames_x);
        assert!(frame.y < self.num_frames_y);

        let source = Rect::new(self.offset.x + frame.x * self.frame_width,
                self.offset.y + frame.y * self.frame_height,
                self.frame_width,
                self.frame_height);
        let dest = Rect::new(pos.x, pos.y, self.frame_width, self.frame_height);

        renderer.copy_ex(*self.spritesheet.borrow(), Some(source), Some(dest),
            0.0, None, flip_state);
    }
}

#[deriving(Clone, Eq)]
pub struct Animation {
    sprite: Sprite,
    frame_time: f32,
    repeat: bool,
}

pub struct AnimationPlayer {
    animation: Animation,
    frame: Vec2<i32>,
    speed_up: f32,
    wait_time: f32,
    stopped: bool,
    flip_state: RendererFlip,
}

impl AnimationPlayer {
    pub fn new(animation: Animation) -> AnimationPlayer {
        AnimationPlayer {
            animation: animation,
            frame: Vec2::zero(),
            speed_up: 1.0,
            wait_time: 0.0,
            stopped: false,
            flip_state: FlipNone,
        }
    }

    pub fn play(&mut self, animation: Animation) {
        if self.animation != animation {
            self.animation = animation;
            self.reset();
        }
    }

    pub fn reset(&mut self) {
        self.frame = Vec2::zero();
        self.wait_time = 0.0;
        self.speed_up = 1.0;
        self.stopped = false;
    }

    pub fn flip_horizontal(&mut self, flip: bool) {
        if flip {
            self.flip_state = FlipHorizontal;
        }
        else {
            self.flip_state = FlipNone;
        }
    }

    pub fn update(&mut self, secs: f32) {
        if !self.stopped  && self.animation.frame_time != 0.0 {
            self.wait_time += secs;
            while self.wait_time > self.animation.frame_time * self.speed_up {
                self.wait_time -= self.animation.frame_time * self.speed_up;
                self.frame.x =
                    if self.frame.x+1 < self.animation.sprite.num_frames_x {
                        self.frame.x + 1
                    }
                    else {
                        if !self.animation.repeat {
                            self.stopped = true;
                            break;
                        }
                        else { 0 }
                    };
            }
        }
    }

    pub fn draw(&self, pos: Vec2<i32>, renderer: &Renderer) {
        self.animation.sprite.draw(self.frame, pos, self.flip_state, renderer);
    }
}
