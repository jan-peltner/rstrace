use std::rc::Rc;

use crate::{
    aabb::AABB,
    material::Material,
    ray::{Hit, Hittable, Hittables, Ray3},
    utils::Interval,
    vec::{Point, Vec3},
};

#[derive(Debug)]
pub struct Quad {
    q: Point,
    v: Vec3,
    u: Vec3,
    n: Vec3,
    w: Vec3,
    d: f64,
    mat: Rc<dyn Material>,
    bbox: AABB,
}

impl Quad {
    pub fn new(q: Point, v: Vec3, u: Vec3, mat: Rc<dyn Material>) -> Self {
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

    pub fn new_rc(q: Point, v: Vec3, u: Vec3, mat: Rc<dyn Material>) -> Rc<Self> {
        Rc::from(Self::new(q, v, u, mat))
    }

    fn aabb(q: Point, v: Vec3, u: Vec3) -> AABB {
        let box_q_qvu = AABB::from_points(&q, &(q + v + u));
        let box_qu_qv = AABB::from_points(&(q + u), &(q + v));
        // add some padding to the aabb in case the two-dimensional quad lies in XY, YZ, or XZ
        // plane
        AABB::from_bboxes(&box_q_qvu, &box_qu_qv).maybe_pad()
    }

    pub fn spawn_box(a: Point, b: Point, mat: Rc<dyn Material>) -> Hittables {
        let mut quads = Hittables::new();

        let min = Point {
            x: a.x.min(b.x),
            y: a.y.min(b.y),
            z: a.z.min(b.z),
        };

        let max = Point {
            x: a.x.max(b.x),
            y: a.y.max(b.y),
            z: a.z.max(b.z),
        };

        let dx = Vec3 {
            x: max.x - min.x,
            y: 0.0,
            z: 0.0,
        };
        let dy = Vec3 {
            x: 0.0,
            y: max.y - min.y,
            z: 0.0,
        };
        let dz = Vec3 {
            x: 0.0,
            y: 0.0,
            z: max.z - min.z,
        };

        quads.add(Quad::new_rc(min + dz, dy, dx, mat.clone())); // front face
        quads.add(Quad::new_rc(min + dz + dx, dy, dz, mat.clone())); // right face
        quads.add(Quad::new_rc(min, dy, dx, mat.clone())); // back face
        quads.add(Quad::new_rc(min, dy, dz, mat.clone())); // left face
        quads.add(Quad::new_rc(min + dy, dz, dx, mat.clone())); // top face
        quads.add(Quad::new_rc(min, dz, dx, mat.clone())); // bottom face

        quads
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray3, t_range: &mut Interval) -> Option<Hit> {
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

        let unit_interval = Interval::unit();
        // check if hit occurs outside the quadrilateral
        if !unit_interval.contains(alpha) || !unit_interval.contains(beta) {
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
