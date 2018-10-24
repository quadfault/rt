// camera.rs - Cameras.
// Written by quadfault
// 10/19/18

use rand::prelude::*;

use crate::math::{ Point, Ray, Vector };

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vector,
    vertical: Vector,
    u: Vector,
    v: Vector,
    w: Vector,
    lens_radius: f64,
}

impl Camera {
    pub fn new(lookfrom: Point,
               lookat: Point,
               vup: Vector,
               vfov: f64,
               aspect: f64,
               aperture: f64,
               focus_dist: f64)
        -> Self
    {
        let theta = vfov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let origin = lookfrom;
        let w = (lookfrom - lookat).hat();
        let u = vup.cross(w).hat();
        let v = w.cross(u);

        Self {
            origin,
            lower_left_corner: origin
                - u * (half_width * focus_dist)
                - v * (half_height * focus_dist)
                - w * focus_dist,
            horizontal: u * (2.0 * half_width * focus_dist),
            vertical: v * (2.0 * half_height * focus_dist),
            u, v, w,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner 
                + self.horizontal * s
                + self.vertical * t
                - self.origin
                - offset,
        )
    }
}

fn random_in_unit_disk() -> Vector {
    let mut rng = thread_rng();

    let mut p;
    loop {
        p = Vector::new(rng.gen(), rng.gen(), 0.0) * 2.0
          - Vector::new(1.0, 1.0, 0.0);

        if p.dot(p) >= 1.0 {
            break;
        }
    }

    p
}
