use rand::{Rng, RngExt};
use crate::color::{write_color, Color};
use crate::hittable::HittableList;
use crate::ray::{Point3, Ray};
use crate::ray_color;
use crate::vec::Vec3;

pub struct Camera {
    aspect_ratio: f64,
    image_height: u32,
    image_width: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: u32,
    pixel_sample_scale: f64,
    max_depth: u32,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_height: u32, samples_per_pixel: u32) -> Self {
        let mut cam = Self {
            aspect_ratio,
            image_height,
            samples_per_pixel,
            image_width: 0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            pixel_sample_scale: 1.0,
            max_depth: 50
        };

        cam.initialize();
        cam
    }
    pub fn render(&self, world: &HittableList) {
        println!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        let mut rng = rand::rng();
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut color = Color::new(0.0, 0.0, 0.0);
                for _s in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j, &mut rng);
                    color += ray_color(&ray, world, &mut rng, self.max_depth);
                }
                write_color(&(color * self.pixel_sample_scale));
            }
        }
    }
    pub fn initialize(&mut self) {
        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f64;
        let image_width = (self.image_height as f64 * self.aspect_ratio) as u32;
        let image_width = image_width.max(1);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width =
            viewport_height * (image_width as f64 / self.image_height as f64);

        let camera_center = Point3::new(0.0, 0.0, 0.0);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left: Point3 =
            camera_center
                - Vec3::new(0.0, 0.0, focal_length)
                - viewport_u / 2.0
                - viewport_v / 2.0;

        let pixel00_loc =
            viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        self.image_width = image_width;
        self.center = camera_center;
        self.pixel_delta_u = pixel_delta_u;
        self.pixel_delta_v = pixel_delta_v;
        self.pixel00_loc = pixel00_loc;
    }

    pub fn get_ray(&self, i: u32, j: u32, rng: &mut impl Rng) -> Ray {
        let offset = Self::sample_square(rng);
        let pixel_sample = self.pixel00_loc + (self.pixel_delta_u * (offset[0] + i as f64) ) + (self.pixel_delta_v * (offset[1] + j as f64));
        let ray_origin = self.center;
        let ray_direction = pixel_sample - self.center;
        Ray::new(ray_origin, ray_direction)
    }

    pub fn sample_square(rng: &mut impl Rng) -> Vec3 {
        Vec3::new(rng.random::<f64>() - 0.5, rng.random::<f64>() - 0.5, 0.0)
    }
}