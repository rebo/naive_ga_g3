use crate::elements::{
    Bivector, BivectorE12, BivectorE23, BivectorE31, Multivector, Pseudoscalar, Vector,
};

use std::ops::{Add, Mul, Sub};

impl Add<f64> for Bivector {
    type Output = Multivector;

    fn add(self, rhs: f64) -> Multivector {
        Multivector {
            scalar: rhs,
            vector: Vector::zero(),
            bivector: self,
            pseudoscalar: Pseudoscalar::zero(),
        }
    }
}

impl Add<Bivector> for f64 {
    type Output = Multivector;

    fn add(self, rhs: Bivector) -> Multivector {
        Multivector {
            scalar: self,
            vector: Vector::zero(),
            bivector: rhs,
            pseudoscalar: Pseudoscalar::zero(),
        }
    }
}

impl Add<f64> for BivectorE12 {
    type Output = Multivector;

    fn add(self, rhs: f64) -> Multivector {
        Multivector {
            scalar: rhs,
            vector: Vector::zero(),
            bivector: Bivector::from(self),
            pseudoscalar: Pseudoscalar::zero(),
        }
    }
}

impl Add<BivectorE12> for f64 {
    type Output = Multivector;

    fn add(self, rhs: BivectorE12) -> Multivector {
        Multivector {
            scalar: self,
            vector: Vector::zero(),
            bivector: Bivector::from(rhs),
            pseudoscalar: Pseudoscalar::zero(),
        }
    }
}

impl Add<f64> for BivectorE23 {
    type Output = Multivector;

    fn add(self, rhs: f64) -> Multivector {
        Multivector {
            scalar: rhs,
            vector: Vector::zero(),
            bivector: Bivector::from(self),
            pseudoscalar: Pseudoscalar::zero(),
        }
    }
}

impl Add<BivectorE23> for f64 {
    type Output = Multivector;

    fn add(self, rhs: BivectorE23) -> Multivector {
        Multivector {
            scalar: self,
            vector: Vector::zero(),
            bivector: Bivector::from(rhs),
            pseudoscalar: Pseudoscalar::zero(),
        }
    }
}

impl Add<f64> for BivectorE31 {
    type Output = Multivector;

    fn add(self, rhs: f64) -> Multivector {
        Multivector {
            scalar: rhs,
            vector: Vector::zero(),
            bivector: Bivector::from(self),
            pseudoscalar: Pseudoscalar::zero(),
        }
    }
}

impl Add<BivectorE31> for f64 {
    type Output = Multivector;

    fn add(self, rhs: BivectorE31) -> Multivector {
        Multivector {
            scalar: self,
            vector: Vector::zero(),
            bivector: Bivector::from(rhs),
            pseudoscalar: Pseudoscalar::zero(),
        }
    }
}
//Neg
impl Sub<f64> for Bivector {
    type Output = Multivector;

    fn sub(self, rhs: f64) -> Multivector {
        Multivector {
            scalar: -rhs,
            vector: Vector::zero(),
            bivector: self,
            pseudoscalar: Pseudoscalar::zero(),
        }
    }
}

impl Sub<Bivector> for f64 {
    type Output = Multivector;

    fn sub(self, rhs: Bivector) -> Multivector {
        Multivector {
            scalar: self,
            vector: Vector::zero(),
            bivector: -1.0 * rhs,
            pseudoscalar: Pseudoscalar::zero(),
        }
    }
}

impl Sub<f64> for BivectorE12 {
    type Output = Multivector;

    fn sub(self, rhs: f64) -> Multivector {
        Multivector {
            scalar: -rhs,
            vector: Vector::zero(),
            bivector: Bivector::from(self),
            pseudoscalar: Pseudoscalar::zero(),
        }
    }
}

impl Sub<BivectorE12> for f64 {
    type Output = Multivector;

    fn sub(self, rhs: BivectorE12) -> Multivector {
        Multivector {
            scalar: self,
            vector: Vector::zero(),
            bivector: -1.0 * Bivector::from(rhs),
            pseudoscalar: Pseudoscalar::zero(),
        }
    }
}

impl Sub<f64> for BivectorE23 {
    type Output = Multivector;

    fn sub(self, rhs: f64) -> Multivector {
        Multivector {
            scalar: -rhs,
            vector: Vector::zero(),
            bivector: Bivector::from(self),
            pseudoscalar: Pseudoscalar::zero(),
        }
    }
}

impl Sub<BivectorE23> for f64 {
    type Output = Multivector;

    fn sub(self, rhs: BivectorE23) -> Multivector {
        Multivector {
            scalar: self,
            vector: Vector::zero(),
            bivector: -1.0 * Bivector::from(rhs),
            pseudoscalar: Pseudoscalar::zero(),
        }
    }
}

impl Sub<f64> for BivectorE31 {
    type Output = Multivector;

    fn sub(self, rhs: f64) -> Multivector {
        Multivector {
            scalar: -rhs,
            vector: Vector::zero(),
            bivector: Bivector::from(self),
            pseudoscalar: Pseudoscalar::zero(),
        }
    }
}

impl Sub<BivectorE31> for f64 {
    type Output = Multivector;

    fn sub(self, rhs: BivectorE31) -> Multivector {
        Multivector {
            scalar: self,
            vector: Vector::zero(),
            bivector: -1.0 * Bivector::from(rhs),
            pseudoscalar: Pseudoscalar::zero(),
        }
    }
}

// MUL

impl Mul<BivectorE12> for f64 {
    type Output = BivectorE12;

    fn mul(self, rhs: BivectorE12) -> BivectorE12 {
        BivectorE12(self * rhs.0)
    }
}

impl Mul<f64> for BivectorE12 {
    type Output = BivectorE12;

    fn mul(self, rhs: f64) -> BivectorE12 {
        BivectorE12(self.0 * rhs)
    }
}

impl Mul<BivectorE23> for f64 {
    type Output = BivectorE23;

    fn mul(self, rhs: BivectorE23) -> BivectorE23 {
        BivectorE23(self * rhs.0)
    }
}

impl Mul<f64> for BivectorE23 {
    type Output = BivectorE23;

    fn mul(self, rhs: f64) -> BivectorE23 {
        BivectorE23(self.0 * rhs)
    }
}

impl Mul<BivectorE31> for f64 {
    type Output = BivectorE31;

    fn mul(self, rhs: BivectorE31) -> BivectorE31 {
        BivectorE31(self * rhs.0)
    }
}

impl Mul<f64> for BivectorE31 {
    type Output = BivectorE31;

    fn mul(self, rhs: f64) -> BivectorE31 {
        BivectorE31(self.0 * rhs)
    }
}

impl Mul<f64> for Bivector {
    type Output = Bivector;

    fn mul(self, rhs: f64) -> Bivector {
        Self {
            e12: BivectorE12(self.e12.0 * rhs),
            e23: BivectorE23(self.e23.0 * rhs),
            e31: BivectorE31(self.e31.0 * rhs),
        }
    }
}

impl Mul<Bivector> for f64 {
    type Output = Bivector;

    fn mul(self, rhs: Bivector) -> Bivector {
        Bivector {
            e12: BivectorE12(rhs.e12.0 * self),
            e23: BivectorE23(rhs.e23.0 * self),
            e31: BivectorE31(rhs.e31.0 * self),
        }
    }
}
