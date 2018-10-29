// models/sphere.rs - Spheres.
// Written by quadfault
// 10/24/18

use crate::materials::Material;
use crate::math::{ Point, Ray };

use super::{ HitResult, Model };

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point,
               radius: f64,
               material: Box<dyn Material>)
        -> Self
    {
        Self { center, radius, material }
    }
}

impl Model for Sphere {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitResult> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let dsqrt = discriminant.sqrt();
            for &t in &[(-b - dsqrt) / a, (-b + dsqrt) / a] {
                if tmin < t && t < tmax {
                    let hit_point = ray.at(t);

                    return Some(HitResult {
                        t,
                        hit_point,
                        normal: (hit_point - self.center) / self.radius,
                        material: self.material.as_ref(),
                    })
                }
            }
        }

        None
    }
}
