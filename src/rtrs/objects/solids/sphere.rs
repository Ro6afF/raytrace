use crate::rtrs::objects::Aabb;
use crate::HitRecord;
use crate::Hitable;
use crate::Material;
use crate::Point;
use crate::Ray;
use crate::Vector;
use std::sync::Arc;

#[derive(Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center: center,
            radius: radius,
            material: material,
        }
    }

    fn get_uv(&self, p: &Point, u: &mut f64, v: &mut f64) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + std::f64::consts::PI;

        *u = phi / (2.0 * std::f64::consts::PI);
        *v = theta / std::f64::consts::PI;
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction * ray.direction;
        let b = oc * ray.direction;
        let c = oc * oc - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant < 0.0 {
            return false;
        }
        let discriminant_sqrt = discriminant.sqrt();

        let mut root = (-b - discriminant_sqrt) / a;

        if root < t_min || root > t_max {
            root = (-b + discriminant_sqrt) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        record.t = root;
        record.p = ray.at(root);
        let outward_normal = (record.p - self.center) / self.radius;
        record.set_face_normal(ray, &outward_normal);
        record.material = Some(self.material.clone());
        self.get_uv(&outward_normal, &mut record.u, &mut record.v);

        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        output_box.min = self.center - Vector::new(self.radius, self.radius, self.radius);
        output_box.max = self.center + Vector::new(self.radius, self.radius, self.radius);
        true
    }
}
