use rand::{Rng, RngCore};

/// Three-dimensional vector that's used for points, colors, offsets etc.
#[derive(Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Pixel that's used to render an image pub type Pixel = Vec3;
pub type Pixel = Vec3;
/// Point in 3d space
pub type Point = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn zero() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn rand_unit_square_offset(rng: &mut dyn RngCore) -> Self {
        Vec3 {
            x: rng.random_range(-0.5..0.5),
            y: rng.random_range(-0.5..0.5),
            z: 0.0,
        }
    }

    pub fn rand<R: Rng>(rng: &mut R) -> Self {
        Vec3 {
            x: rng.random(),
            y: rng.random(),
            z: rng.random(),
        }
    }

    pub fn rand_range(rng: &mut dyn RngCore, min: f64, max: f64) -> Self {
        Vec3 {
            x: rng.random_range(min..max),
            y: rng.random_range(min..max),
            z: rng.random_range(min..max),
        }
    }

    pub fn rand_unit_sphere_vec(rng: &mut dyn RngCore) -> Self {
        loop {
            let v = Self::rand_range(rng, -1.0, 1.0);
            let len_sqr = v.len_sqr();
            if len_sqr <= 1.0 {
                return v / len_sqr.sqrt();
            }
        }
    }

    pub fn rand_unit_sphere_vec_on_hemisphere(rng: &mut dyn RngCore, normal: &Vec3) -> Self {
        let unit_sphere_vec = Self::rand_unit_sphere_vec(rng);
        if unit_sphere_vec.dot(normal) > 0.0 {
            unit_sphere_vec
        } else {
            unit_sphere_vec * -1.0
        }
    }

    pub fn reflect(&self, normal: &Vec3) -> Self {
        self + &((normal * self.dot(normal)) * -2.0)
    }

    pub fn refract(&self, normal: &Vec3, etai_over_etao: f64) -> Self {
        let cos_theta = (self * -1.0).dot(normal).min(1.0); // safety clamp to avoid float rounding
                                                            // above 1.0

        // component of R' that is perpendicular to surface
        let r_perp = (self + &(normal * cos_theta)) * etai_over_etao;
        // component of R' that is parallel to surface
        let r_para = normal * -(1.0 - &r_perp.len_sqr()).abs().sqrt();
        r_perp + r_para
    }

    pub fn len_sqr(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> f64 {
        self.len_sqr().sqrt()
    }

    pub fn norm(&self) -> Self {
        self / self.len()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }
}

impl Color {
    pub fn red() -> Self {
        return Color {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
    }

    pub fn green() -> Self {
        return Color {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
    }

    pub fn blue() -> Self {
        return Color {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
    }

    pub fn white() -> Self {
        return Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Mul for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl std::ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}
