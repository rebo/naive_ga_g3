use super::super::{Multivector, Rotor, Vector};

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
impl Mul<Rotor> for Vector {
    type Output = Multivector;

    fn mul(self, rhs: Rotor) -> Multivector {
        let rotor = Multivector::from(rhs);
        let vector = Multivector::from(self);
        vector * rotor
    }
}

impl Mul<Vector> for Rotor {
    type Output = Multivector;

    fn mul(self, rhs: Vector) -> Multivector {
        let rotor = Multivector::from(self);
        let vector = Multivector::from(rhs);
        rotor * vector
    }
}
