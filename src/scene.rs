// scene.rs - Scenes.
// Written by quadfault
// 10/19/18

use crate::models::*;
use crate::math::Ray;

pub struct Scene {
    models: Vec<Box<dyn Model>>,
}

impl Scene {
    pub fn new() -> Self {
        Self { models: vec![] }
    }

    pub fn add(&mut self, model: Box<dyn Model>) {
        self.models.push(model);
    }
}

impl Model for Scene {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitResult> {
        let mut closest_so_far = tmax;
        let mut rc = None;

        for model in &self.models {
            match model.hit(r, tmin, closest_so_far) {
                Some(hr) => {
                    closest_so_far = hr.t;
                    rc = Some(hr);
                }
                None => {}
            }
        }

        rc
    }
}
