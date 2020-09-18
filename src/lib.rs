use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign, Div, DivAssign};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, other:f64) -> Vec3 {
        Vec3{x:self.x / other, y:self.y / other, z:self.z / other}
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other:f64) {
        *self = Self{x:self.x / other, y:self.y / other, z:self.z / other};
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other:Vec3) -> Vec3 {
        Vec3{x:self.x + other.x, y:self.y + other.y, z:self.z + other.z}
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other:Vec3) -> Vec3 {
        Vec3{x:self.x - other.x, y:self.y - other.y, z:self.z - other.z}
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other:Vec3) -> Vec3 {
        Vec3{x:self * other.x, y:self * other.y, z:self * other.z}
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other:Vec3) -> Vec3 {
        Vec3{x:self.x * other.x, y:self.y * other.y, z:self.z * other.z}
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scal:f64) -> Vec3 {
        Vec3{x:self.x * scal, y:self.y * scal, z:self.z * scal}
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3{x:-self.x, y:-self.y, z:-self.z}
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x:self.x + other.x,
            y:self.y + other.y,
            z:self.z + other.z
        };
    }
}


impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x:self.x - other.x,
            y:self.y - other.y,
            z:self.z - other.z
        }
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x:self.x * other.x,
            y:self.y * other.y,
            z:self.z * other.z
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            x:self.x * other,
            y:self.y * other,
            z:self.z * other
        };
    }
}


impl Vec3 {
    pub fn length_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_sq().sqrt()
    }

    pub fn unit(&self) -> Vec3 {
       (*self) / self.length()
    }

    pub fn dump(&self) {
        println!("{} {} {}", self.x, self.y, self.z);
    }

    pub fn dot(&self, other:Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other:Self) -> Vec3 {
        Self {
            x:self.y * other.z - self.z * other.y,
            y:self.z * other.x + self.x * other.z,
            z:self.x * other.y - self.y * other.x
        }
    }
}

pub fn write_color(vec: &Vec3) {
    let scale = 255.999;
    println!("{} {} {}",
             (vec.x * scale) as u32,
             (vec.y * scale) as u32,
             (vec.z * scale) as u32
    );
}

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3
}

impl Ray {
    pub fn at(&self, t:f64) -> Vec3 {
        self.origin + self.dir * t
    }
}

pub struct HitRecord {
    pub p: Vec3,
    normal: Vec3,
    t: f64,
    front_face: bool
}

trait Hittable {
    fn hit(&self, r:Ray, t_min:f64, t_max:f64, rec: &HitRecord) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Vec3,
    radius: f64
}

impl Hittable for Sphere {
    fn hit(&self, r:Ray, t_min:f64, t_max:f64, rec: &HitRecord) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.dir.length_sq();
        let half_b = oc.dot(r.dir);
        let c = oc.length_sq() - self.radius * self.radius;
        let dis = half_b * half_b - a * c;

        if dis > 0.0 {
            let root = dis.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.at(rec.t);
                let normal = (p - self.center) / self.radius;
                let mut hr = HitRecord {p, t, normal, front_face:false};
                git let outward_normal = (rec.p - self.center) / self.radius;
                hr.set_face_normal(&r, &outward_normal);
                return Some(hr);
            }
        }
        None
    }
}

impl HitRecord {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.dir.dot(*outward_normal) < 0.0;
        if !self.front_face {
            self.normal = - *outward_normal;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn vec3_add() {
        let added = Vec3{x:1.0, y:2.0, z:3.0} + Vec3{x:4.0, y:5.0, z:6.0};
        assert_eq!(added.x, 5.0);
        assert_eq!(added.y, 7.0);
        assert_eq!(added.z, 9.0);
    }

    #[test]
    fn vec3_add_assign() {
        let mut added = Vec3{x:1.0, y:2.0, z:3.0};
        added += Vec3{x:4.0, y:5.0, z:6.0};
        assert_eq!(added.x, 5.0);
        assert_eq!(added.y, 7.0);
        assert_eq!(added.z, 9.0);
    }
    #[test]
    fn vec3_sub() {
        let subbed = Vec3{x:1.0, y:2.0, z:3.0} - Vec3{x:4.0, y:5.0, z:6.0};
        assert_eq!(subbed.x, -3.0);
        assert_eq!(subbed.y, -3.0);
        assert_eq!(subbed.z, -3.0);
    }

    #[test]
    fn vec3_sub_assign() {
        let mut subbed = Vec3{x:1.0, y:2.0, z:3.0};
        subbed -= Vec3{x:4.0, y:5.0, z:6.0};
        assert_eq!(subbed.x, -3.0);
        assert_eq!(subbed.y, -3.0);
        assert_eq!(subbed.z, -3.0);
    }

    #[test]
    fn vec3_mul() {
        let mul = Vec3{x:1.0, y:2.0, z:3.0} * Vec3{x:4.0, y:5.0, z:6.0};
        assert_eq!(mul.x, 4.0);
        assert_eq!(mul.y, 10.0);
        assert_eq!(mul.z, 18.0);

        let mul2 = Vec3{x:1.0, y:1.0, z:1.0} * 10.0;
        assert_eq!(mul2.x, 10.0);
        assert_eq!(mul2.y, 10.0);
        assert_eq!(mul2.z, 10.0);
    }

    #[test]
    fn vec3_mul_assign() {
        let mut mul = Vec3{x:1.0, y:2.0, z:3.0};
        mul *= Vec3{x:4.0, y:5.0, z:6.0};
        assert_eq!(mul.x, 4.0);
        assert_eq!(mul.y, 10.0);
        assert_eq!(mul.z, 18.0);

        let mut mul2 = Vec3{x:1.0, y:1.0, z:1.0};
        mul2 *= 10.0;
        assert_eq!(mul2.x, 10.0);
        assert_eq!(mul2.y, 10.0);
        assert_eq!(mul2.z, 10.0);
    }

    #[test]
    fn vec3_length() {
        let v = Vec3{x:1., y:1., z:1.};
        assert_eq!(v.length_sq(), 3.);

        assert_eq!(v.length(), 3_f64.sqrt());

    }
}