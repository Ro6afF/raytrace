use super::vector::Point;
use super::vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}
