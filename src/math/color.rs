// color.rs - Colors.
// Written by quadfault
// 10/18/18

use std::ops::{ AddAssign, DivAssign, Mul };

use super::Vector;

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn blend(t: f32, start: Self, end: Self) -> Self {
        Self {
            r: (1.0 - t) * start.r + t * end.r,
            g: (1.0 - t) * start.g + t * end.g,
            b: (1.0 - t) * start.b + t * end.b,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl DivAssign<f32> for Color {
    fn div_assign(&mut self, rhs: f32) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(
            rhs * self.r,
            rhs * self.g,
            rhs * self.b,
        )
    }
}

impl Mul<Vector> for Color {
    type Output = Self;

    fn mul(self, rhs: Vector) -> Self::Output {
        Self::new(
            self.r * rhs.x,
            self.g * rhs.y,
            self.b * rhs.z,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let c = Color::new(0.5, 0.25, 0.0);

        assert_eq!(c.r, 0.5);
        assert_eq!(c.g, 0.25);
        assert_eq!(c.b, 0.0);
    }
}
