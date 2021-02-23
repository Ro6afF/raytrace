use super::super::objects::HitRecord;
use super::super::Color;
use super::super::Ray;
use super::super::Vector;
use super::Material;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
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
        attenuation.r = self.albedo.r;
        attenuation.g = self.albedo.g;
        attenuation.b = self.albedo.b;

        scattered.direction * record.normal > 0.0
    }
}
