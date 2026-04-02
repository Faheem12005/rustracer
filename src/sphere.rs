use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::*;

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Self { center, radius: radius.max(0.0) }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot_product(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None
        }
        let mut root = (h - discriminant.sqrt()) / a;
        if !interval.surrounds(root) {
            root = (h + discriminant.sqrt()) / a;
            if !interval.surrounds(root) {
                return None
            }
        }
        let outward_normal = (ray.at(root) - self.center) / self.radius;
        Some(HitRecord::new(ray, ray.at(root), outward_normal, root))
    }
}