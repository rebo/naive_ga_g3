use super::super::{
    Bivector, Bivector_e12, Bivector_e23, Bivector_e31, Multivector, Pseudoscalar, Rotor, Vector,
};

use std::ops::{Add, BitXor, Mul, Neg};

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
            + Multivector::from(self * rhs.vector)
            + Multivector::from(self * rhs.bivector)
            + Multivector::from(self * rhs.pseudoscalar)
    }
}

impl Mul<Vector> for Multivector {
    type Output = Multivector;

    fn mul(self, rhs: Vector) -> Multivector {
        Multivector::from(self.scalar * rhs)
            + Multivector::from(self.vector * rhs)
            + Multivector::from(self.bivector * rhs)
            + Multivector::from(self.pseudoscalar * rhs)
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
