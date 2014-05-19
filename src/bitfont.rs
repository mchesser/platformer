use std::rc::Rc;
use sdl2::render::{Texture, Renderer, FlipNone};

use game::sprite::Sprite;
use gmath::vectors::Vec2;

/// A bitfont that can be used for rendering text
pub struct BitFont {
    ascii_offset: u8,
    sprite: Sprite,
}

impl BitFont {
    /// Create a new bitfont
    /// # Arguments
    /// `ascii_offset` - The offset of the ascii code to the bitfont
    /// `num_chars` - The number of characters in the texture
    /// `char_width` - The width of a single character in pixels
    /// `char_height` - The height of a single character in pixels
    /// `texture` - The bitfont texture
    /// # Return
    /// A bit font using the texture specified
    pub fn new(ascii_offset: u8, num_chars: i32, char_width: i32, char_height: i32,
            texture: Rc<Texture>) -> BitFont {
        BitFont {
            ascii_offset: ascii_offset,
            sprite: Sprite {
                spritesheet: texture,
                offset: Vec2::zero(),
                frame_width: char_width,
                frame_height: char_height,
                num_frames_x: num_chars,
                num_frames_y: 1,
            },
        }
    }

    /// Draw text on the renderer
    /// # Arguments
    /// `position` - The position to draw the text at
    /// `text` - The text to draw
    /// `renderer` - The renderer
    pub fn draw_text(&self, position: Vec2<i32>, text: &str, renderer: &Renderer) {
        let mut pos = position;
        for char in text.bytes() {
            if char == '\n' as u8 {
                pos.y += self.sprite.frame_height;
                pos.x = position.x;
            }
            else if char == '\r' as u8 {
                // Discard
            }
            else {
                let char = (char - self.ascii_offset) as i32;
                self.sprite.draw(Vec2::new(char, 0), pos, FlipNone, renderer);
                pos.x += self.sprite.frame_width;
            }
        }
    }
}
