use super::super::Color;
use super::super::Point;
use super::Texture;

pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(c: Color) -> SolidColor {
        SolidColor { color: c }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point) -> Color {
        self.color
    }
}
