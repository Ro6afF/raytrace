use crate::rtrs::objects::Aabb;
use crate::HitRecord;
use crate::Hitable;
use crate::Ray;
use crate::Vector;
use std::sync::Arc;

#[derive(Debug)]
pub struct Translation {
    pub obj: Arc<dyn Hitable>,
    pub offset: Vector,
}

impl Translation {
    pub fn new(obj: Arc<dyn Hitable>, offset: Vector) -> Translation {
        Translation {
            obj: obj,
            offset: offset,
        }
    }
}

impl Hitable for Translation {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let moved = Ray::new(ray.origin - self.offset, ray.direction, ray.time);
        if !self.obj.hit(&moved, t_min, t_max, record) {
            return false;
        }

        record.p = record.p + self.offset;
        record.set_face_normal(&moved, &record.normal.clone());

        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        if !self.obj.bounding_box(time0, time1, output_box) {
            return false;
        }

        output_box.min = output_box.min + self.offset;
        output_box.max = output_box.max + self.offset;

        true
    }
}
