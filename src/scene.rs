// scene.rs - Scenes.
// Written by quadfault
// 10/19/18

use rand::prelude::*;

use crate::cameras::Camera;
use crate::math::{ Color, Ray };
use crate::models::{ HitResult, Model };

pub struct Scene {
    camera: Box<dyn Camera>,
    models: Vec<Box<dyn Model>>,
}

impl Scene {
    pub fn new(camera: Box<dyn Camera>) -> Self {
        Self {
            camera,
            models: vec![],
        }
    }

    pub fn add(&mut self, model: Box<dyn Model>) {
        self.models.push(model);
    }

    pub fn render(&self) {
        println!("P3");
        println!(
            "{} {}",
            self.camera.get_image_width(),
            self.camera.get_image_height()
        );
        println!("255");

        let mut rng = thread_rng();

        for y in (0..self.camera.get_image_height()).rev() {
            for x in 0..self.camera.get_image_width() {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for ray in self.camera.rays(x, y) {
                    pixel_color += self.color(ray, 0, &mut rng);
                }
                pixel_color /= self.camera.get_samples_per_pixel() as f32;

                pixel_color.r = pixel_color.r.sqrt();
                pixel_color.g = pixel_color.g.sqrt();
                pixel_color.b = pixel_color.b.sqrt();

                let ir = (255.99 * pixel_color.r) as i32;
                let ig = (255.99 * pixel_color.g) as i32;
                let ib = (255.99 * pixel_color.b) as i32;

                println!("{} {} {}", ir, ig, ib);
            }
        }
/*
        let mut rng = thread_rng();

        for j in (0..self.image_height).rev() {
            for i in 0..self.image_width {
                let mut c = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let u = (i as f64 + rng.gen::<f64>()) 
                          / self.image_width as f64;
                    let v = (j as f64 + rng.gen::<f64>()) 
                          / self.image_height as f64;

                    let r = self.camera.get_ray(u, v);

                    c += self.color(r, 0, &mut rng);
                }
                c /= self.samples_per_pixel as f32;
                c = Color::new(c.r.sqrt(), c.g.sqrt(), c.b.sqrt());

                let ir = (255.99 * c.r) as i32;
                let ig = (255.99 * c.g) as i32;
                let ib = (255.99 * c.b) as i32;

                println!("{} {} {}", ir, ig, ib);
            }
        }
*/
    }

    pub fn color(&self, r: Ray, depth: i32, rng: &mut ThreadRng) -> Color {
        match self.hit(&r, 0.001, std::f64::MAX) {
            Some(hr) => {
                if depth < 50 {
                    match hr.material.scatter(&r, &hr) {
                        Some(sr) => self.color(sr.scattered, depth + 1, rng)
                                  * sr.attenuation,
                        None => Color::new(0.0, 0.0, 0.0),
                    }
                } else {
                    Color::new(0.0, 0.0, 0.0)
                }
            }
            None => {
                let unit_direction = r.d.hat();
                let t = 0.5 * (unit_direction.y as f32 + 1.0);

                Color::blend(
                    t,
                    Color::new(1.0, 1.0, 1.0),
                    Color::new(0.5, 0.7, 1.0),
                )
            }
        }
    }

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
