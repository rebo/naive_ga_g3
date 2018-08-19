use super::super::{
    Bivector, Bivector_e12, Bivector_e23, Bivector_e31, Multivector, Pseudoscalar, Vector,
};

use std::ops::{Add, BitXor, Mul, Neg};

// vector mul multivector

impl Mul<f64> for Pseudoscalar {
    type Output = Pseudoscalar;

    fn mul(self, rhs: f64) -> Pseudoscalar {
        Pseudoscalar(self.0 * rhs)
    }
}

impl Mul<Pseudoscalar> for f64 {
    type Output = Pseudoscalar;

    fn mul(self, rhs: Pseudoscalar) -> Pseudoscalar {
        Pseudoscalar(self * rhs.0)
    }
}
