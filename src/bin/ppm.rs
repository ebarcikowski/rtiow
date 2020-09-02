extern crate rtiow;

use rtiow::Vec3;

fn main() {
    let width = 256;
    let height = 256;

    println!("P3 {} {} 255", width, height);
    for i in (0..height).rev() {
        eprintln!("Scanlines remaining: {}", i);
        for j in 0..width {

            let r = (j as f64) / (width as f64 - 1.0);
            let g = (i as f64) / (height as f64 - 1.0);
            let b = 0.25;
            let color = Vec3 {x:r, y:g, z:b};
            rtiow::write_color(&color);
        }
    }
    eprintln!("*****\nDone!\n*****");
}
