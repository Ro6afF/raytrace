use super::super::objects::HitRecord;
use super::super::textures::Texture;
use super::super::Color;
use super::super::Ray;
use super::super::Vector;
use super::Material;
use std::sync::Arc;

pub struct Lambertian {
    pub albedo: Arc<dyn Texture + Send + Sync>,
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture + Send + Sync>) -> Lambertian {
        Lambertian { albedo: albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        scattered.origin = record.p;
        scattered.direction = record.normal + Vector::random_unit();

        let v = self.albedo.value(record.u, record.v, &record.p);

        attenuation.r = v.r;
        attenuation.g = v.g;
        attenuation.b = v.b;

        true
    }
}
