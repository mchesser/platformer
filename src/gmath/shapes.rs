#[allow(dead_code)];

use gmath::vectors::Vec2;
use std::num::{min, max};

/// Circle structure, with center and radius
#[deriving(Clone)]
pub struct Circle {
    center: Vec2<f32>,
    radius: f32,
}

/// Rectangle structure
#[deriving(Clone)]
pub struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Rect {
    /// Creates a new rectangle
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Rect {
        Rect {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }

    /// Gets the top left coordinate of the rectangle
    /// # Return
    /// The top left coordinate of the rectangle
    pub fn top_left(&self) -> Vec2<f32> {
        Vec2::new(self.top(), self.left())
    }

    /// Gets the top right coordinate of the rectangle
    /// # Return
    /// The top right coordinate of the rectangle
    pub fn top_right(&self) -> Vec2<f32> {
        Vec2::  new(self.x + self.width, self.y)
    }

    /// Gets the bottom left coordinate of the rectangle
    /// # Return
    /// The bottom left coordinate of the rectangle
    pub fn bottom_left(&self) -> Vec2<f32> {
        Vec2::new(self.x, self.y + self.height)
    }

    /// Gets the bottom right coordinate of the rectangle
    /// # Return
    /// The bottom right coordinate of the rectangle
    pub fn bottom_right(&self) -> Vec2<f32> {
        Vec2::new(self.x + self.width, self.y + self.height)
    }

    /// Gets the center of the rectangle
    /// # Return
    /// The center of the rectangle
    pub fn center(&self) -> Vec2<f32> {
        Vec2::new(self.x + self.width/2.0, self.y + self.height/2.0)
    }

    /// Gets the x value of the left of the rectangle
    /// # Return
    /// The left of the rectangle
    pub fn left(&self) -> f32 {
        self.x
    }

    /// Gets the x value of the right of the rectangle
    /// # Return
    /// The right of the rectangle
    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    /// Gets the y value of the top of the rectangle
    /// # Return
    /// The top of the rectangle
    pub fn top(&self) -> f32 {
        self.y
    }

    /// Gets the y value of the bottom of the rectangle
    /// # Return
    /// The bottom of the rectangle
    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }

    /// Moves the rectangle using a vector
    /// # Arguments
    /// `vec` - the vector to move by
    /// # Return
    /// The moved rectangle
    pub fn move_vec(&self, vec: Vec2<f32>) -> Rect {
        Rect {
            x: self.x + vec.x,
            y: self.y + vec.y,
            width: self.width,
            height: self.height
        }
    }

    /// Calculate the intersection area of two rectangles
    /// # Arguments
    /// `other` - the rectangle to calculate the intersection with
    /// # Return
    /// The intersection area
    pub fn intersect_area(&self, other: &Rect) -> f32 {
        let x_intersect = min(self.right(), other.right()) - max(self.left(), other.left());
        let y_intersect = min(self.bottom(), other.bottom()) - max(self.top(), other.top());

        if x_intersect < 0.0 || y_intersect < 0.0 {
            0.0
        }
        else {
            x_intersect * y_intersect
        }
    }
}
