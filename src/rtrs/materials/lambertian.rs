use super::super::objects::HitRecord;
use super::super::textures::Texture;
use super::super::Color;
use super::super::Ray;
use super::super::Vector;
use super::Material;
use std::sync::Arc;

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture>) -> Lambertian {
        Lambertian { albedo: albedo }
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
}
