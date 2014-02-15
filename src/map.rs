use std::vec;
use std::rc::Rc;
use std::io::{File, BufReader, MemReader};

use sdl2::render;
use sdl2::rect::Rect;

use gmath::vectors::Vec2;
use game::tiles::{TileSet, TileInfo};

pub struct Map {
    priv tiles: ~[u16],
    priv width_: uint,
    priv height_: uint,
    priv tileset: Rc<TileSet>
}

impl Map {
    /// Loads a map from a file
    pub fn load_map(file: &mut File, tileset: Rc<TileSet>) -> Map {
        static VERSION: u8 = 1;
        static MAGIC_ID: [u8, ..3] = ['M' as u8, 'A' as u8, 'P' as u8];

        let mut header_buffer = [0, ..12];
        // Load header into the buffer
        match file.read(header_buffer) {
            Ok(n) if n == 12 => {},
            _ => fail!("Could not read file header"),
        }

        let mut reader = BufReader::new(header_buffer);

        // Check the magic id
        match (reader.read_byte(), reader.read_byte(), reader.read_byte()) {
            (Ok(a), Ok(b), Ok(c)) if [a, b, c] == MAGIC_ID => {},
            _ => fail!("Invalid magic id")
        }

        // Check the version number
        match reader.read_byte() {
            Ok(a) if a == VERSION => {},
            _ => fail!("Invalid map version")
        }

        // Get the width and height of the map
        let width = match reader.read_le_u32() {
            Ok(w) => w as uint,
            Err(err) => fail!("Failed to get map width: {}", err.desc)
        };
        let height = match reader.read_le_u32() {
            Ok(h) => h as uint,
            Err(err) => fail!("Failed to get map height: {}", err.desc)
        };

        // Read the tiles
        let mut tile_buffer: ~[u8] = vec::from_elem(width * height * 2, 0u8);
        match file.read(tile_buffer) {
            Ok(n) if n == width * height * 2 => {},
            Ok(n) => fail!("Invalid number of tiles, expected: {}, but found: {}",
                           width*height*2, n),
            _ => fail!("Could not load map tiles")
        }

        let mut reader = MemReader::new(tile_buffer);
        let tiles = vec::from_fn(width * height, |_| {
            match reader.read_le_u16() {
                Ok(x) => x,
                Err(err) => fail!("Failed to read map: {}", err.desc)
            }
        });

        Map {
            tiles: tiles,
            width_: width,
            height_: height,
            tileset: tileset
        }
    }

    pub fn pixel_width(&self) -> i32 {
        self.width_ as i32 * self.tile_size()
    }

    pub fn pixel_height(&self) -> i32 {
        self.height_ as i32 * self.tile_size()
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

    pub fn draw(&self, camera: Vec2<i32>, renderer: &render::Renderer) {
        for tile_x in range(0, self.width()) {
            for tile_y in range(0, self.height()) {
                let x = (tile_x * self.tile_size() as uint) as i32;
                let y = (tile_y * self.tile_size() as uint) as i32;
                let dest_rect = Rect::new(x - camera.x, y - camera.y,
                        self.tile_size() as i32, self.tile_size() as i32);
                self.tileset.borrow().draw(self.get(tile_x, tile_y), dest_rect, renderer);
            }
        }
    }
}
