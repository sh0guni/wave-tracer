mod color;
mod vec3;
use crate::color::write_color;
use crate::color::Color;
use std::io::{self, Write};

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
