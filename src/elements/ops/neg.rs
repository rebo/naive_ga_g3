use super::super::{
    Bivector, BivectorE12, BivectorE23, BivectorE31, Multivector, Pseudoscalar, Vector,
};

use std::ops::{Add, BitXor, Mul, Neg};
// Multiplication operations

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self {
        Self {
            e1: -self.e1,
            e2: -self.e2,
        }
    }
}

impl Neg for Bivector {
    type Output = Bivector;

    fn neg(self) -> Self {
        Self {
            e12: BivectorE12(-self.e12.0),
            e23: BivectorE23(-self.e23.0),
            e31: BivectorE31(-self.e31.0),
        }
    }
}

impl Neg for Multivector {
    type Output = Multivector;

    fn neg(self) -> Self {
        Self {
            scalar: -self.scalar,
            vector: -self.vector,
            bivector: -self.bivector,
            pseudoscalar: -self.pseudoscalar,
        }
    }
}

impl Neg for Pseudoscalar {
    type Output = Pseudoscalar;

    fn neg(self) -> Self {
        Pseudoscalar(-self.0)
    }
}
