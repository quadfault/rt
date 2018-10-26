// cameras/orthographic_camera.rs - An orthographic-projecting camera.
// Written by quadfault
// 10/26/18

use crate::math::{ Point, Ray, Vector };

use super::Camera;

pub struct OrthographicCamera {
    pixel_width: f64,
    pixel_height: f64,
    horizontal_sample_offset: f64,
    vertical_sample_offset: f64,
    d: Vector,
}

impl OrthographicCamera {
    pub fn new(horizontal_pixel_count: usize,
               vertical_pixel_count: usize,
               view_plane_width: f64)
        -> Self
    {
        let horizontal_pixel_count = horizontal_pixel_count as f64;
        let vertical_pixel_count = vertical_pixel_count as f64;
        let view_plane_height = view_plane_width
            * (vertical_pixel_count / horizontal_pixel_count);
        let pixel_width = view_plane_width / horizontal_pixel_count;
        let pixel_height = view_plane_height / vertical_pixel_count;

        Self {
            pixel_width,
            pixel_height,
            horizontal_sample_offset: (pixel_width - view_plane_width) / 2.0,
            vertical_sample_offset: (pixel_height - view_plane_height) / 2.0,
            d: Vector::new(0.0, 0.0, -1.0),
        }
    }
}

impl Camera for OrthographicCamera {
    fn get_ray(&self, x: usize, y: usize) -> Ray {
        let o = Point::new(
            x as f64 * self.pixel_width + self.horizontal_sample_offset,
            y as f64 * self.pixel_height + self.vertical_sample_offset,
            0.0,
        );

        Ray { o, d: self.d }
    }
}
