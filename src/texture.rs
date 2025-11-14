use image::{ImageError, ImageReader, RgbImage};

use crate::vec::{Color, Point};
use std::{fmt::Debug, path::Path};

pub trait Texture: Debug + Send + Sync {
    fn value(&self, uv: (f64, f64), p: &Point) -> Color;
}

#[derive(Debug, Clone)]
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
    fn value(&self, _uv: (f64, f64), _p: &Point) -> Color {
        self.albedo.clone()
    }
}

#[derive(Debug, Clone)]
pub struct CheckerTex {
    even: SolidTex,
    odd: SolidTex,
    scale: f64,
}

impl CheckerTex {
    pub fn new(c1: Color, c2: Color, scale: f64) -> Self {
        Self {
            even: SolidTex::new(c1),
            odd: SolidTex::new(c2),
            scale,
        }
    }
}

impl Texture for CheckerTex {
    fn value(&self, uv: (f64, f64), p: &Point) -> Color {
        let scaled_u = (uv.0 * self.scale).floor() as i32;
        let scaled_v = (uv.1 * self.scale).floor() as i32;

        if (scaled_u + scaled_v) % 2 == 0 {
            self.even.value(uv, p)
        } else {
            self.odd.value(uv, p)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone)]
pub struct StripeTex {
    even: SolidTex,
    odd: SolidTex,
    scale: f64,
    orientation: Orientation,
}

impl StripeTex {
    pub fn new(c1: Color, c2: Color, scale: f64, orientation: Orientation) -> Self {
        Self {
            even: SolidTex::new(c1),
            odd: SolidTex::new(c2),
            scale,
            orientation,
        }
    }
}

impl Texture for StripeTex {
    fn value(&self, uv: (f64, f64), p: &Point) -> Color {
        let or = match self.orientation {
            Orientation::Vertical => (uv.0 * self.scale).floor() as i32,
            Orientation::Horizontal => (uv.1 * self.scale).floor() as i32,
        };
        if or % 2 == 0 {
            self.even.value(uv, p)
        } else {
            self.odd.value(uv, p)
        }
    }
}

#[derive(Debug, Clone)]
pub struct UVTex {
    pub color_u: SolidTex,
    pub color_v: SolidTex,
}

impl UVTex {
    pub fn new() -> Self {
        Self {
            color_u: SolidTex::red(),
            color_v: SolidTex::green(),
        }
    }
}

impl Texture for UVTex {
    fn value(&self, uv: (f64, f64), p: &Point) -> Color {
        let (u, v) = uv;

        self.color_u.value(uv, p) * u + self.color_v.value(uv, p) * v
    }
}

#[derive(Debug, Clone)]
pub struct ImageTex {
    data: RgbImage,
}

impl Texture for ImageTex {
    fn value(&self, uv: (f64, f64), _p: &Point) -> Color {
        let width = self.data.width();
        let height = self.data.height();

        let u = uv.0;
        let v = 1.0 - uv.1;

        let i = (u * (width - 1) as f64) as u32;
        let j = (v * (height - 1) as f64) as u32;

        self.data.get_pixel(i, j).into()
    }
}

impl ImageTex {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, ImageError> {
        let img = ImageReader::open(path)
            .map_err(|err| ImageError::IoError(err))?
            .decode()?
            .into_rgb8();
        Ok(Self { data: img })
    }
}
