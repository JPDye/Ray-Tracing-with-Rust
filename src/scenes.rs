use rand::Rng;

use crate::texture::{CheckeredTexture, NoiseTexture, SolidColour};

use crate::camera::Camera;
use crate::colour::Colour;
use crate::hittable::HittableList;
use crate::material::{Dielectric, Lambertian, Metal, DiffuseLight};

use crate::cuboid::Cuboid;
use crate::rect::{XYRect, XZRect, YZRect};
use crate::sphere::Sphere;

use crate::vec::Vec3;
use crate::vec::Axis::{self, *};

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
        Colour::new(0.7, 0.8, 1.0),
        Vec3(13.0, 2.0, 3.0),
        Vec3(0.0, 0.0, 0.0),
        Vec3(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
        0.0,
        1.0,
    );

    (world, camera)
}

pub fn two_checkered_spheres(aspect_ratio: f64) -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let white = SolidColour::new(Colour::new(0.9, 0.9, 0.9));
    let green = SolidColour::new(Colour::new(0.2, 0.3, 0.1));
    let checker = CheckeredTexture::new(white, green);

    let mat = Lambertian::new(checker);
    world.push(Box::new(Sphere::new(Vec3(0.0, -10.0, 0.0), 10.0, mat)));
    world.push(Box::new(Sphere::new(Vec3(0.0, 10.0, 0.0), 10.0, mat)));

    let camera = Camera::new(
        Colour::new(0.7, 0.8, 1.0),
        Vec3(13.0, 2.0, 3.0),
        Vec3(0.0, 0.0, 0.0),
        Vec3(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
        0.0,
        1.0,
    );

    (world, camera)
}

pub fn two_perlin_spheres(aspect_ratio: f64) -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let perlin_1 = NoiseTexture::new();
    let perlin_2 = NoiseTexture::new();

    world.push(Box::new(Sphere::new(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(perlin_1),
    )));
    world.push(Box::new(Sphere::new(
        Vec3(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new(perlin_2),
    )));

    let camera = Camera::new(
        Colour::new(0.7, 0.8, 1.0),
        Vec3(13.0, 2.0, 3.0),
        Vec3(0.0, 0.0, 0.0),
        Vec3(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
        0.0,
        1.0,
    );

    (world, camera)
}

pub fn simple_light(aspect_ratio: f64) -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let white = SolidColour::new(Colour::new(0.9, 0.9, 0.9));
    let green = SolidColour::new(Colour::new(0.2, 0.3, 0.1));
    let blue = SolidColour::new(Colour::new(0.1, 0.2, 0.3));

    let green_checker = CheckeredTexture::new(white, green);
    let green_checker_mat = Lambertian::new(green_checker);

    let blue_checker = CheckeredTexture::new(white, blue);
    let blue_checker_mat = Lambertian::new(blue_checker);

    world.push(Box::new(Sphere::new(Vec3(0.0, -1000.0, 0.0), 1000.0, green_checker_mat)));
    world.push(Box::new(Sphere::new(Vec3(0.0, 2.0, 0.0), 2.0, blue_checker_mat)));

    let light = DiffuseLight::new(SolidColour::new(Colour::new(4.0, 4.0, 4.0)));
    world.push(Box::new(Sphere::new(Vec3(5.0, 8.0, 5.0), 2.0, light)));

    let camera = Camera::new(
        Colour::new(0.001, 0.001, 0.001),
        Vec3(26.0, 3.0, 6.0),
        Vec3(0.0, 2.0, 0.0),
        Vec3(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.05,
        20.0,
        0.0,
        1.0,
    );

    (world, camera)
}

pub fn cornell_box(aspect_ratio: f64) -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let red = Lambertian::new(SolidColour::new(Colour::new(0.65, 0.05, 0.05)));
    let white = Lambertian::new(SolidColour::new(Colour::new(0.73, 0.73, 0.73)));
    let green = Lambertian::new(SolidColour::new(Colour::new(0.12, 0.45, 0.15)));

    let light = DiffuseLight::new(SolidColour::new(Colour::new(15.0, 15.0, 15.0)));
    world.push(Box::new(XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light))); // light

    world.push(Box::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red))); // right
    world.push(Box::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green))); // left
    world.push(Box::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white))); // bottom
    world.push(Box::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white))); // top
    world.push(Box::new(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white))); // back

    world.push(Box::new(Cuboid::new(Vec3(130.0, 0.0, 65.0), Vec3(296.0, 165.0, 230.0), white)));
    world.push(Box::new(Cuboid::new(Vec3(265.0, 0.0, 295.0), Vec3(430.0, 330.0, 460.0), white)));

    let camera = Camera::new(
        Colour::new(0.0, 0.0, 0.0),
        Vec3(278.0, 278.0, -800.0),
        Vec3(278.0, 278.0, 0.0),
        Vec3(0.0, 1.0, 0.0),
        40.0,
        aspect_ratio,
        0.00,
        800.0,
        0.0,
        1.0
    );

    (world, camera)
}
