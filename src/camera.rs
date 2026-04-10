use rand::{Rng, RngExt};
use crate::color::{write_color, Color};
use crate::hittable::{Hittable, HittableList};
use crate::interval::Interval;
use crate::ray::{Point3, Ray};
use crate::utils::{degrees_to_radians, INFINITY};
use crate::vec::Vec3;
use crate::profiler::RenderStats;

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
    pub fn new(aspect_ratio: f64, image_height: u32, samples_per_pixel: u32, lookfrom: Point3, lookat: Point3, vup: Vec3, fov: f64, defocus_angle: f64, focus_dist: f64) -> Self {
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
            defocus_angle,
            focus_dist,
            defocus_disk_u: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 0.0, 0.0),
        };

        cam.initialize();
        cam
    }
    pub fn render(&self, world: &HittableList, profiler: &mut RenderStats) {
        println!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        let mut rng = rand::rng();
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut color = Color::new(0.0, 0.0, 0.0);
                for _s in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j, &mut rng);
                    color += Camera::ray_color(&ray, world, &mut rng, self.max_depth, profiler);
                }
                write_color(&(color * self.pixel_sample_scale));
            }
        }
    }
    pub fn initialize(&mut self) {
        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f64;
        let image_width = (self.image_height as f64 * self.aspect_ratio) as u32;
        let image_width = image_width.max(1);

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

        let viewport_upper_left = camera_center - (self.w * self.focus_dist) - viewport_u/2.0 - viewport_v/2.0;

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
        let pixel_sample: Point3 = self.pixel00_loc + (self.pixel_delta_u * (offset[0] + i as f64) ) + (self.pixel_delta_v * (offset[1] + j as f64));
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample(rng)
        };
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    pub fn ray_color(r: &Ray, world: &HittableList, rng: &mut impl Rng, depth: u32, profiler: &mut RenderStats) -> Color {
        profiler.inc_rays_cast();
        if depth <= 0 { return Color::new(0.0, 0.0, 0.0) }
        if let Some(rec) = world.hit(r, Interval::new(0.001, INFINITY), profiler) {
            if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec, rng) {
                profiler.inc_bounces();
                return attenuation * Camera::ray_color(&scattered, world, rng, depth - 1, profiler);
            }
            return Color::new(0.0, 0.0, 0.0);
        }
        profiler.inc_ray_missed();
        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction[1] + 1.0);

        (Vec3::new(1.0, 1.0, 1.0) * (1.0 - a))
            + (Vec3::new(0.5, 0.7, 1.0) * a)
    }

    pub fn sample_square(rng: &mut impl Rng) -> Vec3 {
        Vec3::new(rng.random::<f64>() - 0.5, rng.random::<f64>() - 0.5, 0.0)
    }

    pub fn defocus_disk_sample(&self, rng: &mut impl Rng) -> Point3 {
        let p = Vec3::random_in_unit_disk(rng);
        self.center + self.defocus_disk_u * p[0] + self.defocus_disk_v * p[1]
    }
}