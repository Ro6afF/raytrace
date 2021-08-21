use crate::Color;
use crate::Point;

pub trait Texture: std::fmt::Debug + Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color;
}
