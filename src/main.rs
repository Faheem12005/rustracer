
use crate::vec::*;
use crate::ray::*;
use crate::color::*;
use crate::hittable::{Hittable, HittableList};
use crate::sphere::Sphere;
use std::sync::Arc;
use crate::camera::Camera;
use crate::interval::Interval;
use crate::utils::INFINITY;
use std::time::Instant;

mod vec;
mod color;
mod ray;
mod hittable;
mod sphere;
mod utils;
mod interval;
mod camera;
use rand::Rng;

pub fn ray_color(r: &Ray, world: &HittableList, rng: &mut impl Rng, depth: u32) -> Color {
    if(depth <= 0) { return Color::new(0.0, 0.0, 0.0) }
    if let Some(rec) = world.hit(r, Interval::new(0.0, INFINITY)) {
        let direction = Vec3::random_on_hemisphere(&rec.normal, rng);
        // return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
        return ray_color(&Ray::new(rec.p, direction), world, rng, depth - 1) * 0.5;
    }

    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction[1] + 1.0);

    (Vec3::new(1.0, 1.0, 1.0) * (1.0 - a))
        + (Vec3::new(0.5, 0.7, 1.0) * a)
}
fn main() {
    let aspect_ratio =  16.0 / 9.0;
    let image_height: u32 = 400;
    
    let cam = Camera::new(aspect_ratio, image_height, 100);
    let mut world: HittableList = HittableList::new();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    let start = Instant::now();
    cam.render(&world);
    let duration = start.elapsed();
    eprintln!("Runtime: {}", duration.as_millis());
}
