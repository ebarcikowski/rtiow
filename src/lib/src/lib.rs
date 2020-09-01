use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign, Div, DivAssign};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64
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
    fn length_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn length(&self) -> f64 {
        self.length_sq().sqrt()
    }

    fn unit(&self) -> Vec3 {
       (*self) / self.length()
    }

    fn dump(&self) {
        println!("{} {} {}", self.x, self.y, self.z);
    }

    fn dot(&self, other:Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(&self, other:Self) -> Vec3 {
        Self {
            x:self.y * other.z - self.z * other.y,
            y:self.z * other.x + self.x * other.z,
            z:self.x * other.y - self.y * other.x
        }
    }
}

fn write_color(vec: &Vec3) {
    let scale = 255.999;
    println!("{} {} {}",
             (vec.x * scale) as u32,
             (vec.y * scale) as u32,
             (vec.z * scale) as u32
    );
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