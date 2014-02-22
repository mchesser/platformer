use std::rc::Rc;
use std::cell::RefCell;
use std::num::clamp;
use std::io::File;

use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2_image::LoadTexture;

use gmath::vectors::Vec2;
use gmath::shapes::Rect;
use game::entity::{Entity, Object, PhysicalProperties};
use game::entity::creature::{Creature, CreatureAnimations};
use game::sprite::{Sprite, Animation};
use game::bitfont::BitFont;
use game::controller::{Controller, KeyboardController, RandomController};
use game::map::Map;
use game::tiles::{TileSet, TileInfo};
use keyboard::KeyboardState;

mod map;
mod tiles;
mod entity;
mod controller;
mod sprite;
mod bitfont;

pub struct Game {
    map: Map,
    tileset: Rc<TileSet>,
    player: Entity<Creature, KeyboardController>,
    cat: Entity<Creature, RandomController>,
    font: BitFont,
    camera: Vec2<i32>,
    background: ~Texture,
}

impl Game {
    pub fn new(keyboard: Rc<RefCell<KeyboardState>>, renderer: &Renderer) -> Game {
        let tile_info = box [
            TileInfo { solid: false, friction: 0.0 },

            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },

            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },

            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 },
        ];
        let tileset = Rc::new(TileSet {
            tile_size: 32,
            sprite: renderer.load_texture(&Path::new("./assets/tileset.png"))
                    .ok().expect("Failed to load tileset"),
            tile_info: tile_info
        });
        let map = Map::load_map(&mut File::open(&Path::new("./assets/maps/map1"))
                .ok().expect("Failed to load map"), tileset.clone());

        // Load human spritesheet
        let human_spritesheet = Rc::new(
                renderer.load_texture(&Path::new("./assets/creatures/player.png"))
                .ok().expect("Failed to load human sprite"));
        // Load cat spritesheet
        let cat_spritesheet = Rc::new(
                renderer.load_texture(&Path::new("./assets/creatures/cat.png"))
                .ok().expect("Failed to load cat sprite"));
        let player = create_player(Vec2::new(50.0, 50.0), keyboard, human_spritesheet.clone());
        let cat = create_cat(Vec2::new(400.0, 50.0), cat_spritesheet.clone());

        let background = renderer.load_texture(&Path::new("./assets/background.png"))
                .ok().expect("Failed to load background image");

        // Load font spritesheet
        let font_spritesheet = Rc::new(
                renderer.load_texture(&Path::new("./assets/fonts/Victoria.png"))
                .ok().expect("Failed to load font"));
        let font = BitFont::new(32, 96, 8, 9, font_spritesheet.clone());

        Game {
            map: map,
            player: player,
            cat: cat,
            font: font,
            tileset: tileset,
            camera: Vec2::zero(),
            background: background,
        }
    }

    pub fn update(&mut self, secs: f32) {
        let map = &self.map;
        self.player.update(map, secs);
        self.cat.update(map, secs);
    }

    pub fn draw(&mut self, renderer: &Renderer) {
        renderer.copy(self.background, None, None);

        // Center the camera on the player:
        let draw_rect = renderer.get_viewport();

        self.camera = self.player.object.center()
                - Vec2::new((draw_rect.w as f32 / 2.0) as i32, (draw_rect.h as f32 / 2.0) as i32);
        self.camera.x = clamp(self.camera.x, 0, self.map.pixel_width() - draw_rect.w);
        self.camera.y = clamp(self.camera.y, 0, self.map.pixel_height() - draw_rect.h);

        self.map.draw(self.camera, renderer);
        self.player.draw(self.camera, renderer);
        self.cat.draw(self.camera, renderer);

        static test_string: &'static str =
r#"Test string,
with multiple lines."#;

        self.font.draw_text(Vec2::new(0, 0), test_string, renderer);
    }
}

