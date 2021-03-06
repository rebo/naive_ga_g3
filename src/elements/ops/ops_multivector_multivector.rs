use crate::elements::{Multivector, Rotor};

use std::ops::{Add, Mul, Sub};

impl Add for Multivector {
    type Output = Multivector;

    fn add(self, rhs: Multivector) -> Self {
        Multivector {
            scalar: self.scalar + rhs.scalar,
            vector: self.vector + rhs.vector,
            bivector: self.bivector + rhs.bivector,
            pseudoscalar: self.pseudoscalar + rhs.pseudoscalar,
        }
    }
}

impl Sub for Multivector {
    type Output = Multivector;

    fn sub(self, rhs: Multivector) -> Self {
        Multivector {
            scalar: self.scalar - rhs.scalar,
            vector: self.vector - rhs.vector,
            bivector: self.bivector - rhs.bivector,
            pseudoscalar: self.pseudoscalar - rhs.pseudoscalar,
        }
    }
}

impl Mul for Multivector {
    type Output = Multivector;
    #[rustfmt::skip]
    fn mul(self, rhs: Multivector) -> Self {
        self.scalar * rhs.scalar + self.vector * rhs.scalar + self.bivector * rhs.scalar + self.pseudoscalar * rhs.scalar +
        self.scalar * rhs.vector + self.vector * rhs.vector + self.bivector * rhs.vector + self.pseudoscalar * rhs.vector +
        self.scalar * rhs.bivector + self.vector * rhs.bivector + self.bivector * rhs.bivector + self.pseudoscalar * rhs.bivector +
        self.scalar * rhs.pseudoscalar + self.vector * rhs.pseudoscalar + self.bivector * rhs.pseudoscalar + self.pseudoscalar * rhs.pseudoscalar
    }
}

// multivector mul multivector
impl Mul<Rotor> for Multivector {
    type Output = Multivector;

    fn mul(self, rhs: Rotor) -> Multivector {
        let rotor = Multivector::from(rhs);
        let vector = self;
        vector * rotor
    }
}

impl Mul<Multivector> for Rotor {
    type Output = Multivector;

    fn mul(self, rhs: Multivector) -> Multivector {
        let rotor = Multivector::from(self);
        let vector = rhs;
        rotor * vector
    }
}
