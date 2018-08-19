use super::super::{
    Bivector, Bivector_e12, Bivector_e23, Bivector_e31, Multivector, Pseudoscalar, Vector,
};

use std::ops::{Add, BitXor, Mul, Neg};

impl Mul<Vector> for Bivector_e12 {
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

impl Mul<Vector> for Bivector_e23 {
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

impl Mul<Vector> for Bivector_e31 {
    type Output = Multivector;

    fn mul(self, rhs: Vector) -> Multivector {
        Multivector {
            scalar: 0.0,
            vector: Vector {
                e1: -rhs.e3 * self.0,
                e2: 0.0,
                e3: rhs.e2 * self.0,
            },
            bivector: Bivector::zero(),
            pseudoscalar: Pseudoscalar(self.0 * rhs.e2),
        }
    }
}

/////////////////////////

impl Mul<Bivector_e12> for Vector {
    type Output = Multivector;

    fn mul(self, rhs: Bivector_e12) -> Multivector {
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

impl Mul<Bivector_e23> for Vector {
    type Output = Multivector;

    fn mul(self, rhs: Bivector_e23) -> Multivector {
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

impl Mul<Bivector_e31> for Vector {
    type Output = Multivector;

    fn mul(self, rhs: Bivector_e31) -> Multivector {
        Multivector {
            scalar: 0.0,
            vector: Vector {
                e1: self.e3 * rhs.0,
                e2: 0.0,
                e3: -self.e2 * rhs.0,
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

impl Mul<Vector> for Bivector {
    type Output = Multivector;

    fn mul(self, rhs: Vector) -> Multivector {
        self.e12 * rhs + self.e23 * rhs + self.e31 * rhs
    }
}
