use std::{fs::File, io::Read, path::Path};

use anyhow::Context;
use macroquad::prelude::{Rect, Vec2};

use crate::tiles::{TileInfo, TileSet};

pub struct Map {
    pub width: usize,
    pub height: usize,
    tiles: Vec<u16>,
    tileset: TileSet,
}

impl Map {
    /// Loads a map from a file
    pub fn load_map(path: &Path, tileset: TileSet) -> anyhow::Result<Self> {
        static VERSION: u8 = 1;
        static MAGIC_ID: [u8; 3] = *b"MAP";

        let mut file =
            File::open(path).with_context(|| format!("failed to open: {}", path.display()))?;

        let mut header = [0; 12];
        // Load header into the buffer
        match file.read(&mut header) {
            Ok(n) if n == 12 => {}
            _ => anyhow::bail!("Could not read file header"),
        }

        // Check the magic id
        if &header[0..3] != &MAGIC_ID {
            anyhow::bail!("Invalid magic id");
        }

        // Check the version number
        if header[3] != VERSION {
            anyhow::bail!("Invalid map version");
        }

        // Get the width and height of the map
        let width = u32::from_le_bytes(header[4..8].try_into().unwrap()) as usize;
        let height = u32::from_le_bytes(header[8..12].try_into().unwrap()) as usize;

        // Read the tiles
        let length = width * height * 2;
        let mut tile_buffer = vec![0; length];
        match file.read(&mut tile_buffer) {
            Ok(n) if n == length => {}
            Ok(n) => anyhow::bail!("Invalid number of tiles, expected: {length}, but found: {n}"),
            _ => anyhow::bail!("Could not load map tiles"),
        }

        let tiles =
            tile_buffer.chunks(2).map(|x| u16::from_le_bytes(x.try_into().unwrap())).collect();

        Ok(Self { tiles, width, height, tileset })
    }

    pub fn size(&self) -> Vec2 {
        let tile_size = self.tile_size() as f32;
        Vec2::new(self.width as f32 * tile_size, self.height as f32 * tile_size)
    }

    pub fn tile_size(&self) -> i32 {
        self.tileset.tile_size
    }

    pub fn tile_info_at(&self, x: usize, y: usize) -> TileInfo {
        self.tileset.id(self.get(x, y))
    }

    fn get(&self, x: usize, y: usize) -> u16 {
        assert!(x < self.width);
        assert!(y < self.height);
        self.tiles[x + y * self.width]
    }

    pub fn draw(&self, camera: Vec2) {
        for tile_x in 0..self.width {
            for tile_y in 0..self.height {
                let x = (tile_x * self.tile_size() as usize) as f32;
                let y = (tile_y * self.tile_size() as usize) as f32;
                let dest_rect = Rect::new(
                    x - camera.x,
                    y - camera.y,
                    self.tile_size() as f32,
                    self.tile_size() as f32,
                );
                self.tileset.draw(self.get(tile_x, tile_y), dest_rect);
            }
        }
    }
}
