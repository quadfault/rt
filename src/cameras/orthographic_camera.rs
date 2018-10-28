// cameras/orthographic_camera.rs - An orthographic-projecting camera.
// Written by quadfault
// 10/26/18

use rand::prelude::*;

use crate::math::{ Point, Ray, Vector };

use super::Camera;

pub struct OrthographicCamera {
    image_width: usize,
    image_height: usize,
    samples_per_pixel: usize,
    pixel_width: f64,
    pixel_height: f64,
    view_plane_half_width: f64,
    view_plane_half_height: f64,
    direction: Vector,
}

impl OrthographicCamera {
    pub fn new(image_width: usize,
               image_height: usize,
               view_plane_width: f64,
               samples_per_pixel: usize)
        -> Self
    {
        let view_plane_height = view_plane_width
            * (image_height as f64 / image_width as f64);
        let pixel_width = view_plane_width / image_width as f64;
        let pixel_height = view_plane_height / image_height as f64;

        Self {
            image_width,
            image_height,
            samples_per_pixel,
            pixel_width,
            pixel_height,
            view_plane_half_width: view_plane_width / 2.0,
            view_plane_half_height: view_plane_height / 2.0,
            direction: Vector::new(0.0, 0.0, -1.0),
        }
    }
}

impl Camera for OrthographicCamera {
    fn get_image_width(&self) -> usize {
        self.image_width
    }

    fn get_image_height(&self) -> usize {
        self.image_height
    }

    fn get_samples_per_pixel(&self) -> usize {
        self.samples_per_pixel
    }

    fn rays<'a>(&'a self, x: usize, y: usize) 
        -> Box<dyn Iterator<Item=Ray> + 'a>
    {
        Box::new(Rays {
            x: x as f64,
            y: y as f64,
            rays_left: self.samples_per_pixel,
            rng: thread_rng(),
            camera: self,
        })
    }
}

struct Rays<'a> {
    x: f64,
    y: f64,
    rays_left: usize,
    rng: ThreadRng,
    camera: &'a OrthographicCamera,
}

impl<'a> Iterator for Rays<'a> {
    type Item = Ray;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rays_left == 0 {
            None
        } else {
            self.rays_left -= 1;

            let horizontal_sample_offset =
                self.rng.gen::<f64>() * self.camera.pixel_width;
            let vertical_sample_offset =
                self.rng.gen::<f64>() * self.camera.pixel_height;

            let origin = Point::new(
                self.x * self.camera.pixel_width
                    - self.camera.view_plane_half_width
                    + horizontal_sample_offset,
                self.y * self.camera.pixel_height
                    - self.camera.view_plane_half_height
                    + vertical_sample_offset,
                0.0,
            );

            Some(Ray {
                origin,
                direction: self.camera.direction
            })
        }
    }
}
