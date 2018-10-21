// camera.rs - Cameras.
// Written by quadfault
// 10/19/18

use crate::math::{ Point, Ray, Vector };

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vector,
    vertical: Vector,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            origin: Point::new(0.0, 0.0, 2.0),
            lower_left_corner: Point::new(-2.0, -1.0, -1.0),
            horizontal: Vector::new(4.0, 0.0, 0.0),
            vertical: Vector::new(0.0, 2.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner 
                + self.horizontal * u
                + self.vertical * v
                - self.origin,
        )
    }
}
