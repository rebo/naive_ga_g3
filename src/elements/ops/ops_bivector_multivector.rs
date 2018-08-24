use crate::elements::{Bivector, BivectorE12, BivectorE23, BivectorE31, Multivector};

use std::ops::{Add, Mul};

impl Add<BivectorE12> for Multivector {
    type Output = Multivector;

    fn add(self, rhs: BivectorE12) -> Self {
        self + Bivector::from(rhs)
    }
}

// fix as per above add triat
impl Add<Multivector> for BivectorE12 {
    type Output = Multivector;

    fn add(self, rhs: Multivector) -> Multivector {
        Bivector::from(self) + rhs
    }
}

impl Add<BivectorE23> for Multivector {
    type Output = Multivector;

    fn add(self, rhs: BivectorE23) -> Self {
        self + Bivector::from(rhs)
    }
}

// fix as per above add triat
impl Add<Multivector> for BivectorE23 {
    type Output = Multivector;

    fn add(self, rhs: Multivector) -> Multivector {
        Bivector::from(self) + rhs
    }
}

impl Add<BivectorE31> for Multivector {
    type Output = Multivector;

    fn add(self, rhs: BivectorE31) -> Self {
        self + Bivector::from(rhs)
    }
}

// fix as per above add triat
impl Add<Multivector> for BivectorE31 {
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

impl Mul<BivectorE12> for Multivector {
    type Output = Multivector;

    fn mul(self, rhs: BivectorE12) -> Self {
        self.bivector * rhs + self.vector * rhs + self.scalar * rhs
    }
}

impl Mul<Multivector> for BivectorE12 {
    type Output = Multivector;

    fn mul(self, rhs: Multivector) -> Multivector {
        self * rhs.bivector * rhs + self * rhs.vector + self * rhs.scalar
    }
}

impl Mul<BivectorE23> for Multivector {
    type Output = Multivector;

    fn mul(self, rhs: BivectorE23) -> Self {
        self.bivector * rhs + self.vector * rhs + self.scalar * rhs
    }
}

impl Mul<Multivector> for BivectorE23 {
    type Output = Multivector;

    fn mul(self, rhs: Multivector) -> Multivector {
        self * rhs.bivector * rhs + self * rhs.vector + self * rhs.scalar
    }
}

impl Mul<BivectorE31> for Multivector {
    type Output = Multivector;

    fn mul(self, rhs: BivectorE31) -> Self {
        self.bivector * rhs + self.vector * rhs + self.scalar * rhs
    }
}

impl Mul<Multivector> for BivectorE31 {
    type Output = Multivector;

    fn mul(self, rhs: Multivector) -> Multivector {
        self * rhs.bivector * rhs + self * rhs.vector + self * rhs.scalar
    }
}
