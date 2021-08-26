use crate::rtrs::objects::Aabb;
use crate::HitRecord;
use crate::Hitable;
use crate::Material;
use crate::Point;
use crate::Ray;
use crate::EPSILON;
use std::sync::Arc;

#[derive(Debug)]
pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
    pub material: Arc<dyn Material>,
}

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point, material: Arc<dyn Material>) -> Self {
        Self {
            p1: p1,
            p2: p2,
            p3: p3,
            material: material,
        }
    }
}

impl Hitable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let e1 = self.p2 - self.p1;
        let e2 = self.p3 - self.p1;
        let h = ray.direction ^ e2;
        let a = e1 * h;
        if a.abs() < EPSILON {
            return false;
        }
        let s = ray.origin - self.p1;
        let u = (s * h) / a;
        if u < 0.0 || u > 1.0 {
            return false;
        }

        let q = s ^ e1;
        let v = (ray.direction * q) / a;
        if v < 0.0 || u + v > 1.0 {
            return false;
        }

        let t = e2 * q / a;
        if t < t_min || t > t_max {
            return false;
        }

        let normal = (e1 ^ e2).normailzed();

        record.t = t;
        record.p = ray.at(t);
        record.u = u;
        record.v = v;
        record.material = Some(self.material.clone());
        record.set_face_normal(ray, &normal);
        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        let mut x = self.p1.x;
        if self.p2.x < x {
            x = self.p2.x;
        }
        if self.p3.x < x {
            x = self.p3.x;
        }
        let mut y = self.p1.y;
        if self.p2.y < y {
            y = self.p2.y;
        }
        if self.p3.y < y {
            y = self.p3.y;
        }
        let mut z = self.p1.z;
        if self.p2.z < z {
            z = self.p2.z;
        }
        if self.p3.z < z {
            z = self.p3.z;
        }
        output_box.min = Point::new(x - EPSILON, y - EPSILON, z - EPSILON);

        x = self.p1.x;
        if self.p2.x > x {
            x = self.p2.x;
        }
        if self.p3.x > x {
            x = self.p3.x;
        }
        y = self.p1.y;
        if self.p2.y > y {
            y = self.p2.y;
        }
        if self.p3.y > y {
            y = self.p3.y;
        }
        z = self.p1.z;
        if self.p2.z > z {
            z = self.p2.z;
        }
        if self.p3.z > z {
            z = self.p3.z;
        }
        output_box.max = Point::new(x + EPSILON, y + EPSILON, z + EPSILON);
        true
    }
}
