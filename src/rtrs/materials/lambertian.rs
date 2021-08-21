use crate::rtrs::textures::Texture;
use crate::Color;
use crate::HitRecord;
use crate::Material;
use crate::Point;
use crate::Ray;
use crate::Vector;
use std::sync::Arc;

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo: albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        scattered.origin = record.p;
        scattered.direction = record.normal + Vector::random_unit();
        scattered.time = ray_in.time;

        let v = self.albedo.value(record.u, record.v, &record.p);

        attenuation.r = v.r;
        attenuation.g = v.g;
        attenuation.b = v.b;

        true
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Point) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}
