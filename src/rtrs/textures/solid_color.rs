use crate::rtrs::textures::Texture;
use crate::Color;
use crate::Point;

#[derive(Debug)]
pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(c: Color) -> Self {
        Self { color: c }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point) -> Color {
        self.color
    }
}
