use rand::{Rng, RngExt};
use crate::color::{write_color, Color};
use crate::hittable::HittableList;
use crate::ray::{Point3, Ray};
use crate::ray_color;
use crate::utils::degrees_to_radians;
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
    fov: f64,
    lookfrom: Point3,
    lookat: Point3,
    vup: Vec3,
    //camera basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_angle: f64,
    focus_dist: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,

}

impl Camera {
    pub fn new(aspect_ratio: f64, image_height: u32, samples_per_pixel: u32, lookfrom: Point3, lookat: Point3, vup: Vec3, fov: f64) -> Self {
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
            max_depth: 50,
            fov,
            lookfrom,
            lookat,
            vup,
            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
            w: Vec3::new(0.0, 0.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: (lookfrom - lookat).length(),
            defocus_disk_u: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 0.0, 0.0),
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

        let focal_length = (self.lookfrom - self.lookat).length();
        let theta = degrees_to_radians(self.fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width =
            viewport_height * (image_width as f64 / self.image_height as f64);

        let camera_center = self.lookfrom;
        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = self.vup.cross_product(&self.w).unit_vector();
        self.v = self.w.cross_product(&self.u);

        let viewport_u = self.u * viewport_width;
        let viewport_v = -self.v * viewport_height;

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = camera_center - (self.w * focal_length) - viewport_u/2.0 - viewport_v/2.0;

        let pixel00_loc =
            viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let defocus_radius =
            self.focus_dist * (degrees_to_radians(self.defocus_angle / 2.0)).tan();

        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;

        self.image_width = image_width;
        self.center = camera_center;
        self.pixel_delta_u = pixel_delta_u;
        self.pixel_delta_v = pixel_delta_v;
        self.pixel00_loc = pixel00_loc;
    }

    pub fn get_ray(&self, i: u32, j: u32, rng: &mut impl Rng) -> Ray {
        let offset = Self::sample_square(rng);
        let pixel_sample = self.pixel00_loc + (self.pixel_delta_u * (offset[0] + i as f64) ) + (self.pixel_delta_v * (offset[1] + j as f64));
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample(rng)
        };
        let ray_direction = pixel_sample - self.center;
        Ray::new(ray_origin, ray_direction)
    }

    pub fn sample_square(rng: &mut impl Rng) -> Vec3 {
        Vec3::new(rng.random::<f64>() - 0.5, rng.random::<f64>() - 0.5, 0.0)
    }

    pub fn defocus_disk_sample(&self, rng: &mut impl Rng) -> Point3 {
        let p = Vec3::random_in_unit_disk(rng);
        self.center + self.defocus_disk_u * p[0] + self.defocus_disk_v * p[1]
    }
}