use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;
use std::ops::Div;

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn make_unit_vector(mut self) {
        let k = 1.0 / self.length();
        self.x = self.x * k;
        self.y = self.y * k;
        self.z = self.z * k;
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: -(self.x * other.z - self.z * other.x),
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        let k = 1.0 / self.length();
        Vec3 {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k
        }
    }
}

impl<'a> Add<&'a Vec3> for &'a Vec3 {
    type Output = (Vec3);

    fn add(self, rhs: &'a Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<'a> Mul<&'a Vec3> for &'a Vec3 {
    type Output = (Vec3);

    fn mul(self, rhs: &'a Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<'a> Mul<&'a f64> for &'a Vec3 {
    type Output = (Vec3);

    fn mul(self, rhs: &'a f64) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<'a> Sub<&'a Vec3> for &'a Vec3 {
    type Output = (Vec3);

    fn sub(self, rhs: &'a Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<'a> Div<&'a Vec3> for &'a Vec3 {
    type Output = (Vec3);

    fn div(self, rhs: &'a Vec3) -> Vec3 {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<'a> Div<&'a f64> for &'a Vec3 {
    type Output = (Vec3);

    fn div(self, rhs: &'a f64) -> Vec3 {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

mod tests {
    use vec3::Vec3;

    #[test]
    fn multiply_by_vec() {
        let a  = Vec3 {x: 1.0, y: 2.0, z: 3.0};
        let b  = Vec3 {x: 1.0, y: 2.0, z: 3.0};
        assert_eq!(&a * &b, Vec3 {x: 1.0, y: 4.0, z: 9.0});
        assert_eq!(a, b);
    }
}