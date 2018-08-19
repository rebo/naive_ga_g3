use super::super::{
    Bivector, Bivector_e12, Bivector_e23, Bivector_e31, Multivector, Pseudoscalar, Vector,
};

use std::ops::{Add, BitXor, Mul, Neg};

impl Mul<Vector> for Pseudoscalar {
    type Output = Bivector;

    fn mul(self, rhs: Vector) -> Bivector {
        Bivector {
            e12: Bivector_e12(self.0 * rhs.e3),
            e23: Bivector_e23(self.0 * rhs.e1),
            e31: Bivector_e31(self.0 * rhs.e2),
        }
    }
}

impl Mul<Pseudoscalar> for Vector {
    type Output = Bivector;

    fn mul(self, rhs: Pseudoscalar) -> Bivector {
        Bivector {
            e23: Bivector_e23(self.e1 * rhs.0),
            e31: Bivector_e31(self.e2 * rhs.0),
            e12: Bivector_e12(self.e3 * rhs.0),
        }
    }
}
