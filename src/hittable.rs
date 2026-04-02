use crate::ray::*;
use crate::vec::*;
use std::sync::Arc;
use crate::interval::Interval;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        r: &Ray,
        p: Point3,
        outward_normal: Vec3,
        t: f64,
    ) -> Self {
        let front_face = r.direction().dot_product(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self { p, normal, t, front_face }
    }
}

pub trait Hittable {
    fn hit(
        &self,
        r: &Ray,
        interval: Interval
    ) -> Option<HitRecord>;
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
    pub fn new() -> Self {
        Self { objects: vec![] }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, interval: Interval) -> Option<HitRecord> {
        let mut closest_so_far = interval.max;
        let mut result: Option<HitRecord> = None;
        for object in &self.objects {
            if let Some(rec) = object.hit(r, Interval::new(interval.min, closest_so_far)) {
                closest_so_far = rec.t;
                result = Some(rec);
            }
        }
        result
    }
}