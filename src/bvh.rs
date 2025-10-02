use std::rc::Rc;

use rand::Rng;

use crate::{
    aabb::AABB,
    ray::{Hit, Hittable, Ray3},
    utils::Interval,
};

// represents both individual nodes in the tree as well as the tree itself (root node)
// tree structure:
// non-leaf-nodes -> BvhNode
// leaf-nodes -> geometry primitives
pub struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB,
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray3, t_range: &mut Interval) -> Option<Hit> {
        if !self.bbox().hit(ray, t_range) {
            return None;
        }

        let hit_left = self.left.hit(ray, t_range);

        if let Some(hit_left) = &hit_left {
            if hit_left.t < t_range.max {
                t_range.max = hit_left.t
            }
        }

        let hit_right = self.right.hit(ray, t_range);

        hit_left.or(hit_right)
    }

    fn bbox(&self) -> AABB {
        self.bbox
    }
}

impl BvhNode {
    pub fn from_hittables<R: Rng>(
        hittables: &mut [Rc<dyn Hittable>],
        rng: &mut R,
    ) -> Rc<dyn Hittable> {
        Self::build_tree(hittables, 0, hittables.len(), rng)
    }

    fn build_tree<R: Rng>(
        hittables: &mut [Rc<dyn Hittable>],
        start: usize,
        end: usize,
        rng: &mut R,
    ) -> Rc<dyn Hittable> {
        let axis_index: usize = rng.random_range(0..3);

        let span = end - start;

        let left: Rc<dyn Hittable>;
        let right: Rc<dyn Hittable>;

        if span == 1 {
            left = hittables[start].clone();
            right = hittables[start].clone();
        } else if span == 2 {
            left = hittables[start].clone();
            right = hittables[start + 1].clone();
        } else {
            hittables[start..end].sort_by(|a, b| {
                let a_box = a.bbox();
                let b_box = b.bbox();
                let a_min = a_box.axis_interval(axis_index).min;
                let b_min = b_box.axis_interval(axis_index).min;

                a_min.partial_cmp(&b_min).unwrap()
            });

            let mid = start + span / 2;

            left = Self::build_tree(hittables, start, mid, rng);
            right = Self::build_tree(hittables, mid, end, rng);
        }

        Rc::from(Self {
            bbox: AABB::from_bboxes(&left.bbox(), &right.bbox()),
            left,
            right,
        })
    }
}
