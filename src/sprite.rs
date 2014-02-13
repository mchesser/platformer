use std::rc::Rc;
use sdl2::render::Texture;
use sdl2::render::Renderer;
use sdl2::rect::Rect;

use gmath::vectors::Vec2;


pub struct Sprite {
    spritesheet: Rc<~Texture>,
    offset: Vec2<i32>,
    frame_width: i32,
    frame_height: i32,
    num_frames_x: i32,
    num_frames_y: i32,
}

impl Sprite {
    pub fn draw(&self, frame: Vec2<i32>, pos: Vec2<i32>, renderer: &Renderer) {
        assert!(frame.x < self.num_frames_x);
        assert!(frame.y < self.num_frames_y);

        let source_rect = Rect::new(self.offset.x + frame.x * self.frame_width,
                                    self.offset.y + frame.y * self.frame_height,
                                    self.frame_width,
                                    self.frame_height);
        let dest_rect = Rect::new(pos.x, pos.y, self.frame_width, self.frame_height);

        renderer.copy(*self.spritesheet.borrow(), Some(source_rect), Some(dest_rect));
    }
}

pub struct Animation {
    sprite: Sprite,
    frame: Vec2<i32>,
    frame_time: f32,
    wait_time: f32,
}

impl Animation {
    pub fn new(sprite: Sprite, frame_time: f32) -> Animation {
        Animation {
            sprite: sprite,
            frame: Vec2::zero(),
            frame_time: frame_time,
            wait_time: 0.0,
        }
    }

    pub fn update(&mut self, secs: f32) {
        if self.frame_time == 0.0 {
            self.frame.x = 0;
        }
        else {
            self.wait_time += secs;
            while self.wait_time > self.frame_time {
                self.wait_time -= self.frame_time;
                self.frame.x =
                        if self.frame.x+1 < self.sprite.num_frames_x { self.frame.x + 1 } else { 0 };
            }
        }
    }

    pub fn draw(&self, pos: Vec2<i32>, renderer: &Renderer) {
        self.sprite.draw(self.frame, pos, renderer);
    }
}
