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
    pub fn new(lookfrom: Point,
               lookat: Point,
               vup: Vector,
               vfov: f32,
               aspect: f32)
        -> Self
    {
        let theta = vfov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let origin = lookfrom;
        let w = (lookfrom - lookat).as_unit();
        let u = vup.cross(w).as_unit();
        let v = w.cross(u);

        Self {
            origin,
            lower_left_corner: origin - u * half_width - v * half_height - w,
            horizontal: u * (2.0 * half_width),
            vertical: v * (2.0 * half_height),
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
