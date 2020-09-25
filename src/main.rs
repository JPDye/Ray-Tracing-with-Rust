mod vec;
use vec::*;

mod colour;
use colour::*;

fn main() {
    const IMAGE_WIDTH: u32 = 256;    // Image width
    const IMAGE_HEIGHT: u32 = 256;    // Image height

    // Image Header
    println!("P3\n {} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    // Image Body
    for i in 0..IMAGE_HEIGHT {
        eprint!("\r{} of {} rows rendered...", i+1, IMAGE_HEIGHT);
        for j in 0..IMAGE_WIDTH {
            let c = Colour { r: i as f64 / IMAGE_WIDTH as f64, g: j as f64 / IMAGE_HEIGHT as f64, b: 0.25 };
            println!("{}", c);
        }
    }
    eprintln!("\nFinished!");
}
