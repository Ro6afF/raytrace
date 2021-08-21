use crate::Color;
use crate::HitRecord;
use crate::Point;
use crate::Ray;

pub trait Material: std::fmt::Debug + Send + Sync {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;

    fn emitted(&self, u: f64, v: f64, p: &Point) -> Color;
}
