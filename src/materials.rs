// materials.rs - Materials.
// Written by quadfault
// 10/20/18

use rand::prelude::*;

use crate::models::HitResult;
use crate::math::{ Ray, Vector };

pub struct ScatterResult {
    pub scattered: Ray,
    pub attenuation: Vector,
}

pub trait Material {
    fn scatter(&self, r: &Ray, hr: &HitResult) -> Option<ScatterResult>;
}

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

pub struct Metal {
    albedo: Vector,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vector, mut fuzz: f32) -> Self {
        if fuzz > 1.0 {
            fuzz = 1.0;
        }

        Self { albedo, fuzz }
    }

    fn reflect(&self, v: Vector, n: Vector) -> Vector {
        v - n * (2.0 * v.dot(n))
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, hr: &HitResult) -> Option<ScatterResult> {
        let reflected = self.reflect(r.d.as_unit(), hr.n);
        let scattered = Ray::new(
            hr.p,
            reflected + random_in_unit_sphere() * self.fuzz,
        );
        
        if scattered.d.dot(hr.n) > 0.0 {
            Some(ScatterResult {
                scattered,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}

fn random_in_unit_sphere() -> Vector {
    let mut rng = thread_rng();
    let mut p;
    loop {
        p = (Vector::new(
                rng.gen(),
                rng.gen(),
                rng.gen()
            ) * 2.0)
          - Vector::new(1.0, 1.0, 1.0);

        if p.norm_sqr() < 1.0 {
            break;
        }
    }

    p
}
