extern crate rtiow;

use rtiow::{Vec3, Ray, write_color};

fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - *center;
    let a = ray.dir.dot(ray.dir);
    let half_b = oc.dot(ray.dir);
    let c = oc.dot(oc) - radius * radius;
    let dis = half_b * half_b - a * c;
    if dis < 0.0 {
        return -1.0;
    }
    (-half_b - dis.sqrt()) / a
}

fn ray_color(r: &Ray) -> Vec3 {
    let c = Vec3{x:0.0, y:0.0, z:-1.0};
    // distance from source to edge of sphere.
    let t = hit_sphere(&c, 0.5, r);
    if t > 0.0 {
        // normal on sphere is ray - center of sphere.
        let n = (r.at(t) - Vec3{x:0.0, y:0.0, z:-1.0}).unit();
        // RGB color is simply the x, y and z components
        return 0.5 * Vec3{x:n.x + 1.0, y:n.y + 1.0, z:n.z + 1.0}
    }
    // Missed the sphere. Just use same background.
    let ud = r.dir.unit();
    // Why s? I wanted a new variable that's like t, so s. Yeah kind of lazy.
    let s = 0.5 * (ud.y + 1.0);
    (1.0 - s) * Vec3{x:1.0, y:1.0, z:1.0} + s * Vec3{x:0.5, y:0.7, z:1.0}
}

fn main() {
    let ar = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / ar) as i32;

    let viewport_height = 2.0;
    let viewport_width = ar * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3{x:0.0, y:0.0, z:0.0};
    let horizontal = Vec3{x:viewport_width, y:0.0, z:0.0};
    let vertical = Vec3{x:0.0, y:viewport_height, z:0.0};
    let lower_left_corner = origin - vertical / 2.0
        - horizontal / 2.0 - Vec3{x:0.0, y:0.0, z:focal_length};

    println!("P3\n {} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("Scanline remaining: {}", j);
        for i in 0..image_width {
            let u = (i as f64) / ((image_width - 1) as f64);
            let v = (j as f64) / ((image_height - 1) as f64);
            let dir = lower_left_corner + u * horizontal + v * vertical - origin;
            let r = Ray{origin, dir};
            let pixel_color = ray_color(&r);
            write_color(&pixel_color);
        }
    }
    eprintln!("Done");
}

