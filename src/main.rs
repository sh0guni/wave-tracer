mod camera;
mod color;
mod diffusion;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod util;
mod vec3;
use crate::camera::Camera;
use crate::color::write_color;
use crate::color::Color;
use crate::diffusion::{random_in_hemisphere, random_in_unit_sphere, random_unit_vector};
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};
use rand::distributions::Standard;
use rand::{thread_rng, Rng};
use std::f64::consts::PI;
use std::io::{self, Write};
use std::rc::Rc;

fn ray_color(r: &Ray, world: &impl Hittable, depth: usize) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some(scatter) = rec.material.scatter(r, &rec) {
            return scatter.attenuation * ray_color(&scatter.scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
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

    let material_ground = Rc::new(Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    });
    let material_center = Rc::new(Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    });
    let material_left: Rc<dyn Material> = Rc::new(Dielectric { ir: 1.5 });
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let world = HittableList {
        objects: vec![
            Box::new(Sphere {
                center: Point3::new(0.0, -100.5, -1.0),
                radius: 100.0,
                material: material_ground,
            }),
            Box::new(Sphere {
                center: Point3::new(0.0, 0.0, -1.0),
                radius: 0.5,
                material: material_center,
            }),
            Box::new(Sphere {
                center: Point3::new(-1.0, 0.0, -1.0),
                radius: 0.5,
                material: Rc::clone(&material_left),
            }),
            Box::new(Sphere {
                center: Point3::new(-1.0, 0.0, -1.0),
                radius: -0.4,
                material: material_left,
            }),
            Box::new(Sphere {
                center: Point3::new(1.0, 0.0, -1.0),
                radius: 0.5,
                material: material_right,
            }),
        ],
    };

    // Camera
    let cam = Camera::new(
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
    );

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
