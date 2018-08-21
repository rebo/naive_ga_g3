use super::super::{
    Bivector, BivectorE12, BivectorE23, BivectorE31, Multivector, Pseudoscalar, Vector,
};

use std::ops::Neg;
// Multiplication operations

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self {
        Self {
            e1: -self.e1,
            e2: -self.e2,
            e3: -self.e3,
        }
    }
}

impl Neg for Bivector {
    type Output = Bivector;

    fn neg(self) -> Bivector {
        Bivector {
            e12: -self.e12,
            e23: -self.e23,
            e31: -self.e31,
        }
    }
}

impl Neg for BivectorE12 {
    type Output = BivectorE12;

    fn neg(self) -> BivectorE12 {
        BivectorE12(-self.0)
    }
}

impl Neg for BivectorE23 {
    type Output = BivectorE23;

    fn neg(self) -> BivectorE23 {
        BivectorE23(-self.0)
    }
}

impl Neg for BivectorE31 {
    type Output = BivectorE31;

    fn neg(self) -> BivectorE31 {
        BivectorE31(-self.0)
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
