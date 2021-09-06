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
use crate::color::get_pixel;
use crate::camera::Camera;
use crate::color::Color;
use crate::diffusion::{random_in_unit_sphere, random_unit_vector};
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};
use rand::distributions::{Distribution, Standard, Uniform};
use rand::{thread_rng, Rng};
use std::io::{self, Write};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

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

fn random_scene() -> HittableList {
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    let mut rng = thread_rng();
    let metal_between = Uniform::from(0.5..1.0);

    let ground_material = Rc::new(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });
    objects.push(Box::new(Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
    }));

    let p = Point3::new(4.0, 0.2, 0.0);

    for a in -11..11 {
        for b in -11..11 {
            let (choose_mat, x, z) = rng.gen::<(f64, f64, f64)>();
            let center = Point3::new(a as f64 + 0.9 * x, 0.2, b as f64 + 0.9 * z);

            if (center - p).length() > 0.9 {
                let sphere_material: Rc<dyn Material> = if choose_mat < 0.8 {
                    let (r1, g1, b1, r2, g2, b2) = rng.gen();
                    // diffuse
                    let albedo = Color::new(r1, g1, b1) * Color::new(r2, g2, b2);
                    Rc::new(Lambertian { albedo })
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::new(
                        metal_between.sample(&mut rng),
                        metal_between.sample(&mut rng),
                        metal_between.sample(&mut rng),
                    );
                    let fuzz = rng.gen_range(0.5..1.0);
                    Rc::new(Metal::new(albedo, fuzz))
                } else {
                    //glass
                    Rc::new(Dielectric { ir: 1.5 })
                };
                objects.push(Box::new(Sphere {
                    center,
                    radius: 0.2,
                    material: sphere_material,
                }));
            }
        }
    }

    objects.push(Box::new(Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Rc::new(Dielectric { ir: 1.5 }),
    }));

    objects.push(Box::new(Sphere {
        center: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Rc::new(Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1),
        }),
    }));

    objects.push(Box::new(Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 1.0)),
    }));

    HittableList { objects }
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: f64 = 1200.0;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: usize = 500;
    const MAX_DEPTH: usize = 50;

    // World

    let world = random_scene();

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    // Render

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    let scanlines = Arc::new(Mutex::new(IMAGE_HEIGHT));

    let image: String = (0..IMAGE_HEIGHT)
        .rev()
        .into_iter()
        .map(|j| {
            io::stdout().flush().unwrap();
            let line: String = (0..IMAGE_WIDTH as u32)
                .into_iter()
                .map(|i| {
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

                    let pixel = get_pixel(pixel_color, SAMPLES_PER_PIXEL);
                    pixel
                })
                .collect();
            let scanlines = Arc::clone(&scanlines);
            let mut scanline = scanlines.lock().unwrap();
            *scanline -= 1;
            eprint!("\rScanlines remaining: {} ", scanline);
            line
        })
        .collect();

    print!("{}", image);
    eprintln!("Done.");
}
