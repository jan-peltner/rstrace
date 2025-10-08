use crate::vec::{Color, Point};

pub trait Texture {
    fn value(u: f64, v: f64, p: &Point) -> Color;
}
