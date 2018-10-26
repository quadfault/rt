// cameras/orthographic_camera.rs - An orthographic-projecting camera.
// Written by quadfault
// 10/26/18

use crate::math::{ Point, Ray, Vector };

use super::Camera;

pub struct OrthographicCamera {
    image_width: usize,
    image_height: usize,
    view_plane_width: f64,
    view_plane_height: f64,
}

impl OrthographicCamera {
    pub fn new(image_width: usize,
               image_height: usize,
               view_plane_width: f64,
               view_plane_height: f64)
        -> Self
    {
        Self {
            image_width,
            image_height,
            view_plane_width,
            view_plane_height,
        }
    }
}

impl Camera for OrthographicCamera {
    fn get_ray(&self, x: usize, y: usize) -> Ray {
        let d = Vector::new(0.0, 0.0, -1.0);

        let pixel_width = self.view_plane_width / self.image_width as f64;
        let pixel_height = self.view_plane_height / self.image_height as f64;
        let o = Point::new(
            x as f64 * pixel_width
                + (pixel_width / 2.0)
                - (self.view_plane_width / 2.0),
            y as f64 * pixel_height
                + (pixel_height / 2.0)
                - (self.view_plane_height / 2.0),
            0.0,
        );

        Ray { o, d }
    }
}
