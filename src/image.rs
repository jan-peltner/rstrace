use crate::v3::*;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec<Pixel>>,
}

impl Image {
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
                write!(
                    f,
                    "{} {} {} ",
                    pixel.x as u64, pixel.y as u64, pixel.z as u64
                )?
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
