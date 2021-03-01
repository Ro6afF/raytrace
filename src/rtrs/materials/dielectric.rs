use super::super::objects::HitRecord;
use super::super::textures::Texture;
use super::super::Color;
use super::super::Point;
use super::super::Ray;
use super::Material;
use std::sync::Arc;

#[derive(Debug)]
pub struct Dielectric {
    pub refraction_index: f64,
    pub albedo: Arc<dyn Texture>,
}

impl Dielectric {
    pub fn new(refraction_index: f64, albedo: Arc<dyn Texture>) -> Dielectric {
        Dielectric {
            refraction_index: refraction_index,
            albedo: albedo,
        }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let v = self.albedo.value(record.u, record.v, &record.p);

        attenuation.r = v.r;
        attenuation.g = v.g;
        attenuation.b = v.b;

        let refraction_ratio = if record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction.normailzed();

        let mut cos_theta = -unit_direction * record.normal;
        if cos_theta > 1.0 {
            cos_theta = 1.0;
        }
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let mut r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        r0 *= r0;
        let reflectance = r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5);

        let direction;
        if refraction_ratio * sin_theta > 1.0 || reflectance > fastrand::f64() {
            direction = unit_direction - (2.0 * unit_direction * record.normal) * record.normal;
        } else {
            let prep = refraction_ratio * (unit_direction + cos_theta * record.normal);
            let parallel = -((1.0 - prep * prep).abs().sqrt()) * record.normal;
            direction = prep + parallel;
        }

        scattered.origin = record.p;
        scattered.direction = direction;
        scattered.time = ray_in.time;

        true
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Point) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}
