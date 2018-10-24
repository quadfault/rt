// materials/metal.rs - Metal.
// Written by quadfault
// 10/24/18

use crate::math::{ Ray, Vector };
use crate::models::HitResult;

use super::{ Material, ScatterResult, random_in_unit_sphere, reflect };

pub struct Metal {
    albedo: Vector,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vector, mut fuzz: f64) -> Self {
        if fuzz > 1.0 {
            fuzz = 1.0;
        }

        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, hr: &HitResult) -> Option<ScatterResult> {
        let reflected = reflect(r.d.hat(), hr.n);
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
