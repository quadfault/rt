// cameras/pinhole_camera.rs - Pinhole cameras.
// Written by quadfault
// 10/24/18

use crate::math::{ Point, Ray, Vector };

pub struct PinholeCamera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vector,
    vertical: Vector,
}

impl PinholeCamera {
    pub fn new() -> Self {
        Self {
            origin: Point::new(0.0, 0.0, 2.0),
            lower_left_corner: Point::new(-2.0, -1.0, -1.0),
            horizontal: Vector::new(4.0, 0.0, 0.0),
            vertical: Vector::new(0.0, 2.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner
                + self.horizontal * u
                + self.vertical * v
                - self.origin,
        )
    }
}
