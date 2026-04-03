use std::ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut, AddAssign};
use rand::{Rng, RngExt};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    vec: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { vec: [x, y, z] }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.vec[0] * self.vec[0]
            + self.vec[1] * self.vec[1]
            + self.vec[2] * self.vec[2]
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn cross_product(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            vec: [
                self.vec[1] * other.vec[2] - self.vec[2] * other.vec[1],
                self.vec[2] * other.vec[0] - self.vec[0] * other.vec[2],
                self.vec[0] * other.vec[1] - self.vec[1] * other.vec[0],
            ],
        }
    }

    pub fn dot_product(&self, other: &Vec3) -> f64 {
        self.vec[0] * other.vec[0]
            + self.vec[1] * other.vec[1]
            + self.vec[2] * other.vec[2]
    }

    pub fn random_range(min: f64, max: f64, rng: &mut impl Rng) -> Vec3 {
        Vec3::new(rng.random_range(min..max), rng.random_range(min..max), rng.random_range(min..max))
    }

    pub fn random(rng: &mut impl Rng) -> Vec3 {
        Vec3::new(rng.random(), rng.random(), rng.random())
    }

    pub fn random_unit_vector(rng: &mut impl Rng) -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0, rng);
            let lensq = p.length_squared();

            if lensq > 1e-160 && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3, rng: &mut impl Rng) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector(rng);
        if on_unit_sphere.dot_product(normal) > 0.0 {
            on_unit_sphere
        } else { -on_unit_sphere }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            vec: [
                self.vec[0] + other.vec[0],
                self.vec[1] + other.vec[1],
                self.vec[2] + other.vec[2],
            ],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.vec[0] += other.vec[0];
        self.vec[1] += other.vec[1];
        self.vec[2] += other.vec[2];
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            vec: [
                self.vec[0] - other.vec[0],
                self.vec[1] - other.vec[1],
                self.vec[2] - other.vec[2],
            ],
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            vec: [
                self.vec[0] * other.vec[0],
                self.vec[1] * other.vec[1],
                self.vec[2] * other.vec[2],
            ],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3 {
            vec: [
                self.vec[0] * t,
                self.vec[1] * t,
                self.vec[2] * t,
            ],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        self * (1.0 / t)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            vec: [-self.vec[0], -self.vec[1], -self.vec[2]],
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        &self.vec[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.vec[i]
    }
}