//! # Spatial
//!
//! The `spatial` module provides the [`Vec2`] struct to specify positions on the terminal screen
//! and the [`Direction`] enum to specify and provide utility methods for relative directions.

use std::cmp::Ordering;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use num::cast::ToPrimitive;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Represents a two-dimensional spatial vector.
///
/// It is generally used as a position vector, representing a point on the
/// [`Canvas`](crate::terminal::Canvas). In the terminal, the origin is (by default) set at the
/// top-left corner with `y` increasing downwards, i.e., counting from top to bottom.
///
/// At times, it is also used as a size. The differences are shown in the example below.
///
/// ## Example
///
/// ```rust
/// # use ruscii::spatial::Vec2;
/// # use ruscii::terminal::{Canvas, VisualElement};
/// #
/// let mut canvas = Canvas::new(
///     Vec2::xy(20, 20),  // (20, 20) is used as a size here.
///     &VisualElement::default()
/// );
/// let a = Vec2::xy(20, 20);  // (20, 20) is used as a position here
/// let b = Vec2::xy(19, 19);  // and (19, 19) as a position here.
///
/// assert!(canvas.contains(b));  // b is a valid point on the Canvas.
/// assert!(!canvas.contains(a));  // The bottom-right corner is actually (19, 19).
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    /// Constructs a [`Vec2`] representing (0, 0).
    pub fn zero() -> Vec2 {
        Vec2 { x: 0, y: 0 }
    }

    /// Constructs a [`Vec2`] with the given `x`- and `y`-coordinates.
    pub fn xy<T1: ToPrimitive, T2: ToPrimitive>(x: T1, y: T2) -> Vec2 {
        Vec2 {
            x: x.to_i32().unwrap(),
            y: y.to_i32().unwrap(),
        }
    }

    /// Constructs a [`Vec2`] with the given `x`-coordinate and a `y`-coordinate of 0.
    pub fn x<T: ToPrimitive>(x: T) -> Vec2 {
        Vec2 {
            x: x.to_i32().unwrap(),
            y: 0,
        }
    }

    /// Constructs a [`Vec2`] with the given `y`-coordinate and an `x`-coordinate of 0.
    pub fn y<T: ToPrimitive>(y: T) -> Vec2 {
        Vec2 {
            x: 0,
            y: y.to_i32().unwrap(),
        }
    }

    /// Sets the [`Vec2`] object to (0, 0).
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

impl From<Direction> for Vec2 {
    /// Converts a [`Direction`] to a unit-length [`Vec2`] in that direction.
    ///
    /// Because terminal coordinates begin at the top line, vertical directions are inverted
    /// compared to what you may expect. More information in the example. For [`Direction::None`], a
    /// zero [`Vec2`] is returned.
    ///
    /// ## Example
    ///
    /// In the terminal, `y` increases going downward from the top. Therefore, [`Direction::Up`]
    /// is a negative vector and [`Direction::Down`] is a positive vector.
    ///
    /// ```rust
    /// # use ruscii::spatial::{Direction, Vec2};
    /// #
    /// let up = Vec2::from(Direction::Up);
    /// let down = Vec2::from(Direction::Down);
    ///
    /// assert_eq!(up, Vec2::y(-1));
    /// assert_eq!(down, Vec2::y(1));
    /// ```
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Vec2::y(-1),
            Direction::Down => Vec2::y(1),
            Direction::Right => Vec2::x(1),
            Direction::Left => Vec2::x(-1),
            Direction::None => Vec2::zero(),
        }
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
    /// Returns the opposite [`Direction`].
    ///
    /// For [`Direction::None`], [`Direction::None`] is returned.
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

impl TryFrom<Vec2> for Direction {
    type Error = TryFromVec2Error;

    /// Converts a [`Vec2`] to a [`Direction`].
    ///
    /// Because terminal coordinates begin at the top line, vertical directions are inverted
    /// compared to what you may expect. For more information, see the example.
    ///
    /// ## Errors
    ///
    /// If the given `value` is not orthogonal, i.e., one of the components is not zero,
    /// [`TryFromVec2Error`] is returned.
    ///
    /// ## Examples
    ///
    /// In the terminal, `y` increases going downward from the top. Therefore, [`Direction::Up`]
    /// is a negative vector and [`Direction::Down`] is a positive vector.
    ///
    /// ```rust
    /// # use std::convert::TryFrom;
    /// # use ruscii::spatial::{Direction, Vec2};
    /// #
    /// let negative_y = Direction::try_from(Vec2::y(-1)).unwrap();
    /// let positive_y = Direction::try_from(Vec2::y(1)).unwrap();
    ///
    /// assert_eq!(negative_y, Direction::Up);
    /// assert_eq!(positive_y, Direction::Down);
    /// ```
    ///
    /// Passing non-orthogonal vectors, that is, vectors that are neither parallel to the `x`- or
    /// `y`-axis will result in an error.
    ///
    /// ```rust,should_panic
    /// # use std::convert::TryFrom;
    /// # use ruscii::spatial::{Direction, Vec2};
    /// #
    /// Direction::try_from(Vec2::xy(1, 1)).unwrap();  // panics!
    /// ```
    fn try_from(value: Vec2) -> Result<Self, Self::Error> {
        match (value.x.cmp(&0), value.y.cmp(&0)) {
            (Ordering::Less, Ordering::Equal) => Ok(Direction::Left),
            (Ordering::Greater, Ordering::Equal) => Ok(Direction::Right),
            (Ordering::Equal, Ordering::Greater) => Ok(Direction::Down),
            (Ordering::Equal, Ordering::Less) => Ok(Direction::Up),
            (Ordering::Equal, Ordering::Equal) => Ok(Direction::None),
            _ => Err(TryFromVec2Error { value }),
        }
    }
}

#[derive(Debug)]
pub struct TryFromVec2Error {
    value: Vec2,
}

impl fmt::Display for TryFromVec2Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "displacement vector ({}, {}) is not orthogonal", self.value.x, self.value.y)
    }
}

impl Error for TryFromVec2Error {}
