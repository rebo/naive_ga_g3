use crate::elements::{Bivector, Multivector, Rotor};
use float_cmp::ApproxEq;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    pub e1: f64,
    pub e2: f64,
    pub e3: f64,
}

pub trait OverloadedDot<T, R> {
    fn overloaded_dot(self, rhs: T) -> R;
}

impl OverloadedDot<Vector, f64> for Vector {
    fn overloaded_dot(self, rhs: Vector) -> f64 {
        self.e1 * rhs.e1 + self.e2 * rhs.e2 + self.e3 * rhs.e3
    }
}

#[allow(eq_op)]
impl OverloadedDot<Bivector, Vector> for Vector {
    fn overloaded_dot(self, rhs: Bivector) -> Vector {
        0.5 * (self * rhs - rhs * self).vector
    }
}

impl Vector {
    pub fn new(e1: f64, e2: f64, e3: f64) -> Vector {
        Vector { e1, e2, e3 }
    }

    pub fn angle(self, v: Vector) -> f64 {
        let cos_theta = self.dot(v) / (self.mag() * v.mag());
        cos_theta.acos()
    }

    pub fn lerp(self, v: Vector, s: f64) -> Vector {
        (1.0 - s) * self + s * v
    }

    pub fn slerp(self, v: Vector, s: f64) -> Vector {
        // Note that this will only point self towards v, it will not shorten or lengthen self.
        // s should be  between 0.0 and 1.0.

        let theta = self.angle(v);

        let a_s = ((1.0 - s) * theta).sin() / theta.sin();
        let b_s = (s * theta).sin() / theta.sin();

        a_s * self + b_s * v
    }

    pub fn rev(self) -> Vector {
        // let k = 1;
        // (-1.0f64).powi((k * (k - 1)) / 2) * self
        self
    }

    pub fn dot<T, R>(self, rhs: T) -> R
    where
        Self: OverloadedDot<T, R>,
    {
        self.overloaded_dot(rhs)
    }

    pub fn is_zero(self) -> bool {
        self.e1.approx_eq(&0.0, 2.0 * std::f64::EPSILON, 2)
            && self.e2.approx_eq(&0.0, 2.0 * std::f64::EPSILON, 2)
    }

    pub fn mag(self) -> f64 {
        self.mag2().powf(0.5)
    }

    pub fn inv(self) -> Vector {
        let mag2 = self.mag2();
        Self {
            e1: self.e1 / mag2,
            e2: self.e2 / mag2,
            e3: self.e3 / mag2,
        }
    }

    pub fn mag2(self) -> f64 {
        self.e1 * self.e1 + self.e2 * self.e2 + self.e3 * self.e3
    }

    pub fn normalize(self) -> Vector {
        let v_size = self.mag();

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

    pub fn reflect_in_vector(self, v: Vector) -> Vector {
        self.proj(v) - self.rej(v)
    }

    pub fn reflect_in_plane_with_normal(self, n: Vector) -> Vector {
        // ensure n is normalized
        let n = n.normalize();

        -(n * self * n).vector
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
