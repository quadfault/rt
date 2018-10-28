// cameras/orthographic_camera.rs - An orthographic-projecting camera.
// Written by quadfault
// 10/26/18

use crate::math::{ Point, Ray, Vector };

use super::Camera;

pub struct OrthographicCamera {
    horizontal_pixel_count: f64,
    vertical_pixel_count: f64,
    pixel_width: f64,
    pixel_height: f64,
    horizontal_sample_offset: f64,
    vertical_sample_offset: f64,
    d: Vector,
    samples_per_pixel: usize,
}

impl OrthographicCamera {
    pub fn new(horizontal_pixel_count: usize,
               vertical_pixel_count: usize,
               view_plane_width: f64,
               samples_per_pixel: usize)
        -> Self
    {
        let horizontal_pixel_count = horizontal_pixel_count as f64;
        let vertical_pixel_count = vertical_pixel_count as f64;
        let view_plane_height = view_plane_width
            * (vertical_pixel_count / horizontal_pixel_count);
        let pixel_width = view_plane_width / horizontal_pixel_count;
        let pixel_height = view_plane_height / vertical_pixel_count;

        Self {
            horizontal_pixel_count,
            vertical_pixel_count,
            pixel_width,
            pixel_height,
            horizontal_sample_offset: (pixel_width - view_plane_width) / 2.0,
            vertical_sample_offset: (pixel_height - view_plane_height) / 2.0,
            d: Vector::new(0.0, 0.0, -1.0),
            samples_per_pixel,
        }
    }
}

impl Camera for OrthographicCamera {
    fn rays<'a>(&'a self, x: usize, y: usize) 
        -> Box<dyn Iterator<Item=Ray> + 'a>
    {
        Box::new(Rays {
            x: x as f64,
            y: y as f64,
            rays_left: self.samples_per_pixel,
            camera: self,
        })
    }
}

struct Rays<'a> {
    x: f64,
    y: f64,
    rays_left: usize,
    camera: &'a OrthographicCamera,
}

impl<'a> Iterator for Rays<'a> {
    type Item = Ray;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rays_left == 0 {
            None
        } else {
            self.rays_left -= 1;

            let o = Point::new(
                self.x * self.camera.pixel_width
                    + self.camera.horizontal_sample_offset,
                self.y * self.camera.pixel_height
                    + self.camera.vertical_sample_offset,
                0.0,
            );

            Some(Ray { o, d: self.camera.d })
        }
    }
}
