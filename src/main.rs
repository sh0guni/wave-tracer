mod color;
mod ray;
mod vec3;
use crate::color::write_color;
use crate::color::Color;
use crate::ray::Ray;
use std::io::{self, Write};

// Returns a simple gradient as background
fn ray_color(r: &Ray) -> Color {
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stdout().flush().unwrap();
        for i in 0..image_width {
            let color = Color {
                r: (i as f64) / (image_width - 1) as f64,
                g: (j as f64) / (image_height - 1) as f64,
                b: 0.25,
            };

            write_color(color);
        }
    }
    eprintln!("Done.");
}
