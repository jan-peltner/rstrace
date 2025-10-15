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
    mat: M,
    bbox: AABB,
}

impl<T: Texture> Quad<Lambertian<T>> {
    fn new_lambertian(q: Point, v: Vec3, u: Vec3, tex: T) -> Self {
        Self {
            q,
            v,
            u,
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
        todo!()
    }

    fn bbox(&self) -> AABB {
        self.bbox
    }
}
