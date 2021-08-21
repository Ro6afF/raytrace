use super::Aabb;
use super::HitRecord;
use crate::Ray;

pub trait Hitable: std::fmt::Debug + Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool;
}
