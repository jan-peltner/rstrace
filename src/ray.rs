use std::{fmt::Debug, sync::Arc};

use rand::Rng;

use crate::{
    aabb::AABB,
    interval::Interval,
    material::Material,
    vec::{Color, Point, Vec3},
};

/// Ray in 3d space. A ray has an origin point `Point` and a direction `Vec3`.
#[derive(Clone, Debug)]
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

pub struct Hit {
    pub p: Point,
    pub normal: Vec3,
    pub uv: (f64, f64),
    pub front_face: bool,
    pub t: f64,
    pub mat: Arc<dyn Material>,
}

pub trait Hittable<R: Rng>: Debug + Send + Sync {
    fn hit(&self, ray: &Ray3, t_range: &mut Interval, rng: &mut R) -> Option<Hit>;
    fn bbox(&self) -> AABB;
}

pub struct Hittables<R: Rng> {
    pub objects: Vec<Arc<dyn Hittable<R>>>,
    bbox: AABB,
}

impl<R: Rng> std::fmt::Debug for Hittables<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hittables")
            .field("objects", &self.objects)
            .field("bbox", &self.bbox)
            .finish()
    }
}

impl<R: Rng> Hittables<R> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AABB::empty(),
        }
    }

    pub fn from_vec(objects: Vec<Arc<dyn Hittable<R>>>) -> Self {
        let mut bbox = AABB::empty();
        for obj in objects.iter() {
            bbox = AABB::from_bboxes(&bbox, &obj.bbox());
        }
        Self { objects, bbox }
    }

    pub fn add(&mut self, obj: Arc<dyn Hittable<R>>) {
        if self.objects.is_empty() {
            self.bbox = obj.bbox();
        } else {
            self.bbox = AABB::from_bboxes(&self.bbox, &obj.bbox());
        }
        self.objects.push(obj);
    }

    pub fn extend(&mut self, hittables: Hittables<R>) {
        self.bbox = AABB::from_bboxes(&self.bbox, &hittables.bbox());
        self.objects.extend(hittables.objects);
    }
}

impl<R: Rng> Hittable<R> for Hittables<R> {
    fn hit(&self, ray: &Ray3, t_range: &mut Interval, rng: &mut R) -> Option<Hit> {
        let mut closest_hit: Option<Hit> = None;

        for hittable in self.objects.iter() {
            if let Some(hit) = hittable.hit(ray, t_range, rng) {
                t_range.max = hit.t;
                closest_hit = Some(hit);
            }
        }
        closest_hit
    }

    fn bbox(&self) -> AABB {
        self.bbox
    }
}

pub struct Scatter {
    pub attenuation: Color,
    pub scattered_ray: Ray3,
}
