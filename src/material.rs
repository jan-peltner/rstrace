use rand::RngCore;

use crate::{
    ray::{Hit, Ray3, Scatter},
    vec::Vec3,
};

pub trait Material {
    fn scatter(&self, incident_ray: &Ray3, hit: &Hit, rng: &mut dyn RngCore) -> Option<Scatter>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _incident_ray: &Ray3, hit: &Hit, rng: &mut dyn RngCore) -> Option<Scatter> {
        let reflection_dir = &hit.normal + &Vec3::rand_unit_sphere_vec(rng);
        Some(Scatter {
            attenuation: &self.albedo,
            scattered_ray: Ray3 {
                origin: hit.p.clone(),
                dir: reflection_dir,
            },
        })
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, incident_ray: &Ray3, hit: &Hit, rng: &mut dyn RngCore) -> Option<Scatter> {
        Some(Scatter {
            attenuation: &self.albedo,
            scattered_ray: Ray3 {
                origin: hit.p.clone(),
                dir: incident_ray.dir.norm().reflect(&hit.normal)
                    + (Vec3::rand_unit_sphere_vec(rng) * self.fuzz),
            },
        })
    }
}
