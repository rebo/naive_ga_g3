use crate::elements::{Multivector, Rotor, Vector};

use std::ops::{Add, Mul};

impl Add<Multivector> for Vector {
    type Output = Multivector;

    fn add(self, rhs: Multivector) -> Multivector {
        Multivector::from(self) + rhs
    }
}

impl Add<Vector> for Multivector {
    type Output = Multivector;

    fn add(self, rhs: Vector) -> Multivector {
        self + Multivector::from(rhs)
    }
}

impl Mul<Multivector> for Vector {
    type Output = Multivector;

    fn mul(self, rhs: Multivector) -> Multivector {
        Multivector::from(self * rhs.scalar)
            + self * rhs.vector
            + self * rhs.bivector
            + self * rhs.pseudoscalar
    }
}

impl Mul<Vector> for Multivector {
    type Output = Multivector;

    fn mul(self, rhs: Vector) -> Multivector {
        Multivector::from(self.scalar * rhs)
            + self.vector * rhs
            + self.bivector * rhs
            + self.pseudoscalar * rhs
    }
}
// multivector mul multivector
