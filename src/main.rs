mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod util;
mod vec3;
use crate::camera::Camera;
use crate::color::write_color;
use crate::color::Color;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};
use rand::distributions::Standard;
use rand::{thread_rng, Rng};
use std::io::{self, Write};

fn ray_color(r: &Ray, world: &impl Hittable) -> Color {
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400.0;
    let image_height = (image_width / aspect_ratio) as u32;
    let samples_per_pixel = 100;

    // World
    let world = HittableList {
        objects: vec![
            Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
            Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
        ],
    };

    // Camera
    let cam = Camera::new();

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stdout().flush().unwrap();
        for i in 0..image_width as u32 {
            let pixel_color = thread_rng()
                .sample_iter::<(f64, f64), &Standard>(&Standard)
                .take(samples_per_pixel)
                .map(|(ir, ij)| {
                    let u = (i as f64 + ir) / (image_width - 1.0);
                    let v = (j as f64 + ij) / (image_height as f64 - 1.0);
                    let r = cam.get_ray(u, v);
                    ray_color(&r, &world)
                })
                .fold(Color::new(0.0, 0.0, 0.0), |acc, c| acc + c);

            write_color(pixel_color, samples_per_pixel);
        }
    }
    eprintln!("Done.");
}
