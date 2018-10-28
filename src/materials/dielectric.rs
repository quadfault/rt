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
    fn scatter(&self, ray: &Ray, hr: &HitResult) -> Option<ScatterResult> {
        let reflected = reflect(ray.direction, hr.normal);
        let outward_normal;
        let ni_over_nt;
        let cosine;

        if ray.direction.dot(hr.normal) > 0.0 {
            outward_normal = -hr.normal;
            ni_over_nt = self.refractive_index;
            cosine = self.refractive_index
                * ray.direction.dot(hr.normal)
                / ray.direction.norm();
        } else {
            outward_normal = hr.normal;
            ni_over_nt = 1.0 / self.refractive_index;
            cosine = -ray.direction.dot(hr.normal) / ray.direction.norm();
        }

        let reflect_prob;
        let mut refracted = Vector::zero();
        match refract(ray.direction, outward_normal, ni_over_nt) {
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
                scattered: Ray::new(hr.hit_point, reflected),
                attenuation: Vector::new(1.0, 1.0, 1.0),
            })
        } else {
            Some(ScatterResult {
                scattered: Ray::new(hr.hit_point, refracted),
                attenuation: Vector::new(1.0, 1.0, 1.0),
            })
        }
    }
}
