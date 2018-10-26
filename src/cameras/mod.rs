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
    fn rays(&self) -> Box<dyn Iterator<Item=Ray>>;
}
