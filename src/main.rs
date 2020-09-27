mod hittable;
use hittable::*;

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

use rand::prelude::*;


fn ray_colour(r: &Ray, world: &HittableList) -> Colour {
    if let Some(hit) = world.hit(r, 0.0, f64::MAX) {
        Colour::new(hit.norm.x + 1.0, hit.norm.y + 1.0, hit.norm.z + 1.0) * 0.5
    } else {
        let norm_dir = r.direction.norm();
        let t = 0.5 * (norm_dir.y + 1.0);
        Colour::new(1.0, 1.0, 1.0) * (1.0 - t) + Colour::new(0.3, 0.5, 1.0) * t
    }
}


fn main() {
    // Constants
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const NUM_SAMPLES: u32 = 25;


    // Initialise random number generator
    let mut rng = rand::thread_rng();

    // Initialise camera
    let camera = Camera::default();


    // Initialise world
    let world = HittableList::new(
        vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ]);


    // Render

    // Write header
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
                let u = (i as f64 + rng.gen::<f64>()) / IMAGE_WIDTH as f64;
                let v = (j as f64 + rng.gen::<f64>()) / IMAGE_HEIGHT as f64;
                let r = camera.get_ray(u, v);
                c += ray_colour(&r, &world);
            }

            // Write pixel
            println!("{}", c / NUM_SAMPLES as f64);
        }
    }
    eprintln!("\nFinished!");
}
