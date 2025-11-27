use std::sync::Arc;

use rand::Rng;

use crate::{
    aabb::AABB,
    interval::Interval,
    material::Material,
    ray::{Hit, Hittable, Ray3},
    vec::{Point, Vec3},
};

#[derive(Debug)]
pub struct Triangle {
    q: Point,
    v: Vec3,
    u: Vec3,
    n: Vec3,
    w: Vec3,
    d: f64,
    mat: Arc<dyn Material>,
    bbox: AABB,
}

impl Triangle {
    pub fn new(q: Point, v: Vec3, u: Vec3, mat: Arc<dyn Material>) -> Self {
        // normal vector to the quad containing 2d plane -> defines our plane
        let n = u.cross(&v);
        let n_norm = n.norm();
        // constant D for the 2d plane equation
        let d = n_norm.dot(&q);
        // cache w for computing the planar coordinates
        let w = n / n.dot(&n);
        Self {
            q,
            v,
            u,
            n: n_norm,
            w,
            d,
            mat,
            bbox: Self::aabb(q, v, u),
        }
    }

    pub fn new_arc(q: Point, v: Vec3, u: Vec3, mat: Arc<dyn Material>) -> Arc<Self> {
        Arc::from(Self::new(q, v, u, mat))
    }

    fn aabb(q: Point, v: Vec3, u: Vec3) -> AABB {
        let box_q_qvu = AABB::from_points(&q, &(q + v + u));
        let box_qu_qv = AABB::from_points(&(q + u), &(q + v));
        // add some padding to the aabb in case the two-dimensional quad lies in XY, YZ, or XZ
        // plane
        AABB::from_bboxes(&box_q_qvu, &box_qu_qv).maybe_pad()
    }
}

impl<R: Rng> Hittable<R> for Triangle {
    fn hit(&self, ray: &Ray3, t_range: &mut Interval, _rng: &mut R) -> Option<Hit> {
        let denominator = ray.dir.dot(&self.n);
        // ray lies parallel to the 2d plane -> no intersection
        if denominator.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - ray.origin.dot(&self.n)) / denominator;
        if !t_range.contains(t) {
            return None;
        }
        let p = ray.at(t);
        // vector from quad origin Q to intersection point P
        let qp = p - self.q;
        let alpha = self.w.dot(&qp.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&qp));

        if alpha < 0.0 || beta < 0.0 || alpha + beta > 1.0 {
            return None;
        }

        let front_face = ray.dir.dot(&self.n) < 0.0;

        Some(Hit {
            t,
            p,
            mat: self.mat.clone(),
            front_face,
            normal: if front_face { self.n } else { self.n * -1.0 },
            uv: (alpha, beta),
        })
    }

    fn bbox(&self) -> AABB {
        self.bbox
    }
}
