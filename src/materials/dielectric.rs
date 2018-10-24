// materials/dielectric.rs - Dielectric materials.
// Written by quadfault
// 10/24/18

use rand::prelude::*;

use crate::models::HitResult;
use crate::math::{ Ray, Vector };

use super::{ Material, ScatterResult, reflect, refract, schlick };

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, hr: &HitResult) -> Option<ScatterResult> {
        let reflected = reflect(r.d, hr.n);
        let outward_normal;
        let ni_over_nt;
        let cosine;

        if r.d.dot(hr.n) > 0.0 {
            outward_normal = -hr.n;
            ni_over_nt = self.refractive_index;
            cosine = self.refractive_index * r.d.dot(hr.n) / r.d.norm();
        } else {
            outward_normal = hr.n;
            ni_over_nt = 1.0 / self.refractive_index;
            cosine = -r.d.dot(hr.n) / r.d.norm();
        }

        let reflect_prob;
        let mut refracted = Vector::zero();
        match refract(r.d, outward_normal, ni_over_nt) {
            Some(r) => {
                refracted = r;
                reflect_prob = schlick(cosine, self.refractive_index);
            }
            None => {
                reflect_prob = 1.0;
            }
        }

        if thread_rng().gen::<f64>() < reflect_prob {
            Some(ScatterResult {
                scattered: Ray::new(hr.p, reflected),
                attenuation: Vector::new(1.0, 1.0, 1.0),
            })
        } else {
            Some(ScatterResult {
                scattered: Ray::new(hr.p, refracted),
                attenuation: Vector::new(1.0, 1.0, 1.0),
            })
        }
    }
}
