mod image;
mod ray;
mod v3;

use image::*;
use v3::*;

fn main() {
    let image = Image::new(256, 256, |x, y| {
        let x = x as f64 / (256.0 - 1.0);
        let y = y as f64 / (256.0 - 1.0);

        return Pixel {
            x: x * 255.99,
            y: y * 255.99,
            z: 0.0,
        };
    });
    println!("{}", image);
}
