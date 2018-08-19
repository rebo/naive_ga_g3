use super::super::{
    Bivector, Bivector_e12, Bivector_e23, Bivector_e31, Multivector, Pseudoscalar, Vector,
};

use std::ops::{Add, BitXor, Mul, Neg};

impl Add<Bivector_e12> for Multivector {
    type Output = Multivector;

    fn add(self, rhs: Bivector_e12) -> Self {
        self + Bivector::from(rhs)
    }
}

// fix as per above add triat
impl Add<Multivector> for Bivector_e12 {
    type Output = Multivector;

    fn add(self, rhs: Multivector) -> Multivector {
        Bivector::from(self) + rhs
    }
}

impl Add<Bivector_e23> for Multivector {
    type Output = Multivector;

    fn add(self, rhs: Bivector_e23) -> Self {
        self + Bivector::from(rhs)
    }
}

// fix as per above add triat
impl Add<Multivector> for Bivector_e23 {
    type Output = Multivector;

    fn add(self, rhs: Multivector) -> Multivector {
        Bivector::from(self) + rhs
    }
}

impl Add<Bivector_e31> for Multivector {
    type Output = Multivector;

    fn add(self, rhs: Bivector_e31) -> Self {
        self + Bivector::from(rhs)
    }
}

// fix as per above add triat
impl Add<Multivector> for Bivector_e31 {
    type Output = Multivector;

    fn add(self, rhs: Multivector) -> Multivector {
        Bivector::from(self) + rhs
    }
}

impl Add<Bivector> for Multivector {
    type Output = Multivector;

    fn add(self, rhs: Bivector) -> Self {
        Multivector {
            scalar: self.scalar,
            vector: self.vector,
            bivector: self.bivector + rhs,
            pseudoscalar: self.pseudoscalar,
        }
    }
}

// fix as per above add triat
impl Add<Multivector> for Bivector {
    type Output = Multivector;

    fn add(self, rhs: Multivector) -> Multivector {
        Multivector {
            scalar: rhs.scalar,
            vector: rhs.vector,
            bivector: self + rhs.bivector,
            pseudoscalar: rhs.pseudoscalar,
        }
    }
}

impl Mul<Bivector> for Multivector {
    type Output = Multivector;

    fn mul(self, rhs: Bivector) -> Self {
        self.bivector * rhs + self.vector * rhs + self.scalar * rhs
    }
}

impl Mul<Multivector> for Bivector {
    type Output = Multivector;

    fn mul(self, rhs: Multivector) -> Multivector {
        self * rhs.bivector * rhs + self * rhs.vector + self * rhs.scalar
    }
}

///
///
///

impl Mul<Bivector_e12> for Multivector {
    type Output = Multivector;

    fn mul(self, rhs: Bivector_e12) -> Self {
        self.bivector * rhs + self.vector * rhs + self.scalar * rhs
    }
}

impl Mul<Multivector> for Bivector_e12 {
    type Output = Multivector;

    fn mul(self, rhs: Multivector) -> Multivector {
        self * rhs.bivector * rhs + self * rhs.vector + self * rhs.scalar
    }
}

impl Mul<Bivector_e23> for Multivector {
    type Output = Multivector;

    fn mul(self, rhs: Bivector_e23) -> Self {
        self.bivector * rhs + self.vector * rhs + self.scalar * rhs
    }
}

impl Mul<Multivector> for Bivector_e23 {
    type Output = Multivector;

    fn mul(self, rhs: Multivector) -> Multivector {
        self * rhs.bivector * rhs + self * rhs.vector + self * rhs.scalar
    }
}

impl Mul<Bivector_e31> for Multivector {
    type Output = Multivector;

    fn mul(self, rhs: Bivector_e31) -> Self {
        self.bivector * rhs + self.vector * rhs + self.scalar * rhs
    }
}

impl Mul<Multivector> for Bivector_e31 {
    type Output = Multivector;

    fn mul(self, rhs: Multivector) -> Multivector {
        self * rhs.bivector * rhs + self * rhs.vector + self * rhs.scalar
    }
}
