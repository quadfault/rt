// models.rs - Things that go in scenes and can be rendered.
// Written by quadfault
// 10/19/18

use crate::materials::Material;
use crate::math::{ Point, Ray, Vector };

pub struct HitResult<'a> {
    pub t: f64,
    pub p: Point,
    pub n: Vector,
    pub material: &'a dyn Material,
}

pub trait Model {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitResult>;
}

pub struct Sphere {
    c: Point,
    r: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(c: Point, r: f64, material: Box<dyn Material>) -> Self {
        Self { c, r, material }
    }
}

impl Model for Sphere {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitResult> {
        let oc = r.o - self.c;
        let a = r.d.dot(r.d);
        let b = oc.dot(r.d);
        let c = oc.dot(oc) - self.r * self.r;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / a;
            if tmin < t && t < tmax {
                let p = r.at(t);

                return Some(HitResult {
                    t,
                    p,
                    n: (p - self.c) / self.r,
                    material: self.material.as_ref(),
                })
            }

            let t = (-b + discriminant.sqrt()) / a;
            if tmin < t && t < tmax {
                let p = r.at(t);

                return Some(HitResult {
                    t,
                    p,
                    n: (p - self.c) / self.r,
                    material: self.material.as_ref(),
                })
            }
        }

        None
    }
}

#[cfg(test)]
mod sphere_tests {
    use super::*;

    #[test]
    fn new() {
        let c = Point::new(3.0, 4.0, 5.0);
        let r = 6.0;
        let s = Sphere::new(c, r);

        assert_eq!(s.c, c);
        assert_eq!(s.r, r);
    }
}
