use super::super::{
    Bivector, Bivector_e12, Bivector_e23, Bivector_e31, Multivector, Pseudoscalar, Vector,
};

use std::ops::{Add, BitXor, Mul, Neg};

impl Add<Multivector> for Pseudoscalar {
    type Output = Multivector;

    fn add(self, rhs: Multivector) -> Multivector {
        Multivector::from(self) + rhs
    }
}

impl Add<Pseudoscalar> for Multivector {
    type Output = Multivector;

    fn add(self, rhs: Pseudoscalar) -> Multivector {
        self + Multivector::from(rhs)
    }
}
