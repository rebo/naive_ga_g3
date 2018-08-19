use super::super::{Bivector, BivectorE12, BivectorE23, BivectorE31, Pseudoscalar, Vector};

use std::ops::Mul;

impl Mul<Vector> for Pseudoscalar {
    type Output = Bivector;

    fn mul(self, rhs: Vector) -> Bivector {
        Bivector {
            e12: BivectorE12(self.0 * rhs.e3),
            e23: BivectorE23(self.0 * rhs.e1),
            e31: BivectorE31(self.0 * rhs.e2),
        }
    }
}

impl Mul<Pseudoscalar> for Vector {
    type Output = Bivector;

    fn mul(self, rhs: Pseudoscalar) -> Bivector {
        Bivector {
            e23: BivectorE23(self.e1 * rhs.0),
            e31: BivectorE31(self.e2 * rhs.0),
            e12: BivectorE12(self.e3 * rhs.0),
        }
    }
}
