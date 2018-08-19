// use super::bivector::Bivector;
use super::multivector::{Multivector, Rotor};
use float_cmp::ApproxEq;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    pub e1: f64,
    pub e2: f64,
    pub e3: f64,
}

impl Vector {
    pub fn rev(self) -> Vector {
        // let k = 1;
        // (-1.0f64).powi((k * (k - 1)) / 2) * self
        self
    }

    pub fn is_zero(self) -> bool {
        self.e1.approx_eq(&0.0, 2.0 * std::f64::EPSILON, 2)
            && self.e2.approx_eq(&0.0, 2.0 * std::f64::EPSILON, 2)
    }

    pub fn inv(self) -> Vector {
        Self {
            e1: self.e1 / self.abs_sq(),
            e2: self.e2 / self.abs_sq(),
            e3: self.e3 / self.abs_sq(),
        }
    }

    pub fn abs_sq(self) -> f64 {
        self.e1 * self.e1 + self.e2 * self.e2 + self.e3 * self.e3
    }

    pub fn normalize(self) -> Vector {
        let v_size = (self.abs_sq()).powf(0.5);

        Vector {
            e1: self.e1 / v_size,
            e2: self.e2 / v_size,
            e3: self.e3 / v_size,
        }
    }

    pub fn proj(self, rhs: Vector) -> Vector {
        self.dot(rhs) * rhs.inv()
    }

    pub fn rej(self, rhs: Vector) -> Vector {
        //because rhs.inv is in the plane of self^rhs bivector we will get a vector
        ((self ^ rhs) * rhs.inv()).vector
    }

    pub fn reflect(self, v: Vector) -> Vector {
        self.proj(v) - self.rej(v)
    }

    pub fn apply_rotor(self, rotor: Rotor) -> Vector {
        Vector::from(rotor.rev() * self * rotor)
    }

    pub fn zero() -> Vector {
        Vector {
            e1: 0.0,
            e2: 0.0,
            e3: 0.0,
        }
    }

    pub fn dot(self, rhs: Vector) -> f64 {
        self.e1 * rhs.e1 + self.e2 * rhs.e2 + self.e3 * rhs.e3
    }

    // pub fn dot_with_bivector(self, rhs: Bivector) -> Vector {
    //     self * rhs
    // }

    pub fn e1() -> Self {
        Self {
            e1: 1.0,
            e2: 0.0,
            e3: 0.0,
        }
    }
    pub fn e2() -> Self {
        Self {
            e1: 0.0,
            e2: 1.0,
            e3: 0.0,
        }
    }
    pub fn e3() -> Self {
        Self {
            e1: 0.0,
            e2: 0.0,
            e3: 1.0,
        }
    }
}

use std::convert::From;

impl From<Multivector> for Vector {
    fn from(item: Multivector) -> Self {
        if !(item.scalar.approx_eq(&0.0, 2.0 * std::f64::EPSILON, 2)) {
            panic!("scalar not zero")
        }

        if !(item
            .pseudoscalar
            .0
            .approx_eq(&0.0, 2.0 * std::f64::EPSILON, 2))
        {
            panic!("bivector not zero")
        }

        if !(item
            .bivector
            .e12
            .0
            .approx_eq(&0.0, 2.0 * std::f64::EPSILON, 2))
        {
            panic!("bivector not zero")
        }

        if !(item
            .bivector
            .e23
            .0
            .approx_eq(&0.0, 2.0 * std::f64::EPSILON, 2))
        {
            panic!("bivector not zero")
        }

        if !(item
            .bivector
            .e31
            .0
            .approx_eq(&0.0, 2.0 * std::f64::EPSILON, 2))
        {
            panic!("bivector not zero")
        }

        Vector {
            e1: item.vector.e1,
            e2: item.vector.e2,
            e3: item.vector.e3,
        }
    }
}
