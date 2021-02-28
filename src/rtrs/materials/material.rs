use super::super::objects::HitRecord;
use super::super::Color;
use super::super::Ray;

pub trait Material: std::fmt::Debug + Send + Sync {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}
