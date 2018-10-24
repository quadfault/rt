// main.rs - A raytracer.
// Written by quadfault
// 10/18/18

mod camera;
mod materials;
mod math;
mod models;
mod scene;

use rand::prelude::*;

use self::camera::Camera;
use self::math::{ Color, Point, Ray, Vector };
use self::materials::*;
use self::models::*;
use self::scene::Scene;

fn main() {
    let nx = 1000;
    let ny = 500;
    let ns = 1000;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let mut scene = Scene::new();
    scene.add(Box::new(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        Box::new(Lambertian::new(Vector::new(0.1, 0.2, 0.5)))
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
        Box::new(Dielectric::new(1.5))
    )));
    scene.add(Box::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        -0.45,
        Box::new(Dielectric::new(1.5)),
    )));

    let lookfrom = Point::new(3.0, 3.0, 2.0);
    let lookat = Point::new(0.0, 0.0, -1.0);
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vector::new(0.0, 1.0, 0.0),
        20.0,
        nx as f64 / ny as f64,
        2.0,
        (lookfrom - lookat).norm(),
    );

    let mut rng = thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut c = Color::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                let v = (j as f64 + rng.gen::<f64>()) / ny as f64;

                let r = camera.get_ray(u, v);

                c += color(r, &mut scene, 0, &mut rng);
            }
            c /= ns as f32;
            c = Color::new(c.r.sqrt(), c.g.sqrt(), c.b.sqrt());

            let ir = (255.99 * c.r) as i32;
            let ig = (255.99 * c.g) as i32;
            let ib = (255.99 * c.b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn color(r: Ray, scene: &mut Scene, depth: i32, rng: &mut ThreadRng) -> Color {
    match scene.hit(&r, 0.001, std::f64::MAX) {
        Some(hr) => {
            if depth < 50 {
                match hr.material.scatter(&r, &hr) {
                    Some(sr) => color(sr.scattered, scene, depth + 1, rng)
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
