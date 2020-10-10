use rand::Rng;

use crate::camera::Camera;
use crate::colour::Colour;
use crate::hittable::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::texture::{CheckeredTexture, SolidColour};
use crate::vec::Vec3;

pub fn random_scene(aspect_ratio: f64) -> (HittableList, Camera) {
    let mut world = HittableList::new();
    let mut rng = rand::thread_rng();

    let white = SolidColour::new(Colour::new(0.9, 0.9, 0.9));
    let green = SolidColour::new(Colour::new(0.2, 0.3, 0.1));
    let checker = CheckeredTexture::new(white, green);

    let ground = Lambertian::new(checker);
    world.push(Box::new(Sphere::new(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        ground,
    )));

    for a in -10..10 {
        for b in -10..10 {
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

                    let colour = SolidColour::new(albedo);
                    let sphere_mat = Lambertian::new(colour);
                    world.push(Box::new(Sphere::new(center, 0.2, sphere_mat)));
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

    let colour = SolidColour::new(Colour::new(0.4, 0.2, 0.1));
    let lambert = Lambertian::new(colour);
    world.push(Box::new(Sphere::new(Vec3(-4.0, 1.0, 0.0), 1.0, lambert)));

    let metal = Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0);
    world.push(Box::new(Sphere::new(Vec3(4.0, 1.0, 0.0), 1.0, metal)));

    let camera = Camera::new(
        Vec3(13.0, 2.0, 3.0),
        Vec3(0.0, 0.0, 0.0),
        Vec3(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.0,
        10.0,
        0.0,
        1.0
    );

    (world, camera)
}

pub fn two_spheres(aspect_ratio: f64) -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let white = SolidColour::new(Colour::new(0.9, 0.9, 0.9));
    let green = SolidColour::new(Colour::new(0.2, 0.3, 0.1));
    let checker = CheckeredTexture::new(white, green);

    let mat = Lambertian::new(checker);
    world.push(Box::new(Sphere::new(Vec3(0.0, -10.0, 0.0), 10.0, mat)));
    world.push(Box::new(Sphere::new(Vec3(0.0, 10.0, 0.0), 10.0, mat)));

    let camera = Camera::new(
        Vec3(13.0, 2.0, 3.0),
        Vec3(0.0, 0.0, 0.0),
        Vec3(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.0,
        10.0,
        0.0,
        1.0
    );

    (world, camera)
}








