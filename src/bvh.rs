use crate::{
    aabb::AABB,
    ray::{Hit, Hittable, Ray3},
    utils::Interval,
};

pub struct BvhNode {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    bbox: AABB,
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray3, t_range: &mut Interval) -> Option<Hit> {
        todo!()
    }

    fn bbox(&self) -> AABB {
        self.bbox
    }
}
