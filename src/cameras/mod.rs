// cameras/mod.rs - Cameras.
// Written by quadfault
// 10/19/18

//mod bad_camera;
mod orthographic_camera;
//mod pinhole_camera;

//pub use self::bad_camera::*;
pub use self::orthographic_camera::*;
//pub use self::pinhole_camera::*;

use crate::math::Ray;

pub trait Camera {
    fn get_image_width(&self) -> usize;
    fn get_image_height(&self) -> usize;
    fn get_samples_per_pixel(&self) -> usize;

    fn rays<'a>(&'a self, x: usize, y: usize)
        -> Box<dyn Iterator<Item=Ray> + 'a>;
}
