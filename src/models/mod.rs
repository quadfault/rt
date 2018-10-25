// models/mod.rs - Things that go in scenes and can be rendered.
// Written by quadfault
// 10/19/18

mod sphere;

pub use self::sphere::*;

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
