mod vec;
use vec::*;

mod colour;
use colour::*;

mod ray;
use ray::*;


fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin - center;

    let a = r.direction.dot(&r.direction);
    let b = oc.dot(&r.direction) * 2.0;
    let c = oc.dot(&oc) - radius * radius;

    let discriminant = b*b - 4.0*a*c;
    discriminant > 0.0
}


fn ray_colour(r: &Ray) -> Colour {
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Colour::new(1.0, 0.0, 0.0)
    }

    let norm_dir = r.direction.norm();
    let t = 0.5 * (norm_dir.y + 1.0);
    Colour::new(1.0, 1.0, 1.0) * (1.0 - t) + Colour::new(0.3, 0.5, 1.0) * t
}


fn main() {
    // Image Settings
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    // Camera Settings
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);


    // Render
    println!("P3\n {} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / IMAGE_WIDTH as f64;
            let v = j as f64 / IMAGE_HEIGHT as f64;

            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v - origin);

            let c = ray_colour(&r);

            println!("{}", c);
        }
    }
}
