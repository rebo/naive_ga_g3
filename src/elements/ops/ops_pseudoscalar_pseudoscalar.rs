use super::super::{
    Bivector, Bivector_e12, Bivector_e23, Bivector_e31, Multivector, Pseudoscalar, Vector,
};

use std::ops::{Add, BitXor, Mul, Neg};

impl Add for Pseudoscalar {
    type Output = Pseudoscalar;

    fn add(self, rhs: Pseudoscalar) -> Pseudoscalar {
        Pseudoscalar(self.0 + rhs.0)
    }
}

impl Mul for Pseudoscalar {
    type Output = f64;

    fn mul(self, rhs: Pseudoscalar) -> f64 {
        -self.0 * rhs.0
    }
}
