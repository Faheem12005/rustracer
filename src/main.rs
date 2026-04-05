
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
mod material;

use rand::Rng;
use crate::material::{Dielectric, Lambertian, Material, Metal};

pub fn ray_color(r: &Ray, world: &HittableList, rng: &mut impl Rng, depth: u32) -> Color {
    if depth <= 0 { return Color::new(0.0, 0.0, 0.0) }
    if let Some(rec) = world.hit(r, Interval::new(0.001, INFINITY)) {
        if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec, rng) {
            return attenuation * ray_color(&scattered, world, rng, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction[1] + 1.0);

    (Vec3::new(1.0, 1.0, 1.0) * (1.0 - a))
        + (Vec3::new(0.5, 0.7, 1.0) * a)
}
fn main() {
    let aspect_ratio =  16.0 / 9.0;
    let image_height: u32 = 400;
    
    let cam = Camera::new(aspect_ratio, image_height, 750,
                          Point3::new(-2.0, 2.0, 1.0),
                          Point3::new(0.0, 0.0, -1.0),
                          Vec3::new(0.0, 1.0, 0.0),
                          20.0
    );
    let mut world: HittableList = HittableList::new();
    let material_ground: Arc<dyn Material> =
        Arc::new(Lambertian { albedo: Color::new(0.8, 0.8, 0.0) });

    let material_center: Arc<dyn Material> =
        Arc::new(Lambertian { albedo: Color::new(0.1, 0.2, 0.5) });

    let material_left: Arc<dyn Material> =
        Arc::new(Dielectric { refractive_index: 1.5 });

    let material_bubble: Arc<dyn Material> =
        Arc::new( Dielectric { refractive_index: 1.0 / 1.5 });
    let material_right: Arc<dyn Material> =
        Arc::new(Metal { albedo: Color::new(0.8, 0.6, 0.2), fuzz: 0.1 });

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble.clone()
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right.clone(),
    )));

    let start = Instant::now();
    cam.render(&world);
    let duration = start.elapsed();
    eprintln!("Runtime: {}", duration.as_millis());
}
