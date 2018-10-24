// materials/lambertian.rs - Lambertian material.
// Written by quadfault
// 10/24/18

use crate::math::{ Ray, Vector };
use crate::models::HitResult;

use super::{ Material, ScatterResult, random_in_unit_sphere };

pub struct Lambertian {
    albedo: Vector,
}

impl Lambertian {
    pub fn new(albedo: Vector) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, hr: &HitResult) -> Option<ScatterResult>
    {
        let target = hr.p + hr.n + random_in_unit_sphere();

        Some(ScatterResult {
            scattered: Ray::new(hr.p, target - hr.p),
            attenuation: self.albedo,
        })
    }
}
