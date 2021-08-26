use crate::rtrs::objects::Aabb;
use crate::HitRecord;
use crate::Hitable;
use crate::Point;
use crate::Ray;
use std::f64::consts::PI;
use std::sync::Arc;

#[derive(Debug)]
pub struct Rotation {
    pub obj: Arc<dyn Hitable>,
    pub angle_x: f64,
    pub angle_y: f64,
    pub angle_z: f64,
}

impl Rotation {
    pub fn new_rad(obj: Arc<dyn Hitable>, angle_x: f64, angle_y: f64, angle_z: f64) -> Rotation {
        Rotation {
            obj: obj,
            angle_x: angle_x,
            angle_y: angle_y,
            angle_z: angle_z,
        }
    }

    pub fn new_deg(obj: Arc<dyn Hitable>, angle_x: f64, angle_y: f64, angle_z: f64) -> Rotation {
        Rotation::new_rad(
            obj,
            PI * angle_x / 180.0,
            PI * angle_y / 180.0,
            PI * angle_z / 180.0,
        )
    }
}

impl Hitable for Rotation {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut origin = ray.origin;
        let mut direction = ray.direction;

        // x axis rotation
        origin.y = origin.y * self.angle_x.cos() - origin.z * self.angle_x.sin();
        origin.z = origin.y * self.angle_x.sin() + origin.z * self.angle_x.cos();

        direction.y = direction.y * self.angle_x.cos() - direction.z * self.angle_x.sin();
        direction.z = direction.y * self.angle_x.sin() + direction.z * self.angle_x.cos();

        // y axis rotation
        origin.x = origin.x * self.angle_y.cos() + origin.z * self.angle_y.sin();
        origin.z = -origin.x * self.angle_y.sin() + origin.z * self.angle_y.cos();

        direction.x = direction.x * self.angle_y.cos() + direction.z * self.angle_y.sin();
        direction.z = -direction.x * self.angle_y.sin() + direction.z * self.angle_y.cos();

        // z axis rotation
        origin.x = origin.x * self.angle_z.cos() - origin.y * self.angle_z.sin();
        origin.y = origin.x * self.angle_z.sin() + origin.y * self.angle_z.cos();

        direction.x = direction.x * self.angle_z.cos() - direction.y * self.angle_z.sin();
        direction.y = direction.x * self.angle_z.sin() + direction.y * self.angle_z.cos();

        let rotated = Ray::new(origin, direction, ray.time);

        if !self.obj.hit(&rotated, t_min, t_max, record) {
            return false;
        }

        let mut p = record.p;
        let mut normal = record.normal;

        // x axis rotation
        p.y = p.y * self.angle_x.cos() + p.z * self.angle_x.sin();
        p.z = -p.y * self.angle_x.sin() + p.z * self.angle_x.cos();

        normal.y = normal.y * self.angle_x.cos() + normal.z * self.angle_x.sin();
        normal.z = -normal.y * self.angle_x.sin() + normal.z * self.angle_x.cos();

        // y axis rotation
        p.x = p.x * self.angle_y.cos() + p.z * self.angle_y.sin();
        p.z = -p.x * self.angle_y.sin() + p.z * self.angle_y.cos();

        normal.x = normal.x * self.angle_y.cos() + normal.z * self.angle_y.sin();
        normal.z = -normal.x * self.angle_y.sin() + normal.z * self.angle_y.cos();

        // z axis rotation
        p.x = p.x * self.angle_z.cos() + p.y * self.angle_z.sin();
        p.y = -p.x * self.angle_z.sin() + p.y * self.angle_z.cos();

        normal.x = normal.x * self.angle_z.cos() + normal.y * self.angle_z.sin();
        normal.y = -normal.x * self.angle_z.sin() + normal.y * self.angle_z.cos();

        record.p = p;
        record.normal = normal;
        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        // TODO
        let min = Point::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let max = Point::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

        output_box.min = min;
        output_box.max = max;
        true
    }
}
