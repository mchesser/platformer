use std::vec;
use std::rc::Rc;

use sdl2::render;
use sdl2::rect::Rect;

use game::tiles::{TileSet, TileInfo};

pub struct Map {
    priv tiles: ~[u16],
    priv width_: uint,
    priv height_: uint,
    priv tileset: Rc<TileSet>
}

impl Map {
    pub fn new_test(width: uint, height: uint, tileset: Rc<TileSet>) -> Map {
        let mut map = Map {
            tiles: vec::from_elem(width * height, 0u16),
            width_: width,
            height_: height,
            tileset: tileset
        };

        for value in map.tiles.mut_iter().skip(width * (height - 2)) {
            *value = 2;
        }

        map
    }

    pub fn new_from_file(filename: ~str) -> Map {
        unimplemented!();
    }

    pub fn width(&self) -> uint {
        self.width_
    }

    pub fn height(&self) -> uint {
        self.height_
    }

    pub fn tile_size(&self) -> i32 {
        self.tileset.borrow().tile_size
    }

    pub fn tile_info_at(&self, x: uint, y: uint) -> TileInfo {
        self.tileset.borrow().id(self.get(x, y))
    }

    fn get(&self, x: uint, y: uint) -> u16 {
        assert!(x < self.width());
        assert!(y < self.height());
        self.tiles[x + y * self.width()]
    }

    pub fn draw(&self, renderer: &render::Renderer) {
        for tile_x in range(0, self.width()) {
            for tile_y in range(0, self.height()) {
                let x = tile_x * self.tile_size() as uint;
                let y = tile_y * self.tile_size() as uint;

                let dest_rect = Rect::new(x as i32, y as i32,
                        self.tile_size() as i32, self.tile_size() as i32);

                self.tileset.borrow().draw(self.get(tile_x, tile_y), dest_rect, renderer);
            }
        }
    }
}
S