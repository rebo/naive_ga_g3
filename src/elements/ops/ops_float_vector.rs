use super::super::{
    Bivector, Bivector_e12, Bivector_e23, Bivector_e31, Multivector, Pseudoscalar, Vector,
};

use std::ops::{Add, BitXor, Mul, Neg};

// scalar mul vector
impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector {
            e1: self * rhs.e1,
            e2: self * rhs.e2,
            e3: self * rhs.e3,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Vector {
        Vector {
            e1: self.e1 * rhs,
            e2: self.e2 * rhs,
            e3: self.e3 * rhs,
        }
    }
}
