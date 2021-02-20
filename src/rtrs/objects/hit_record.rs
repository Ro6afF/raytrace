use super::super::Point;
use super::super::Ray;
use super::super::Vector;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vector,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn blank() -> HitRecord {
        HitRecord {
            p: Point::new(0.0, 0.0, 0.0),
            normal: Vector::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector) {
        self.front_face = (ray.direction * *outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        };
    }
}
