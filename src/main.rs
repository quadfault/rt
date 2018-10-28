// main.rs - A raytracer.
// Written by quadfault
// 10/18/18

mod cameras;
mod materials;
mod math;
mod models;
mod scene;

use self::cameras::OrthographicCamera;
use self::math::{ Point, Vector };
use self::materials::*;
use self::models::*;
use self::scene::Scene;

fn main() {
    let scene = build_scene();
    scene.render();
}

fn build_scene() -> Scene {
    let mut scene = Scene::new(
        Box::new(OrthographicCamera::new(1000, 500, 4.0, 1000)),
    );
    scene.add(Box::new(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        Box::new(Lambertian::new(Vector::new(0.8, 0.3, 0.3)))
    )));
    scene.add(Box::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian::new(Vector::new(0.8, 0.8, 0.0)))
    )));
    scene.add(Box::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        Box::new(Metal::new(Vector::new(0.8, 0.6, 0.2), 0.3))
    )));
    scene.add(Box::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        Box::new(Metal::new(Vector::new(0.8, 0.8, 0.8), 1.0))
    )));

    scene
}
