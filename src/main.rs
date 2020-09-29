mod hittable;
use hittable::*;

mod material;
use material::*;

mod camera;
use camera::*;

mod sphere;
use sphere::*;

mod vec;
use vec::*;

mod colour;
use colour::*;

mod ray;
use ray::*;

use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

fn ray_colour(
    r: &Ray,
    world: &HittableList,
    dist: &Uniform<f64>,
    rng: &mut ThreadRng,
    depth: u32,
) -> Colour {
    if let Some(hit) = world.hit(r, 0.001, f64::MAX) {
        if depth > 0 {
            if let Some((scattered, attenuation)) = hit.material.scatter(&hit, r, dist, rng) {
                return attenuation * ray_colour(&scattered, world, dist, rng, depth - 1);
            }
        }
        Colour::new(0.0, 0.0, 0.0)
    } else {
        let norm_dir = r.direction.norm();
        let t = 0.5 * (norm_dir.y + 1.0);
        return Colour::new(1.0, 1.0, 1.0) * (1.0 - t) + Colour::new(0.3, 0.5, 1.0) * t;
    }
}

fn main() {
    // Constants
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 1920;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const NUM_SAMPLES: u32 = 500;
    const MAX_DEPTH: u32 = 50;

    // RNG. Using uniform distribion for improved performance when generating lots of random numbers.
    let mut rng = rand::thread_rng();
    let dist = Uniform::from(0.0..1.0);

    // Camera.
    let camera = Camera::default();

    // Materials
    let grey_lambert = Lambertian::new(Colour::new(0.5, 0.5, 0.5));
    let blue_lambert = Lambertian::new(Colour::new(0.1, 0.2, 0.5));
    let gold_metal = Metal::new(Colour::new(0.8, 0.6, 0.2), 0.1);
    let dielectric = Dielectric::new(1.5);

    // World
    let world = HittableList::new(vec![
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, grey_lambert)),
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, blue_lambert)),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, dielectric)),
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, gold_metal)),
    ]);

    // -- Render
    // Write Header
    println!("P3\n {} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    // Loop over every row
    for j in (0..IMAGE_HEIGHT).rev() {
        // Display progress
        eprint!("\r{} rows completed!", IMAGE_HEIGHT - j);

        // Loop over every column
        for i in 0..IMAGE_WIDTH {
            // Create colour for this pixel
            let mut c = Colour::new(0.0, 0.0, 0.0);

            // Loop for multiple samples of pixel
            for _ in 0..NUM_SAMPLES {
                // Create ray through pixel (with rng for aliasing)
                let u = (i as f64 + dist.sample(&mut rng)) / IMAGE_WIDTH as f64; // Use uniform distribution for faster rng
                let v = (j as f64 + dist.sample(&mut rng)) / IMAGE_HEIGHT as f64;
                let r = camera.get_ray(u, v);
                c += ray_colour(&r, &world, &dist, &mut rng, MAX_DEPTH);
            }

            // Write pixel
            println!("{}", c / NUM_SAMPLES as f64);
        }
    }
    eprintln!("\nFinished!");
}
