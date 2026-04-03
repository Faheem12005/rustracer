use rand::Rng;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec::Vec3;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut dyn Rng
    ) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    pub albedo: Color
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, rng: &mut dyn Rng) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector(rng);
        if scatter_direction.near_zero() { scatter_direction = rec.normal };
        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut dyn Rng) -> Option<(Color, Ray)> {
        let mut reflected = Vec3::reflect(r_in.direction(), rec.normal);
        reflected = reflected.unit_vector() + (Vec3::random_unit_vector(rng) * self.fuzz);
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;
        if rec.normal.dot_product(&scattered.direction()) <= 0.0 { return None };
        Some((attenuation, scattered))
    }
}