fn create_player(position: Vec2<f32>, keyboard: Rc<RefCell<KeyboardState>>,
        spritesheet: Rc<~Texture>) -> Entity<Creature, KeyboardController> {
    let fw = 64;
    let fh = 128;
    let idle = Animation {
        sprite: Sprite {
            spritesheet : spritesheet.clone(),
            offset      : Vec2::new(0, 0),
            frame_width : fw,
            frame_height: fh,
            num_frames_x: 1,
            num_frames_y: 1,
        },
        repeat: true,
        frame_time: 0.0,
    };
    let walk = Animation {
        sprite: Sprite {
            spritesheet : spritesheet.clone(),
            offset      : Vec2::new(1*fw, 0),
            frame_width : fw,
            frame_height: fh,
            num_frames_x: 6,
            num_frames_y: 1,
        },
        repeat: true,
        frame_time: 0.7,
    };
    let jump = Animation {
        sprite: Sprite {
            spritesheet : spritesheet.clone(),
            offset      : Vec2::new(7*fw, 1*fh),
            frame_width : fw,
            frame_height: fh,
            num_frames_x: 1,
            num_frames_y: 1,
        },
        repeat: true,
        frame_time: 0.6,
    };
    let fall = Animation {
        sprite: Sprite {
            spritesheet : spritesheet.clone(),
            offset      : Vec2::new(8*fw, 1*fh),
            frame_width : fw,
            frame_height: fh,
            num_frames_x: 2,
            num_frames_y: 1,
        },
        repeat: true,
        frame_time: 0.5,
    };

    Entity {
        object: Creature::new(
            position,
            Rect::new(14.0, 36.0, 32.0, 92.0),
            Rect::new(0.0, 0.0, 32.0, 32.0),
            PhysicalProperties {
                c_drag    : 0.470,
                mass      : 70.00, // (kg)
                cross_area: 0.760, // (m^2)
                max_vel_x : 9.000, // (m/s)
                stop_bonus: 6.000,
            },
            6.0,
            5.0,
            CreatureAnimations {
                idle: idle,
                walk: walk,
                jump: jump,
                fall: fall,
            }
        ),
        controller: KeyboardController::new(keyboard),
    }
}

fn create_cat(position: Vec2<f32>,
        spritesheet: Rc<~Texture>) -> Entity<Creature, RandomController> {
    let fw = 40;
    let fh = 32;
    let idle = Animation {
        sprite: Sprite {
            spritesheet : spritesheet.clone(),
            offset      : Vec2::new(0, 0),
            frame_width : fw,
            frame_height: fh,
            num_frames_x: 1,
            num_frames_y: 1,
        },
        repeat: true,
        frame_time: 0.0,
    };
    let walk = Animation {
        sprite: Sprite {
            spritesheet : spritesheet.clone(),
            offset      : Vec2::new(0, fh),
            frame_width : fw,
            frame_height: fh,
            num_frames_x: 6,
            num_frames_y: 1,
        },
        repeat: true,
        frame_time: 0.7,
    };
    let jump = Animation {
        sprite: Sprite {
            spritesheet : spritesheet.clone(),
            offset      : Vec2::new(0, 0),
            frame_width : fw,
            frame_height: fh,
            num_frames_x: 1,
            num_frames_y: 1,
        },
        repeat: true,
        frame_time: 0.6,
    };
    let fall = Animation {
        sprite: Sprite {
            spritesheet : spritesheet.clone(),
            offset      : Vec2::new(0, 0),
            frame_width : fw,
            frame_height: fh,
            num_frames_x: 1,
            num_frames_y: 1,
        },
        repeat: true,
        frame_time: 0.5,
    };

    Entity {
        object: Creature::new(
            position,
            Rect::new(2.0, 2.0, 38.0, 30.0),
            Rect::new(0.0, 0.0, 32.0, 32.0),
            PhysicalProperties {
                c_drag    : 0.470,
                mass      : 70.00, // (kg)
                cross_area: 0.760, // (m^2)
                max_vel_x : 4.000, // (m/s)
                stop_bonus: 6.000,
            },
            6.0,
            5.0,
            CreatureAnimations {
                idle: idle,
                walk: walk,
                jump: jump,
                fall: fall,
            }
        ),
        controller: RandomController::new(0.5),
    }
}
