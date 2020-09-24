fn main() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;


    // Image Header
    println!("P3\n {} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    // Image Body
    for i in 0..IMAGE_HEIGHT {
        eprint!("\r{} of {} rows rendered...", i, IMAGE_HEIGHT);
        for j in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH as f64 - 1.0);
            let g = j as f64 / (IMAGE_WIDTH as f64 - 1.0);
            let b = 0.25;

            let ir = (r * 255.999) as u8;
            let ig = (g * 255.999) as u8;
            let ib = (b * 255.999) as u8;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("\nFinished!");
}
