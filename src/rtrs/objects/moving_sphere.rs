use super::Aabb;
use super::Hitable;
use crate::HitRecord;
use crate::Material;
use crate::Point;
use crate::Ray;
use crate::Vector;
use std::sync::Arc;

#[derive(Debug)]
pub struct MovingSphere {
    pub center0: Point,
    pub center1: Point,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Point,
        center1: Point,
        time0: f64,
        time1: f64,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            center0: center0,
            center1: center1,
            time0: time0,
            time1: time1,
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

    pub fn center(&self, time: f64) -> Point {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center(ray.time);
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
        let outward_normal = (record.p - self.center(ray.time)) / self.radius;
        record.set_face_normal(ray, &outward_normal);
        record.material = Some(self.material.clone());
        self.get_uv(&outward_normal, &mut record.u, &mut record.v);

        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        let box0 = Aabb::new(
            self.center(time0) - Vector::new(self.radius, self.radius, self.radius),
            self.center(time0) + Vector::new(self.radius, self.radius, self.radius),
        );
        let box1 = Aabb::new(
            self.center(time1) - Vector::new(self.radius, self.radius, self.radius),
            self.center(time1) + Vector::new(self.radius, self.radius, self.radius),
        );
        let sbox = Aabb::surrounding_box(&box0, &box1);

        output_box.max = sbox.max;
        output_box.min = sbox.min;

        true
    }
}
