use super::super::Ray;
use super::HitRecord;
use super::Hitable;

pub struct HitableList {
    pub objects: Vec<Box<dyn Hitable + Send + Sync>>,
}

impl HitableList {
    pub fn new() -> HitableList {
        HitableList { objects: vec![] }
    }

    pub fn add(&mut self, obj: Box<dyn Hitable + Send + Sync>) {
        self.objects.push(obj);
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
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
                }
            }
        }

        return hited;
    }
}
