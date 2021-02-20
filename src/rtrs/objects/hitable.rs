use super::super::Ray;
use super::HitRecord;

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}
