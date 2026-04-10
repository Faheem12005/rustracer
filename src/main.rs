use crate::camera::Camera;
use crate::color::*;
use crate::hittable::{HittableList};
use crate::ray::*;
use crate::sphere::Sphere;
use crate::vec::*;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
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
mod profiler;

use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::profiler::RenderStats;

fn main() {
    let aspect_ratio =  16.0 / 9.0;
    let image_height: u32 = 400;
    
    let cam = Camera::new(aspect_ratio, image_height, 750,
                          Point3::new(-2.0, 2.0, 1.0),
                          Point3::new(0.0, 0.0, -1.0),
                          Vec3::new(0.0, 1.0, 0.0),
                          20.0,
                          10.0,
                          3.4
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

    let mut profiler = RenderStats {
        rays_cast: AtomicUsize::new(0),
        intersection_tests: AtomicUsize::new(0),
        bounces: AtomicUsize::new(0),
        rays_missed: AtomicUsize::new(0),

    };
    cam.render(&world, &mut profiler);
    let duration = start.elapsed();
    eprintln!("Runtime: {}", duration.as_millis());
    profiler.report();
}
