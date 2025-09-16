use crate::{
    material::Material,
    utils::Interval,
    vec::{Color, Point, Vec3},
};

/// Ray in 3d space. A ray has an origin point `Point` and a direction `Vec3`.
#[derive(Clone)]
pub struct Ray3 {
    pub origin: Point,
    pub dir: Vec3,
    pub time: f64,
}

impl Ray3 {
    pub fn with_time(origin: Point, dir: Vec3, time: f64) -> Self {
        Self { origin, dir, time }
    }

    pub fn without_time(origin: Point, dir: Vec3) -> Self {
        Self {
            origin,
            dir,
            time: 0.0,
        }
    }

    pub fn at(&self, t: f64) -> Point {
        &self.origin + &(&self.dir * t)
    }
}

pub struct Hit<'a> {
    pub p: Point,
    pub normal: Vec3,
    pub front_face: bool,
    pub t: f64,
    pub mat: &'a dyn Material,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray3, t_range: &mut Interval) -> Option<Hit>;
}

pub struct Hittables {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Hittables {
    pub fn check_hit(&self, ray: &Ray3, t_range: &mut Interval) -> Option<Hit> {
        let mut closest_hit: Option<Hit> = None;

        for hittable in self.objects.iter() {
            if let Some(hit) = hittable.hit(ray, t_range) {
                t_range.max = hit.t;
                closest_hit = Some(hit);
            }
        }
        closest_hit
    }
}

pub struct Scatter<'a> {
    pub attenuation: &'a Color,
    pub scattered_ray: Ray3,
}
