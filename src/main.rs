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
use crate::vec3::{random_in_unit_sphere, Point3, Vec3};
use rand::distributions::Standard;
use rand::{thread_rng, Rng};
use std::io::{self, Write};

fn ray_color(r: &Ray, world: &impl Hittable, depth: usize) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        return 0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1);
    }
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: f64 = 400.0;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: usize = 50;

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

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stdout().flush().unwrap();
        for i in 0..IMAGE_WIDTH as u32 {
            let pixel_color = thread_rng()
                .sample_iter::<(f64, f64), &Standard>(&Standard)
                .take(SAMPLES_PER_PIXEL)
                .map(|(ir, ij)| {
                    let u = (i as f64 + ir) / (IMAGE_WIDTH - 1.0);
                    let v = (j as f64 + ij) / (IMAGE_HEIGHT as f64 - 1.0);
                    let r = cam.get_ray(u, v);
                    ray_color(&r, &world, MAX_DEPTH)
                })
                .fold(Color::new(0.0, 0.0, 0.0), |acc, c| acc + c);

            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("Done.");
}
