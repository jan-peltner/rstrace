use crate::vec::*;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec<Pixel>>,
}

impl Image {
    pub const MAX_RGB: f64 = 255.99;

    pub fn compute_height(w: u32, ar: f64) -> u32 {
        (if w as f64 / ar < 1.0 {
            1.0
        } else {
            w as f64 / ar
        }) as u32
    }

    pub fn new(w: u32, h: u32, pxfn: impl Fn(u32, u32) -> Pixel) -> Self {
        let mut pixels: Vec<Vec<Pixel>> = Vec::with_capacity(h as usize);
        for y in 0..h {
            let mut row: Vec<Pixel> = Vec::with_capacity(w as usize);
            for x in 0..w {
                row.push(pxfn(x, y));
            }
            pixels.push(row);
        }
        Image {
            width: w,
            height: h,
            pixels,
        }
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "P3")?;
        writeln!(f, "{} {}", self.width, self.height)?;
        writeln!(f, "255")?;

        for row in &self.pixels {
            for pixel in row {
                write!(f, "{} {} {} ", pixel.x as u8, pixel.y as u8, pixel.z as u8)?
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
