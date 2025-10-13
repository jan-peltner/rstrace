use crate::vec::{Color, Point};
use std::fmt::Debug;

pub trait Texture: Debug {
    fn value(&self, uv: (f64, f64), p: &Point) -> &Color;
}

#[derive(Debug)]
pub struct SolidTex {
    albedo: Color,
}

impl SolidTex {
    pub fn new(color: Color) -> Self {
        Self { albedo: color }
    }

    pub fn red() -> Self {
        Self {
            albedo: Color::red(),
        }
    }

    pub fn green() -> Self {
        Self {
            albedo: Color::green(),
        }
    }

    pub fn blue() -> Self {
        Self {
            albedo: Color::blue(),
        }
    }

    pub fn white() -> Self {
        Self {
            albedo: Color::white(),
        }
    }
}

impl Texture for SolidTex {
    fn value(&self, _uv: (f64, f64), _p: &Point) -> &Color {
        &self.albedo
    }
}

#[derive(Debug)]
pub struct CheckerTex {
    even: SolidTex,
    odd: SolidTex,
    inv_scale: f64,
}

impl CheckerTex {
    pub fn new(c1: Color, c2: Color, scale: f64) -> Self {
        Self {
            even: SolidTex::new(c1),
            odd: SolidTex::new(c2),
            inv_scale: 1.0 / scale,
        }
    }
}

impl Texture for CheckerTex {
    fn value(&self, uv: (f64, f64), p: &Point) -> &Color {
        let x = (p.x * self.inv_scale).floor() as i32;
        let y = (p.y * self.inv_scale).floor() as i32;
        let z = (p.z * self.inv_scale).floor() as i32;

        if (x + y + z) % 2 == 0 {
            self.even.value(uv, p)
        } else {
            self.odd.value(uv, p)
        }
    }
}

#[derive(Debug)]
pub struct StripeTex {
    even: SolidTex,
    odd: SolidTex,
    inv_scale: f64,
}

impl StripeTex {
    pub fn new(c1: Color, c2: Color, scale: f64) -> Self {
        Self {
            even: SolidTex::new(c1),
            odd: SolidTex::new(c2),
            inv_scale: 1.0 / scale,
        }
    }
}

impl Texture for StripeTex {
    fn value(&self, uv: (f64, f64), p: &Point) -> &Color {
        let x = (p.x * self.inv_scale).floor() as i32;
        if x % 2 == 0 {
            self.even.value(uv, p)
        } else {
            self.odd.value(uv, p)
        }
    }
}
