use std::io::{self, Write};

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stdout().flush().unwrap();
        for i in 0..image_width {
            let r = (i as f64) / (image_width - 1) as f64;
            let g = (j as f64) / (image_height - 1) as f64;
            let b = 0.25;

            let ir = (255.999f64 * r) as u32;
            let ig = (255.999f64 * g) as u32;
            let ib = (255.999f64 * b) as u32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("Done.");
}
