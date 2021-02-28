use super::super::materials::Material;
use super::super::Point;
use super::super::Ray;
use super::super::Vector;
use std::sync::Arc;

#[derive(Debug)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vector,
    pub t: f64,
    pub front_face: bool,
    pub material: Option<Arc<dyn Material>>,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn blank() -> HitRecord {
        HitRecord {
            p: Point::new(0.0, 0.0, 0.0),
            normal: Vector::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            material: None,
            u: 0.0,
            v: 0.0,
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
