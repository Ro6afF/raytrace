use super::Material;
use crate::rtrs::textures::Texture;
use crate::Color;
use crate::HitRecord;
use crate::Point;
use crate::Ray;
use crate::Vector;
use std::sync::Arc;

#[derive(Debug)]
pub struct Metal {
    pub albedo: Arc<dyn Texture>,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Arc<dyn Texture>, fuzz: f64) -> Self {
        Self {
            albedo: albedo,
            fuzz: fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let norm_in = ray_in.direction.normailzed();
        let reflected = norm_in - 2.0 * (norm_in * record.normal) * record.normal
            + self.fuzz * Vector::random_in_unit_sphere();

        scattered.origin = record.p;
        scattered.direction.x = reflected.x;
        scattered.direction.y = reflected.y;
        scattered.direction.z = reflected.z;
        scattered.time = ray_in.time;

        let v = self.albedo.value(record.u, record.v, &record.p);

        attenuation.r = v.r;
        attenuation.g = v.g;
        attenuation.b = v.b;

        scattered.direction * record.normal > 0.0
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Point) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}
