use rand::{Rng, RngCore};
use std::{fmt::Debug, rc::Rc};

use crate::{
    ray::{Hit, Ray3, Scatter},
    texture::Texture,
    vec::{Color, Point, Vec3},
};

pub trait Material: Debug {
    fn scatter(&self, incident_ray: &Ray3, hit: &Hit, rng: &mut dyn RngCore) -> Option<Scatter>;
    fn emit(&self, _uv: (f64, f64), _p: &Point) -> Color {
        Color::zero()
    }
}

#[derive(Debug, Clone)]
pub struct Lambertian<T: Texture> {
    tex: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(tex: T) -> Rc<Self> {
        Rc::from(Self { tex })
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, incident_ray: &Ray3, hit: &Hit, rng: &mut dyn RngCore) -> Option<Scatter> {
        let mut reflection_dir = &hit.normal + &Vec3::rand_unit_sphere_vec(rng);

        if reflection_dir.near_zero() {
            reflection_dir = hit.normal;
        }

        Some(Scatter {
            attenuation: self.tex.value(hit.uv, &hit.p),
            scattered_ray: Ray3::with_time(hit.p, reflection_dir, incident_ray.time),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Metal<T: Texture> {
    tex: T,
    fuzz: f64,
}

impl<T: Texture> Metal<T> {
    pub fn new(tex: T, fuzz: f64) -> Rc<Self> {
        Rc::from(Self { tex, fuzz })
    }
}

impl<T: Texture> Material for Metal<T> {
    fn scatter(&self, incident_ray: &Ray3, hit: &Hit, rng: &mut dyn RngCore) -> Option<Scatter> {
        Some(Scatter {
            attenuation: self.tex.value(hit.uv, &hit.p),
            scattered_ray: Ray3::with_time(
                hit.p,
                incident_ray.dir.norm().reflect(&hit.normal)
                    + (Vec3::rand_unit_sphere_vec(rng) * self.fuzz),
                incident_ray.time,
            ),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Rc<Self> {
        Rc::from(Self { refractive_index })
    }
}

impl Dielectric {
    // schlick's approximation of fresnel reflectance.
    // - the return value is the probability of reflection and is compared against a uniform random sample in [0,1) at the call site.
    // - at shallow/grazing angles (θ → π/2), reflectance → 1 (almost always reflect).
    // - at steep/normal incidence (θ → 0), reflectance → r0 (usually small, e.g. ~0.04 for glass).
    fn reflectance(refraction_index: f64, cos_theta: f64) -> f64 {
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, incident_ray: &Ray3, hit: &Hit, rng: &mut dyn RngCore) -> Option<Scatter> {
        let refraction_ratio = if hit.front_face {
            1.0 / self.refractive_index // we assume that the outside medium is air which has a
                                        // refractive index of ~1
        } else {
            self.refractive_index
        };
        let unit_dir = incident_ray.dir.norm();

        let cos_theta = (&unit_dir * -1.0).dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        // sin(theta') can't be bigger than 1, so if that is the case (or schlicks's) we need to reflect the ray
        // instead

        let dir = if refraction_ratio * sin_theta > 1.0
            || Self::reflectance(refraction_ratio, cos_theta) > rng.random()
        {
            unit_dir.reflect(&hit.normal)
        } else {
            unit_dir.refract(&hit.normal, refraction_ratio)
        };

        Some(Scatter {
            attenuation: Color {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            scattered_ray: Ray3::with_time(hit.p, dir, incident_ray.time),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Emitter<T: Texture> {
    tex: T,
}

impl<T: Texture> Emitter<T> {
    pub fn new(tex: T) -> Rc<Self> {
        Rc::from(Self { tex })
    }
}

impl<T: Texture> Material for Emitter<T> {
    fn scatter(&self, _incident_ray: &Ray3, _hit: &Hit, _rng: &mut dyn RngCore) -> Option<Scatter> {
        None
    }
    fn emit(&self, uv: (f64, f64), p: &Point) -> Color {
        self.tex.value(uv, p)
    }
}

#[derive(Debug, Clone)]
pub struct Isotropic<T: Texture> {
    tex: T,
}

impl<T: Texture> Isotropic<T> {
    pub fn new(tex: T) -> Rc<Self> {
        Rc::from(Self { tex })
    }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, incident_ray: &Ray3, hit: &Hit, rng: &mut dyn RngCore) -> Option<Scatter> {
        Some(Scatter {
            attenuation: self.tex.value(hit.uv, &hit.p),
            scattered_ray: Ray3::with_time(
                hit.p,
                Vec3::rand_unit_sphere_vec(rng),
                incident_ray.time,
            ),
        })
    }
}
