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
    const IMAGE_WIDTH: u32 = 1200;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const NUM_SAMPLES: u32 = 500;
    const MAX_DEPTH: u32 = 50;

    // RNG. Using uniform distribion for improved performance when generating lots of random numbers.
    let mut rng = rand::thread_rng();
    let dist = Uniform::from(0.0..1.0);

    // Camera. Set point to look from, point to look at, upwards direction, vertical field of view and aspect ration.
    let look_from = Vec3::new(-3.0, 3.0, 2.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let aperture = 0.5;
    let focus_dist = (look_from - look_at).mag();


    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        ASPECT_RATIO,
        aperture,
        focus_dist,
    );

    // Materials.
    let grey_lambert = Lambertian::new(Colour::new(0.4, 0.4, 0.4));
    let blue_lambert = Lambertian::new(Colour::new(0.1, 0.2, 0.5));

    let glass = Dielectric::new(1.5);

    let gold = Metal::new(Colour::new(0.8, 0.6, 0.2), 0.3);

    // World.
    let world = HittableList::new(vec![
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, grey_lambert)),
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, blue_lambert)),
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, gold)),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, glass)),
    ]);



    // Render
    println!("P3\n {} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\r{} rows completed!", IMAGE_HEIGHT - j);

        for i in 0..IMAGE_WIDTH {
            let mut c = Colour::new(0.0, 0.0, 0.0);

            for _ in 0..NUM_SAMPLES {
                let u = (i as f64 + dist.sample(&mut rng)) / IMAGE_WIDTH as f64; // Use uniform distribution for faster rng
                let v = (j as f64 + dist.sample(&mut rng)) / IMAGE_HEIGHT as f64;
                let r = camera.get_ray(u, v, &dist, &mut rng);
                c += ray_colour(&r, &world, &dist, &mut rng, MAX_DEPTH);
            }

            println!("{}", c / NUM_SAMPLES as f64);
        }
    }
    eprintln!("\nFinished!");
}
