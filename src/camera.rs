use crate::color::{write_color, Color};
use crate::hittable::HittableList;
use crate::ray::{Point3, Ray};
use crate::ray_color;
use crate::vec::Vec3;

pub struct Camera {
    aspect_ratio: f64,
    image_height: u32,
    image_width: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_height: u32) -> Self {
        let mut cam = Self {
            aspect_ratio,
            image_height,
            image_width: 0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
        };

        cam.initialize();
        cam
    }
    pub fn render(&self, world: &HittableList) {
        println!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);
                let pixel_color: Color = ray_color(&ray, &world);
                write_color(&pixel_color);
            }
        }
    }
    pub fn initialize(&mut self) {
        let image_width = (self.image_height as f64 * self.aspect_ratio) as i32;
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
}