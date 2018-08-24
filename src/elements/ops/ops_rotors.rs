use crate::elements::{Bivector, Multivector, Rotor, Vector};

use std::ops::Mul;

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

impl Mul<Rotor> for Bivector {
    type Output = Multivector;

    fn mul(self, rhs: Rotor) -> Multivector {
        let rotor = Multivector::from(rhs);
        let bivector = Multivector::from(self);
        bivector * rotor
    }
}

impl Mul<Bivector> for Rotor {
    type Output = Multivector;

    fn mul(self, rhs: Bivector) -> Multivector {
        let rotor = Multivector::from(self);
        let bivector = Multivector::from(rhs);

        rotor * bivector
    }
}
