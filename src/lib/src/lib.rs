use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64
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
        }
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
}