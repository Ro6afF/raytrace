use super::vector::Point;
use super::vector::Vector;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector, time: f64) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
            time: time,
        }
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}
