#[allow(dead_code)];

use std::fmt;
use std::num::{zero, one, sqrt, sin_cos, atan2};

/// A 2-dimensional vector.
#[deriving(Eq, Clone)]
pub struct Vec2<T> {
    x: T,
    y: T
}

impl<T> Vec2<T> {
    /// Create a new 2D vector
    /// # Arguments
    /// `x` - the x coordinate
    /// `y` - the y coordinate
    /// # Return
    /// The new vector
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 { x: x, y: y }
    }
}

impl<T: Primitive + Clone> Vec2<T> {
    /// Create a new vector of length 0
    /// # Return
    /// The new vector
    pub fn zero() -> Vec2<T> {
        Vec2 { x: zero(), y: zero() }
    }

    /// Create the unit vector in the x direction
    /// # Return
    /// The new vector
    pub fn unit_x() -> Vec2<T> {
        Vec2 { x: one(), y: zero() }
    }

    /// Create the unit vector in the y direction
    /// # Return
    /// The new vector
    pub fn unit_y() -> Vec2<T> {
        Vec2 { x: zero(), y: one() }
    }

    /// Calculate the dot product between this and another vector
    /// # Arugments
    /// `other` - the other vector
    /// # Return
    /// The dot product
    pub fn dot(&self, other: &Vec2<T>) -> T {
        (self.x * other.x) + (self.y * other.y)
    }
}

impl<T: Primitive + Clone> Add<Vec2<T>, Vec2<T>> for Vec2<T> {
    /// Adds two vectors togeather
    fn add(&self, _rhs: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self.x + _rhs.x, self.y + _rhs.y)
    }
}

impl<T: Primitive + Clone> Sub<Vec2<T>, Vec2<T>> for Vec2<T> {
    /// Subtracts one vector from another
    fn sub(&self, _rhs: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self.x - _rhs.x, self.y - _rhs.y)
    }
}


impl<T: Float> Vec2<T> {
    /// Create a new vector from polar coordinates
    /// # Arguments
    /// `angle` - the angle of the vector
    /// `mag` - the magnitude of the vector
    /// # Return
    /// The new vector
    pub fn from_polar(angle: T, mag: T) -> Vec2<T> {
        let (sin_a, cos_a) = sin_cos(angle);
        Vec2::new(mag * cos_a, mag * sin_a)
    }

    /// Calculates the length squared of the vector. Avoids taking a square root.
    /// # Return
    /// The length of the vector squared
    pub fn length_sqr(&self) -> T {
        self.dot(self)
    }

    /// Calculates the lenght of the vector
    /// # Return
    /// The length of the vector
    pub fn length(&self) -> T {
        sqrt(self.length_sqr())
    }

    /// Normalises the vector
    pub fn normalize(&mut self) {
        let len = self.length();
        self.x = self.x / len;
        self.y = self.y / len;
    }

    /// Creates a unit vector in the direction of the vector
    /// # Return
    /// The unit vector
    pub fn unit(&self) -> Vec2<T> {
        let len = self.length();
        Vec2::new(self.x / len, self.y / len)
    }

    /// Rotates a vector by a specified angle
    /// # Arguments
    /// `angle` - the angle to rotate by
    pub fn rotate(&mut self, angle: T) {
        let (cos_a, sin_a) = sin_cos(angle);
        let (old_x, old_y) = (self.x.clone(), self.y.clone());
        self.x = old_x*cos_a - old_y*sin_a;
        self.y = old_x*sin_a + old_y*cos_a;
    }

    /// Gets the angle of the vector
    /// # Return
    /// The angle of the vector
    pub fn angle(&self) -> T {
        atan2(self.x.clone(), self.y.clone())
    }
}

impl<T: Mul<T, T>> Vec2<T> {
    /// Creates a new vector equal to the vector scaled by a constant
    /// # Arguments
    /// `scalar` - the scalar to use
    /// # Return
    /// A new vector
    pub fn mul(&self, scalar: T) -> Vec2<T> {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}

impl<T: fmt::Show> ToStr for Vec2<T> {
    /// Provides a string representation of the vector
    /// # Return
    /// A string representing a vector
    fn to_str(&self) -> ~str {
        format!("[{}, {}]", self.x, self.y)
    }
}
