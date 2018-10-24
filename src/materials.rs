// materials.rs - Materials.
// Written by quadfault
// 10/20/18

use rand::prelude::*;

use crate::models::HitResult;
use crate::math::{ Ray, Vector };

pub struct ScatterResult {
    pub scattered: Ray,
    pub attenuation: Vector,
}

pub trait Material {
    fn scatter(&self, r: &Ray, hr: &HitResult) -> Option<ScatterResult>;
}

pub struct Lambertian {
    albedo: Vector,
}

impl Lambertian {
    pub fn new(albedo: Vector) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, hr: &HitResult) -> Option<ScatterResult>
    {
        let target = hr.p + hr.n + random_in_unit_sphere();

        Some(ScatterResult {
            scattered: Ray::new(hr.p, target - hr.p),
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    albedo: Vector,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vector, mut fuzz: f64) -> Self {
        if fuzz > 1.0 {
            fuzz = 1.0;
        }

        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, hr: &HitResult) -> Option<ScatterResult> {
        let reflected = reflect(r.d.hat(), hr.n);
        let scattered = Ray::new(
            hr.p,
            reflected + random_in_unit_sphere() * self.fuzz,
        );
        
        if scattered.d.dot(hr.n) > 0.0 {
            Some(ScatterResult {
                scattered,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, hr: &HitResult) -> Option<ScatterResult> {
        let reflected = reflect(r.d, hr.n);
        let outward_normal;
        let ni_over_nt;
        let cosine;

        if r.d.dot(hr.n) > 0.0 {
            outward_normal = -hr.n;
            ni_over_nt = self.refractive_index;
            cosine = self.refractive_index * r.d.dot(hr.n) / r.d.norm();
        } else {
            outward_normal = hr.n;
            ni_over_nt = 1.0 / self.refractive_index;
            cosine = -r.d.dot(hr.n) / r.d.norm();
        }

        let reflect_prob;
        let mut refracted = Vector::zero();
        match refract(r.d, outward_normal, ni_over_nt) {
            Some(r) => {
                refracted = r;
                reflect_prob = schlick(cosine, self.refractive_index);
            }
            None => {
                reflect_prob = 1.0;
            }
        }

        if thread_rng().gen::<f64>() < reflect_prob {
            Some(ScatterResult {
                scattered: Ray::new(hr.p, reflected),
                attenuation: Vector::new(1.0, 1.0, 1.0),
            })
        } else {
            Some(ScatterResult {
                scattered: Ray::new(hr.p, refracted),
                attenuation: Vector::new(1.0, 1.0, 1.0),
            })
        }
    }
}

fn random_in_unit_sphere() -> Vector {
    let mut rng = thread_rng();
    let mut p;
    loop {
        p = (Vector::new(
                rng.gen(),
                rng.gen(),
                rng.gen()
            ) * 2.0)
          - Vector::new(1.0, 1.0, 1.0);

        if p.norm_sqr() < 1.0 {
            break;
        }
    }

    p
}

fn reflect(v: Vector, n: Vector) -> Vector {
    v - n * (2.0 * v.dot(n))
}

fn refract(v: Vector, n: Vector, ni_over_nt: f64) -> Option<Vector> {
    let uv = v.hat();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some((uv - n * dt) * ni_over_nt - n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f64, refractive_index: f64) -> f64 {
    let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    let r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
