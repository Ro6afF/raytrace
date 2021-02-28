use super::super::Point;
use super::super::Ray;
use super::Aabb;
use super::HitRecord;
use super::Hitable;
use std::sync::Arc;

#[derive(Debug)]
pub struct HitableList {
    pub objects: Vec<Arc<dyn Hitable>>,
}

impl HitableList {
    pub fn new() -> HitableList {
        HitableList { objects: vec![] }
    }

    pub fn add(&mut self, obj: Arc<dyn Hitable>) {
        self.objects.push(obj);
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut hited = false;
        let mut closest = t_max;
        let mut temp_record = HitRecord::blank();

        for obj in &self.objects {
            if obj.hit(ray, t_min, t_max, &mut temp_record) {
                hited = true;
                if closest > (&temp_record).t {
                    closest = (&temp_record).t;
                    record.p = temp_record.p;
                    record.front_face = temp_record.front_face;
                    record.normal = temp_record.normal;
                    record.t = temp_record.t;
                    record.material = temp_record.material.clone();
                }
            }
        }

        return hited;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        if self.objects.len() == 0 {
            return false;
        }

        let mut temp_box = Aabb::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 0.0));
        let mut first_box = true;

        for i in &self.objects {
            if !i.bounding_box(time0, time1, &mut temp_box) {
                return false;
            }

            if first_box {
                output_box.min = temp_box.min;
                output_box.max = temp_box.max;
                first_box = false;
            } else {
                let sbox = Aabb::surrounding_box(output_box, &temp_box);
                output_box.min = sbox.min;
                output_box.max = sbox.max;
            }
        }
        true
    }
}
