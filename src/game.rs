use std::rc::Rc;
use std::cell::RefCell;

use sdl2::render::Renderer;
use sdl2_image::LoadTexture;

use gmath::vectors::Vec2;
use game::entity::player::Player;
use game::map::Map;
use game::tiles::{TileSet, TileInfo};
use keyboard::KeyboardState;

mod map;
mod tiles;
mod entity;
mod sprite;

pub struct Game {
    map: Map,
    tileset: Rc<TileSet>,
    player: Player,
}

impl Game {
    pub fn new(keyboard: Rc<RefCell<KeyboardState>>, renderer: &Renderer) -> Game {
        let tile_info = box [
            TileInfo { solid: false, friction: 0.0 },
            TileInfo { solid: true , friction: 1.0 },
            TileInfo { solid: true , friction: 1.0 }
        ];

        let tileset_texture = renderer.load_texture(&Path::new("./assets/tileset.png"))
                .ok().expect("Failed to load tileset");

        let tileset = Rc::new(TileSet {
            tile_size: 32,
            sprite: tileset_texture,
            tile_info: tile_info
        });

        let player_spritesheet = Rc::new(renderer.load_texture(&Path::new("./assets/player.png"))
                .ok().expect("Failed to load player sprite"));

        Game {
            map: Map::new_test(50, 18, tileset.clone()),
            player: Player::new(Vec2::new(50.0, 50.0), player_spritesheet.clone(), keyboard),
            tileset: tileset
        }
    }

    pub fn update(&mut self, secs: f32) {
        let map = &self.map;
        self.player.update(map, secs);
    }

    pub fn draw(&self, renderer: &Renderer) {
        self.map.draw(renderer);
        self.player.draw(renderer);
    }
}
