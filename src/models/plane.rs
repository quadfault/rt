// models/plane.rs - Planes.
// Written by quadfault
// 10/28/18

use crate::materials::Material;
use crate::math::{ Point, Ray, Vector };

use super::{ HitResult, Model };

pub struct Plane {
    point_on_plane: Point,
    normal: Vector,
    material: Box<dyn Material>,
}

impl Plane {
    pub fn new(point_on_plane: Point,
               normal: Vector,
               material: Box<dyn Material>)
        -> Self
    {
        Self {
            point_on_plane,
            normal: normal.hat(),
            material,
        }
    }
}

impl Model for Plane {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitResult> {
        let n_dot_d = self.normal.dot(ray.direction);
        if n_dot_d != 0.0 {
            let t = self.normal.dot(self.point_on_plane - ray.origin)
                / n_dot_d;
            if tmin < t && t < tmax {
                let hit_point = ray.at(t);

                Some(HitResult {
                    t,
                    hit_point,
                    normal: self.normal,
                    material: self.material.as_ref(),
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}
