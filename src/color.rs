use crate::interval::Interval;
use crate::vec::Vec3;

pub type Color = Vec3;

pub fn write_color(color: &Color) {
    let r = color[0].max(0.0).sqrt();
    let g = color[1].max(0.0).sqrt();
    let b = color[2].max(0.0).sqrt();

    const INTENSITY: Interval = Interval::new(0.0, 0.999);
    let rbyte = (256.0 * INTENSITY.clamp(r)) as i32;
    let gbyte = (256.0 * INTENSITY.clamp(g)) as i32;
    let bbyte = (256.0 * INTENSITY.clamp(b)) as i32;

    println!("{} {} {}", rbyte, gbyte, bbyte);
}