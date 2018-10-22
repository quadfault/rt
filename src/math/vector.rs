// vector.rs - Vectors.
// Written by quadfault
// 10/18/18

use std::ops::{ Add, Div, Mul, Neg, Sub };

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn norm(&self) -> f32 {
        self.norm_sqr().sqrt()
    }

    pub fn norm_sqr(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn hat(&self) -> Self {
        let n = self.norm();

        Self::new(self.x / n, self.y / n, self.z / n)
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        )
    }
}

impl Div<f32> for Vector {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output::new(
            self.x / rhs,
            self.y / rhs,
            self.z / rhs,
        )
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output::new(
            rhs * self.x,
            rhs * self.y,
            rhs * self.z,
        )
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output::new(
            -self.x,
            -self.y,
            -self.z,
        )
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let v = Vector::new(6.0, -4.0, 1.5);

        assert_eq!(v.x, 6.0);
        assert_eq!(v.y, -4.0);
        assert_eq!(v.z, 1.5);
    }

    #[test]
    fn zero() {
        let v = Vector::zero();

        assert_eq!(v, Vector::new(0.0, 0.0, 0.0));
    }
}
