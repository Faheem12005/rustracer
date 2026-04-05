use rand::{Rng, RngExt};
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

pub struct Dielectric {
    pub refractive_index: f64
}

impl Dielectric {
    fn double_reflectance(cosine: f64, refraction_index: f64) -> f64 {
        //schlicks approximation
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut dyn Rng) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refractive_index
        }
        else {
            self.refractive_index
        };
        let unit_direction: Vec3 = r_in.direction().unit_vector();
        let cos_theta = -unit_direction.dot_product(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;
        let direction: Vec3;
        if cannot_refract || Dielectric::double_reflectance(cos_theta, ri) > rng.random() {
            direction = Vec3::reflect(unit_direction, rec.normal);
        }
        else {
            direction = Vec3::refract(unit_direction, rec.normal, ri);
        }
        let scattered = Ray::new(rec.p, direction);
        Some((attenuation, scattered))
    }
}