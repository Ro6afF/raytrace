use super::super::Color;
use super::super::Point;
use super::SolidColor;
use super::Texture;

pub struct CheckerTexture {
    odd: SolidColor,
    even: SolidColor,
}

impl CheckerTexture {
    pub fn new(even: SolidColor, odd: SolidColor) -> CheckerTexture {
        CheckerTexture {
            even: even,
            odd: odd,
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color {
        let sin = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sin < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
