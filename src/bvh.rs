use std::sync::Arc;

use rand::Rng;

use crate::{
    aabb::AABB,
    interval::Interval,
    ray::{Hit, Hittable, Ray3},
};

// represents both individual nodes in the tree as well as the tree itself (root node)
// tree structure:
// non-leaf-nodes -> BvhNode
// leaf-nodes -> geometry primitives
pub struct BvhNode<R: Rng> {
    left: Arc<dyn Hittable<R>>,
    right: Arc<dyn Hittable<R>>,
    bbox: AABB,
}

impl<R: Rng> std::fmt::Debug for BvhNode<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BvhNode")
            .field("bbox", &self.bbox)
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}

impl<R: Rng> Hittable<R> for BvhNode<R> {
    fn hit(&self, ray: &Ray3, t_range: &mut Interval, rng: &mut R) -> Option<Hit> {
        if !self.bbox().hit(ray, *t_range) {
            return None;
        }

        let hit_left = self.left.hit(ray, t_range, rng);

        if let Some(hit_left) = &hit_left {
            if hit_left.t < t_range.max {
                t_range.max = hit_left.t
            }
        }

        let hit_right = self.right.hit(ray, t_range, rng);

        hit_right.or(hit_left)
    }

    fn bbox(&self) -> AABB {
        self.bbox
    }
}

impl<R: Rng + 'static> BvhNode<R> {
    pub fn from_hittables(
        hittables: &mut [Arc<dyn Hittable<R>>],
        rng: &mut R,
    ) -> Arc<dyn Hittable<R>> {
        Self::build_tree(hittables, 0, hittables.len(), rng)
    }

    fn build_tree(
        hittables: &mut [Arc<dyn Hittable<R>>],
        start: usize,
        end: usize,
        rng: &mut R,
    ) -> Arc<dyn Hittable<R>> {
        let axis_index: usize = rng.random_range(0..3);

        let (left, right) = match end - start {
            1 => {
                let obj = hittables[start].clone();
                (obj.clone(), obj)
            }
            2 => (hittables[start].clone(), hittables[start + 1].clone()),
            span => {
                hittables[start..end].sort_by(|a, b| {
                    let a_box = a.bbox();
                    let b_box = b.bbox();
                    let a_min = a_box.axis_interval(axis_index).min;
                    let b_min = b_box.axis_interval(axis_index).min;

                    a_min.partial_cmp(&b_min).unwrap()
                });

                let mid = start + span / 2;

                (
                    Self::build_tree(hittables, start, mid, rng),
                    Self::build_tree(hittables, mid, end, rng),
                )
            }
        };

        Arc::from(Self {
            bbox: AABB::from_bboxes(&left.bbox(), &right.bbox()),
            left,
            right,
        })
    }
}
