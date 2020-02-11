use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};
use num::cast::ToPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn zero() -> Vec2 {
        Vec2 {x: 0, y: 0}
    }

    pub fn xy<T1: ToPrimitive, T2: ToPrimitive>(x: T1, y: T2) -> Vec2 {
        Vec2 {x: x.to_i32().unwrap(), y: y.to_i32().unwrap()}
    }

    pub fn x<T: ToPrimitive>(x: T) -> Vec2 {
        Vec2 {x: x.to_i32().unwrap(), y: 0}
    }

    pub fn y<T: ToPrimitive>(y: T) -> Vec2 {
        Vec2 {x: 0, y: y.to_i32().unwrap()}
    }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
    None,
}

impl Direction {
    pub fn vec2(&self) -> Vec2 {
        match *self {
            Direction::Up => Vec2::y(-1),
            Direction::Down => Vec2::y(1),
            Direction::Right => Vec2::x(1),
            Direction::Left => Vec2::x(-1),
            Direction::None => Vec2::zero(),
        }
    }

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


