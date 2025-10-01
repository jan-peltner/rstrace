use crate::{
    aabb::AABB,
    ray::{Hit, Hittable, Ray3},
    utils::Interval,
};

// represents both individual nodes in the tree as well as the tree itself (root node)
// tree structure:
// root -> BvhNode
// non-leaf-nodes -> BvhNode
// leaf-nodes -> geometry primitives
pub struct BvhNode {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    bbox: AABB,
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray3, t_range: &mut Interval) -> Option<Hit> {
        if !self.bbox().hit(ray, t_range) {
            return None;
        }

        let hit_left = self.left.hit(ray, t_range);

        if let Some(hit_left) = self.left.hit(ray, t_range) {
            if hit_left.t < t_range.max {
                t_range.max = hit_left.t
            }
        }

        let hit_right = self.right.hit(ray, t_range);

        hit_left.or(hit_right).or(None)
    }

    fn bbox(&self) -> AABB {
        self.bbox
    }
}

impl BvhNode {
    fn build_tree() -> Self {
        todo!()
    }
}
