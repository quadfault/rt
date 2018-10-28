// materials/mod.rs - Materials.
// Written by quadfault
// 10/20/18

mod dielectric;
mod lambertian;
mod metal;

pub use self::dielectric::*;
pub use self::lambertian::*;
pub use self::metal::*;

use rand::prelude::*;

use crate::models::HitResult;
use crate::math::{ Ray, Vector };

pub struct ScatterResult {
    pub scattered: Ray,
    pub attenuation: Vector,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hr: &HitResult) -> Option<ScatterResult>;
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

fn reflect(v: Vector, n: Vector) -> Vector {
    v - n * (2.0 * v.dot(n))
}

fn refract(v: Vector, n: Vector, ni_over_nt: f64) -> Option<Vector> {
    let uv = v.hat();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some((uv - n * dt) * ni_over_nt - n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f64, refractive_index: f64) -> f64 {
    let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    let r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
