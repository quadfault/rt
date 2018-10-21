// ray.rs - Rays.
// Written by quadfault
// 10/18/18

use super::{ Point, Vector };

pub struct Ray {
    pub o: Point,
    pub d: Vector,
}

impl Ray {
    pub fn new(o: Point, d: Vector) -> Self {
        Self { o, d }
    }

    pub fn at(&self, t: f32) -> Point {
        self.o + self.d * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let o = Point::new(5.0, 4.0, 3.0);
        let d = Vector::new(1.0, 0.0, 0.0);
        let r = Ray::new(o, d);

        assert_eq!(r.o, o);
        assert_eq!(r.d, d);
    }
}
