use super::super::{
    Bivector, Bivector_e12, Bivector_e23, Bivector_e31, Multivector, Pseudoscalar, Vector,
};

use std::ops::{Add, BitXor, Mul, Neg, Sub};

impl Add<f64> for Multivector {
    type Output = Multivector;

    fn add(self, rhs: f64) -> Self {
        Multivector {
            scalar: self.scalar + rhs,
            vector: self.vector,
            bivector: self.bivector,
            pseudoscalar: self.pseudoscalar,
        }
    }
}

impl Add<Multivector> for f64 {
    type Output = Multivector;

    fn add(self, rhs: Multivector) -> Multivector {
        Multivector {
            scalar: self + rhs.scalar,
            vector: rhs.vector,
            bivector: rhs.bivector,
            pseudoscalar: rhs.pseudoscalar,
        }
    }
}

impl Sub<f64> for Multivector {
    type Output = Multivector;

    fn sub(self, rhs: f64) -> Self {
        Multivector {
            scalar: self.scalar - rhs,
            vector: self.vector,
            bivector: self.bivector,
            pseudoscalar: self.pseudoscalar,
        }
    }
}

impl Sub<Multivector> for f64 {
    type Output = Multivector;

    fn sub(self, rhs: Multivector) -> Multivector {
        Multivector {
            scalar: self - rhs.scalar,
            vector: -1.0 * rhs.vector,
            bivector: -1.0 * rhs.bivector,
            pseudoscalar: -1.0 * rhs.pseudoscalar,
        }
    }
}
