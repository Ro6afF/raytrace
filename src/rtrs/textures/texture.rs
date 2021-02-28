use super::super::Color;
use super::super::Point;

pub trait Texture: std::fmt::Debug + Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color;
}
