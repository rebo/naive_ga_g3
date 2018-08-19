#![allow(suspicious_arithmetic_impl)]

use super::super::{Bivector, BivectorE12, BivectorE23, BivectorE31, Multivector, Vector};

use std::ops::{Add, BitXor, Mul, Sub};

impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Vector {
        Vector {
            e1: self.e1 + rhs.e1,
            e2: self.e2 + rhs.e2,
            e3: self.e3 + rhs.e3,
        }
    }
}

impl Mul<Vector> for Vector {
    type Output = Multivector;

    fn mul(self, rhs: Vector) -> Multivector {
        let dot_product = self.dot(rhs);
        let wedge_product = self ^ rhs;
        (dot_product + wedge_product)
    }
}

impl BitXor<Vector> for Vector {
    type Output = Bivector;
    #[rustfmt::skip]
    fn bitxor(self, rhs: Vector) -> Bivector {


        // (a1e1 + a2e2)^(b1e1 + b2e2)
        // a1b1e1^e1 + a1b2e1^e2 + a2b1e2^e1 + a2b2e2^e2
        // 0         + a1b2e1^e2 - a2b1e1^e2 + 0
        // (a1b2 - a2b1 ) e1^e2
        // Bivector(self.e1 * rhs.e2 - self.e2 * rhs.e1)

        // (ae1^de1 + be2^de1 + ce3^de1)
        // (ae1^ee2 + be2^ee2 + ce3^ee2)
        // (ae1^fe3 + be2^fe3 + ce3^fe3))

        let b0 = Bivector::zero();
        let bde21 = Bivector::from(BivectorE12(self.e2*rhs.e1));
        let cde31 = Bivector::from(BivectorE31(self.e3*rhs.e1));
        let aee12 = Bivector::from(BivectorE12(self.e1*rhs.e2));
        let cee32 = Bivector::from(BivectorE23(self.e3*rhs.e2));
        let afe13 = Bivector::from(BivectorE31(self.e1*rhs.e3));
        let bfe23 = Bivector::from(BivectorE23(self.e2*rhs.e3));
        
             b0        -   bde21     +   cde31 +
             aee12     +   b0        -   cee32 +
             -afe13    +   bfe23     +   b0
    }
}
impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector {
            e1: self.e1 - rhs.e1,
            e2: self.e2 - rhs.e2,
            e3: self.e3 - rhs.e3,
        }
    }
}
