use rstrace::image::*;
use rstrace::vec::*;

fn main() {
    // --- Image dimensions ---
    let img_w = 400;
    let ar = 16.0 / 9.0;
    let img_h = Image::compute_height(img_w, ar);

    // --- Render ---
    let image = Image::new(img_w, img_h, |x, y| {
        let w = img_w as f64 - 1.0;
        let h = img_h as f64 - 1.0;

        let x_norm = x as f64 / w;
        let y_norm = y as f64 / h;

        return Pixel {
            x: x_norm * Image::MAX_RGB,
            y: y_norm * Image::MAX_RGB,
            z: 0.0,
        };
    });

    println!("{}", image);
}
