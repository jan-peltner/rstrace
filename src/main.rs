use std::fmt::Display;

struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Vec<Pixel>>,
}

impl Image {
    fn new(w: u32, h: u32, pxfn: impl Fn(u32, u32) -> Pixel) -> Self {
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

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "P3")?;
        writeln!(f, "{} {}", self.width, self.height)?;
        writeln!(f, "255")?;

        for row in &self.pixels {
            for pixel in row {
                write!(f, "{} {} {} ", pixel.r, pixel.g, pixel.b)?
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn main() {
    let image = Image::new(12, 12, |x, y| {
        return Pixel {
            r: (x * 255 / 12) as u8,
            g: (y * 255 / 12) as u8,
            b: 0,
        };
    });
    println!("{}", image);
}
