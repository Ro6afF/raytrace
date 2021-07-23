use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x: x, y: y, z: z }
    }

    pub fn lenght(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normailzed(&self) -> Self {
        Self {
            x: self.x / self.lenght(),
            y: self.y / self.lenght(),
            z: self.z / self.lenght(),
        }
    }

    pub fn normailze(&mut self) {
        *self = *self / self.lenght();
    }

    pub fn random_in_unit_sphere() -> Self {
        let mut vec = Self::new(
            fastrand::f64() * 2.0 - 1.0,
            fastrand::f64() * 2.0 - 1.0,
            fastrand::f64() * 2.0 - 1.0,
        );

        while vec.lenght() > 1.0 {
            vec = Self::new(
                fastrand::f64() * 2.0 - 1.0,
                fastrand::f64() * 2.0 - 1.0,
                fastrand::f64() * 2.0 - 1.0,
            );
        }

        vec
    }

    pub fn random_unit() -> Self {
        Self::random_in_unit_sphere().normailzed()
    }

    pub fn random_in_unit_disc_xy() -> Self {
        let mut vec = Self::new(
            fastrand::f64() * 2.0 - 1.0,
            fastrand::f64() * 2.0 - 1.0,
            0.0,
        );

        while vec.lenght() >= 1.0 {
            vec = Self::new(
                fastrand::f64() * 2.0 - 1.0,
                fastrand::f64() * 2.0 - 1.0,
                0.0,
            );
        }

        vec
    }
}

impl ops::Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl ops::Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        rhs * self
    }
}

impl ops::Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        self * (1.0 / rhs)
    }
}

impl ops::Mul for Vector {
    type Output = f64;

    fn mul(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl ops::Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        self * -1.0
    }
}

impl ops::BitXor for Vector {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

pub type Point = Vector;
