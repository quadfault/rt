// ray.rs - Rays.
// Written by quadfault
// 10/18/18

use super::{ Point, Vector };

pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
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

        assert_eq!(r.origin, o);
        assert_eq!(r.direction, d);
    }
}
