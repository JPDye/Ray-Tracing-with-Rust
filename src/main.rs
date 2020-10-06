use rayon::prelude::*;

use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use rand::Rng;

mod hittable;
use hittable::*;

mod material;
use material::*;

mod camera;
use camera::*;

mod bvh;
use bvh::*;

mod aabb;
use aabb::*;

mod moving_sphere;
use moving_sphere::*;

mod colour;
use colour::*;

mod sphere;
use sphere::*;

mod vec;
use vec::*;

mod ray;
use ray::*;

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let mut rng = rand::thread_rng();

    let ground = Lambertian::new(Colour::new(0.5, 0.5, 0.5));
    world.push(Box::new(Sphere::new(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rng.gen::<f64>();
            let center = Vec3(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Vec3(4.0, 0.2, 0.0)).mag() > 0.9 {
                if choose_material < 0.8 {
                    // Diffuse
                    let albedo = Colour::new(
                        rng.gen::<f64>() * rng.gen::<f64>(),
                        rng.gen::<f64>() * rng.gen::<f64>(),
                        rng.gen::<f64>() * rng.gen::<f64>(),
                    );

                    let center2 = center + Vec3(0.0, rng.gen_range(0.0, 0.5), 0.0);

                    let sphere_mat = Lambertian::new(albedo);
                    world.push(Box::new(MovingSphere::new(
                        center, center2, 0.0, 1.0, 0.2, sphere_mat,
                    )));
                } else if choose_material < 0.95 {
                    // Metal
                    let fuzz = rng.gen_range(0.0, 0.5);
                    let albedo = Colour::new(
                        rng.gen_range(0.5, 1.0),
                        rng.gen_range(0.5, 1.0),
                        rng.gen_range(0.5, 1.0),
                    );
                    let sphere_mat = Metal::new(albedo, fuzz);
                    world.push(Box::new(Sphere::new(center, 0.2, sphere_mat)));
                } else {
                    // Glass
                    let sphere_mat = Dielectric::new(1.5);
                    world.push(Box::new(Sphere::new(center, 0.2, sphere_mat)));
                }
            }
        }
    }

    let glass = Dielectric::new(1.5);
    world.push(Box::new(Sphere::new(Vec3(0.0, 1.0, 0.0), 1.0, glass)));

    let lambert = Lambertian::new(Colour::new(0.4, 0.2, 0.1));
    world.push(Box::new(Sphere::new(Vec3(-4.0, 1.0, 0.0), 1.0, lambert)));

    let metal = Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0);
    world.push(Box::new(Sphere::new(Vec3(4.0, 1.0, 0.0), 1.0, metal)));

    world
}

fn ray_colour(
    r: &Ray,
    world: &BVHNode,
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
        let norm_dir = r.direction.normalise();
        let t = 0.5 * (norm_dir.1 + 1.0);
        return Colour::new(1.0, 1.0, 1.0) * (1.0 - t) + Colour::new(0.3, 0.5, 1.0) * t;
    }
}

fn main() {
    // Constants
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const NUM_SAMPLES: u32 = 100;
    const MAX_DEPTH: u32 = 20;

    // RNG. Using uniform distribion for improved performance when generating lots of random numbers.

    // Camera.
    let look_from = Vec3(13.0, 2.0, 3.0);
    let look_at = Vec3(0.0, 0.0, 0.0);
    let vup = Vec3(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let aperture = 0.1;
    let focus_dist = 10.0;
    let time0 = 0.0;
    let time1 = 1.0;

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        ASPECT_RATIO,
        aperture,
        focus_dist,
        time0,
        time1,
    );

    // World.
    eprintln!("Building BVH");
    let mut world = random_scene();
    let world = BVHNode::new(&mut world.list, 0.0, 1.0);
    eprintln!("Done!");

    // Render
    let image = (0..IMAGE_HEIGHT)
        .into_par_iter()
        .rev()
        .flat_map(|j| {
            let mut rng = rand::thread_rng();
            let dist = Uniform::from(0.0..1.0);

            (0..IMAGE_WIDTH)
                .flat_map(|i| {
                    let mut c = Colour::new(0.0, 0.0, 0.0);

                    for _ in 0..NUM_SAMPLES {
                        let u = (i as f64 + dist.sample(&mut rng)) / IMAGE_WIDTH as f64;
                        let v = (j as f64 + dist.sample(&mut rng)) / IMAGE_HEIGHT as f64;
                        let r = camera.get_ray(u, v, &mut rng);
                        c += ray_colour(&r, &world, &dist, &mut rng, MAX_DEPTH);
                    }

                    vec![
                        (255.999 * (c.r / NUM_SAMPLES as f64).sqrt().max(0.0).min(1.0)) as u8,
                        (255.999 * (c.g / NUM_SAMPLES as f64).sqrt().max(0.0).min(1.0)) as u8,
                        (255.999 * (c.b / NUM_SAMPLES as f64).sqrt().max(0.0).min(1.0)) as u8,
                    ]
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>();

    // Render
    println!("P3\n {} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for colour in image.chunks(3) {
        println!("{} {} {}", colour[0], colour[1], colour[2]);
    }
}
