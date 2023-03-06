//! # Spatial
//!
//! The `spatial` module provides the [Vec2] struct to specify positions on the terminal screen.

use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};
use num::cast::ToPrimitive;

/// Represents a two-dimensional spatial vector.
///
/// It is generally used as a position vector, representing a point on the [Canvas]. In the
/// terminal, the origin is set at the top-left corner with `y` increasing downwards, i.e.,
/// counting from top to bottom.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    /// Constructs a [Vec2] representing (0, 0).
    pub fn zero() -> Vec2 {
        Vec2 {x: 0, y: 0}
    }

    /// Constructs a [Vec2] with the given `x`- and `y`-coordinates.
    pub fn xy<T1: ToPrimitive, T2: ToPrimitive>(x: T1, y: T2) -> Vec2 {
        Vec2 {x: x.to_i32().unwrap(), y: y.to_i32().unwrap()}
    }

    /// Constructs a [Vec2] with the given `x`-coordinate and a `y`-coordinate of 0.
    pub fn x<T: ToPrimitive>(x: T) -> Vec2 {
        Vec2 {x: x.to_i32().unwrap(), y: 0}
    }

    /// Constructs a [Vec2] with the given `y`-coordinate and an `x`-coordinate of 0.
    pub fn y<T: ToPrimitive>(y: T) -> Vec2 {
        Vec2 {x: 0, y: y.to_i32().unwrap()}
    }

    /// Sets the [Vec2] object to (0, 0).
    pub fn clear(&mut self) {
        self.x = 0;
        self.y = 0;
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Div for Vec2 {
    type Output = Vec2;

    fn div(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl<T: ToPrimitive> Mul<T> for Vec2 {
    type Output = Vec2;

    fn mul(self, scalar: T) -> Vec2 {
        let scalar = scalar.to_i32().unwrap();
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl<T: ToPrimitive> Div<T> for Vec2 {
    type Output = Vec2;

    fn div(self, scalar: T) -> Vec2 {
        let scalar = scalar.to_i32().unwrap();
        Vec2 {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        *self = *self + other
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Vec2) {
        *self = *self - other
    }
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, other: Vec2) {
        *self = *self * other
    }
}

impl DivAssign for Vec2 {
    fn div_assign(&mut self, other: Vec2) {
        *self = *self / other
    }
}

impl<T: ToPrimitive> MulAssign<T> for Vec2 {
    fn mul_assign(&mut self, scalar: T) {
        *self = *self * scalar
    }
}

impl<T: ToPrimitive> DivAssign<T> for Vec2 {
    fn div_assign(&mut self, scalar: T) {
        *self = *self / scalar
    }
}

/// The relative directions in a two-dimensional coordinate system, including up, down, left,
/// right, and none.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
    None,
}

impl Direction {
    /// Converts a [Direction] to a unit-length [Vec2] in that direction.
    ///
    /// For [Direction::None], a zero [Vec2] is returned.
    pub fn vec2(&self) -> Vec2 {
        match *self {
            Direction::Up => Vec2::y(-1),
            Direction::Down => Vec2::y(1),
            Direction::Right => Vec2::x(1),
            Direction::Left => Vec2::x(-1),
            Direction::None => Vec2::zero(),
        }
    }

    /// Returns the opposite [Direction].
    ///
    /// For [Direction::None], [Direction::None] is provided.
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
            Direction::None => Direction::None,
        }
    }
}