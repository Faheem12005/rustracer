use crate::vec::Vec3;

pub type Color = Vec3;

pub fn write_color(color: &Color) {
    let r = color[0];
    let g = color[1];
    let b = color[2];

    let rbyte = (255.999 * r) as i32;
    let gbyte = (255.999 * g) as i32;
    let bbyte = (255.999 * b) as i32;

    println!("{} {} {}", rbyte, gbyte, bbyte);
}