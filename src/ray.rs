use std::fmt::Debug;
use std::rc::Rc;

use crate::{
    aabb::AABB,
    material::Material,
    utils::Interval,
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

pub struct Hit<'a> {
    pub p: Point,
    pub normal: Vec3,
    pub front_face: bool,
    pub t: f64,
    pub mat: &'a dyn Material,
}

pub trait Hittable: Debug {
    fn hit(&self, ray: &Ray3, t_range: &mut Interval) -> Option<Hit>;
    fn bbox(&self) -> AABB;
}

#[derive(Debug)]
pub struct Hittables {
    pub objects: Vec<Rc<dyn Hittable>>,
    bbox: AABB,
}

impl Hittables {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AABB::empty(),
        }
    }

    pub fn from_vec(objects: Vec<Rc<dyn Hittable>>) -> Self {
        Self {
            objects,
            bbox: AABB::empty(),
        }
    }

    pub fn add(&mut self, obj: Rc<dyn Hittable>) {
        self.bbox = AABB::from_bboxes(&self.bbox, &obj.bbox());
        self.objects.push(obj);
    }
}

impl Hittable for Hittables {
    fn hit(&self, ray: &Ray3, t_range: &mut Interval) -> Option<Hit> {
        let mut closest_hit: Option<Hit> = None;

        for hittable in self.objects.iter() {
            if let Some(hit) = hittable.hit(ray, t_range) {
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

pub struct Scatter<'a> {
    pub attenuation: &'a Color,
    pub scattered_ray: Ray3,
}
