use crate::{
    aabb::AABB,
    material::{Lambertian, Material},
    ray::{Hit, Hittable, Ray3},
    texture::{SolidTex, Texture},
    utils::Interval,
    vec::{Color, Point, Vec3},
};

#[derive(Debug)]
pub struct Quad<M: Material> {
    q: Point,
    v: Vec3,
    u: Vec3,
    n: Vec3,
    d: f64,
    mat: M,
    bbox: AABB,
}

impl<T: Texture> Quad<Lambertian<T>> {
    fn new_lambertian(q: Point, v: Vec3, u: Vec3, tex: T) -> Self {
        // normal vector to the quad containing 2d plane -> defines our plane
        let n = u.cross(&v).norm();
        // constant D for the 2d plane equation
        let d = n.dot(&q);
        Self {
            q,
            v,
            u,
            n,
            d,
            mat: Lambertian { tex },
            bbox: Self::aabb(q, v, u),
        }
    }
}

impl Quad<Lambertian<SolidTex>> {
    pub fn lambertian_with_albedo(q: Point, v: Vec3, u: Vec3, albedo: Color) -> Self {
        Self::new_lambertian(q, v, u, SolidTex::new(albedo))
    }
}

impl<M: Material> Quad<M> {
    fn aabb(q: Point, v: Vec3, u: Vec3) -> AABB {
        let box_q_qvu = AABB::from_points(&q, &(q + v + u));
        let box_qu_qv = AABB::from_points(&(q + u), &(q + v));
        // add some padding to the aabb in case the two-dimensional quad lies in XY, YZ, or XZ
        // plane
        AABB::from_bboxes(&box_q_qvu, &box_qu_qv).maybe_pad()
    }
}

impl<M: Material> Hittable for Quad<M> {
    fn hit(&self, ray: &Ray3, t_range: &mut Interval) -> Option<Hit<'_>> {
        let denominator = ray.dir.dot(&self.n);
        // ray lies parallel to the 2d plane -> no intersection
        if denominator.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - ray.origin.dot(&self.n)) / denominator;
        if !t_range.contains(t) {
            return None;
        }
        let front_face = ray.dir.dot(&self.n) < 0.0;

        Some(Hit {
            t,
            p: ray.at(t),
            mat: &self.mat,
            front_face,
            normal: if front_face { self.n } else { self.n * -1.0 },
            uv: (0.0, 0.0),
        })
    }

    fn bbox(&self) -> AABB {
        self.bbox
    }
}
