use crate::{utils::Interval, vec::*};

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec<Pixel>>,
}

impl Image {
    const RGB_INTERVAL: Interval = Interval {
        min: 0.0,
        max: 255.99,
    };

    pub fn map_to_rgb_space(val: f64) -> f64 {
        Self::RGB_INTERVAL.clamp(val * 255.99)
    }

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

            let scanline = y + 1;
            if scanline % 100 == 0 {
                println!("Scanlines processed: {}/{}", scanline, h);
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
