extern crate rtiow;

use rtiow::Vec3;

fn main() {
    let width = 256;
    let height = 256;

    println!("P3 {} {} 255", width, height);
    for i in (0..height).rev() {
        eprintln!("Scanlines remaining: {}", i);
        for j in 0..width {

            let v:Vec3 = Vec3 {x:1.0, y:1.0, z:1.0};

            let r = (j as f32) / (width as f32 - 1.0);
            let g = (i as f32) / (height as f32 - 1.0);
            let b:f32 = 0.25;

            let ir = (r * 255.999) as u32;
            let ig = (g * 255.999) as u32;
            let ib = (b * 255.999) as u32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("*****\nDone!\n*****");
}
