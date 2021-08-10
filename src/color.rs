#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

pub fn write_color(color: Color) {
    let ir = (255.999f64 * color.r) as u32;
    let ig = (255.999f64 * color.g) as u32;
    let ib = (255.999f64 * color.b) as u32;
    println!("{} {} {}", ir, ig, ib);
}
