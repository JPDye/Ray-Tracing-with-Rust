mod aabb;
mod camera;
mod material;
mod perlin;
mod sphere;
mod texture;
mod vec;

mod cuboid;
mod rect;

mod scenes;
use scenes::*;

mod hittable;
use hittable::*;

mod bvh;
use bvh::*;

mod colour;
use colour::*;

mod ray;
use ray::*;

use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

use rayon::prelude::*;

fn ray_colour(
    r: &Ray,
    bg: Colour,
    world: &Box<dyn Hittable>,
    dist: &Uniform<f64>,
    rng: &mut ThreadRng,
    depth: u32,
) -> Colour {
    if let Some(hit) = world.hit(r, 0.001, f64::MAX) {
        let emitted = hit.material.emitted(hit.u, hit.v, hit.p, dist, rng);
        if depth > 0 {
            if let Some((scattered, attenuation)) = hit.material.scatter(&hit, r, dist, rng) {
                return emitted + attenuation * ray_colour(&scattered, bg, world, dist, rng, depth - 1);
            }
        }
        emitted
    } else {
        bg
    }
}

fn main() {
    // Constants
    //const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const ASPECT_RATIO: f64 = 1.0;
    const IMAGE_WIDTH: u32 = 1200;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const NUM_SAMPLES: u32 = 1000;
    const MAX_DEPTH: u32 = 50;

    // Scene creation
    let choice = 5;
    let (world, camera) = match choice {
        1 => random_scene(ASPECT_RATIO),
        2 => two_checkered_spheres(ASPECT_RATIO),
        3 => two_perlin_spheres(ASPECT_RATIO),
        4 => simple_light(ASPECT_RATIO),
        5 => cornell_box(ASPECT_RATIO),
        _ => panic!("invalid scene selection"),
    };

    // BVH
    let world = Box::new(BVH::new(world.list, 0.0, 1.0)) as Box<dyn Hittable>;

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
                        c += ray_colour(&r, camera.bg, &world, &dist, &mut rng, MAX_DEPTH);
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

    // Write
    println!("P3\n {} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for colour in image.chunks(3) {
        println!("{} {} {}", colour[0], colour[1], colour[2]);
    }
}
