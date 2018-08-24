use crate::elements::{
    Bivector, BivectorE12, BivectorE23, BivectorE31, Multivector, Pseudoscalar, Vector,
};

use std::ops::{BitXor, Mul};

impl Mul<Vector> for BivectorE12 {
    type Output = Multivector;

    fn mul(self, rhs: Vector) -> Multivector {
        Multivector {
            scalar: 0.0,
            vector: Vector {
                e1: self.0 * rhs.e2,
                e2: -self.0 * rhs.e1,
                e3: 0.0,
            },
            bivector: Bivector::zero(),
            pseudoscalar: Pseudoscalar(self.0 * rhs.e3),
        }
    }
}

impl Mul<Vector> for BivectorE23 {
    type Output = Multivector;

    fn mul(self, rhs: Vector) -> Multivector {
        Multivector {
            scalar: 0.0,
            vector: Vector {
                e1: 0.0,
                e2: rhs.e3 * self.0, //cd    //-bd  + cd  d*e23 (a e1 + b e2 + c e3 ) ,
                e3: -rhs.e2 * self.0, //-bd ,
            },
            bivector: Bivector::zero(),
            pseudoscalar: Pseudoscalar(self.0 * rhs.e1),
        }
    }
}

impl Mul<Vector> for BivectorE31 {
    type Output = Multivector;

    fn mul(self, rhs: Vector) -> Multivector {
        // d e31(a e1 + b e2 + ce3)
        // (ad e1e31 + bd e312 + cd e31e3)
        // (ad e3) + bd e123  -cd e1
        Multivector {
            scalar: 0.0,
            vector: Vector {
                e1: -rhs.e3 * self.0,
                e2: 0.0,
                e3: rhs.e1 * self.0,
            },
            bivector: Bivector::zero(),
            pseudoscalar: Pseudoscalar(self.0 * rhs.e2),
        }
    }
}

/////////////////////////

impl Mul<BivectorE12> for Vector {
    type Output = Multivector;

    fn mul(self, rhs: BivectorE12) -> Multivector {
        Multivector {
            scalar: 0.0,
            vector: Vector {
                e1: -rhs.0 * self.e2,
                e2: rhs.0 * self.e1,
                e3: 0.0,
            },
            bivector: Bivector::zero(),
            pseudoscalar: Pseudoscalar(self.e3 * rhs.0),
        }
    }
}

impl Mul<BivectorE23> for Vector {
    type Output = Multivector;

    fn mul(self, rhs: BivectorE23) -> Multivector {
        Multivector {
            scalar: 0.0,
            vector: Vector {
                e1: 0.0,
                e2: -self.e3 * rhs.0, //cd    //-bd  + cd  d*e23 (a e1 + b e2 + c e3 ) ,
                e3: self.e2 * rhs.0,  //-bd ,
            },
            bivector: Bivector::zero(),
            pseudoscalar: Pseudoscalar(self.e1 * rhs.0),
        }
    }
}

impl Mul<BivectorE31> for Vector {
    type Output = Multivector;

    fn mul(self, rhs: BivectorE31) -> Multivector {
        // (a e1 + b e2 + ce3) d e31
        // (ad e1e31 + bd e231 + cd e3e31)
        // (-ad e3) + bd e123 + cd e1

        Multivector {
            scalar: 0.0,
            vector: Vector {
                e1: self.e3 * rhs.0,
                e2: 0.0,
                e3: -self.e1 * rhs.0,
            },
            bivector: Bivector::zero(),
            pseudoscalar: Pseudoscalar(self.e2 * rhs.0),
        }
    }
}

impl Mul<Bivector> for Vector {
    type Output = Multivector;

    fn mul(self, rhs: Bivector) -> Multivector {
        self * rhs.e12 + self * rhs.e23 + self * rhs.e31
    }
}

impl BitXor<Bivector> for Vector {
    type Output = Pseudoscalar;
    fn bitxor(self, rhs: Bivector) -> Pseudoscalar {
        0.5 * (self * rhs + rhs * self).pseudoscalar
    }
}

impl BitXor<Vector> for Bivector {
    type Output = Pseudoscalar;
    fn bitxor(self, rhs: Vector) -> Pseudoscalar {
        // bivector wedge vector is commutative
        rhs ^ self
    }
}

impl Mul<Vector> for Bivector {
    type Output = Multivector;

    fn mul(self, rhs: Vector) -> Multivector {
        self.e12 * rhs + self.e23 * rhs + self.e31 * rhs
    }
}
