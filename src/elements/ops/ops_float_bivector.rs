use super::super::{
    Bivector, Bivector_e12, Bivector_e23, Bivector_e31, Multivector, Pseudoscalar, Vector,
};

use std::ops::{Add, BitXor, Mul, Neg, Sub};

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

impl Add<f64> for Bivector_e12 {
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

impl Add<Bivector_e12> for f64 {
    type Output = Multivector;

    fn add(self, rhs: Bivector_e12) -> Multivector {
        Multivector {
            scalar: self,
            vector: Vector::zero(),
            bivector: Bivector::from(rhs),
            pseudoscalar: Pseudoscalar::zero(),
        }
    }
}

impl Add<f64> for Bivector_e23 {
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

impl Add<Bivector_e23> for f64 {
    type Output = Multivector;

    fn add(self, rhs: Bivector_e23) -> Multivector {
        Multivector {
            scalar: self,
            vector: Vector::zero(),
            bivector: Bivector::from(rhs),
            pseudoscalar: Pseudoscalar::zero(),
        }
    }
}

impl Add<f64> for Bivector_e31 {
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

impl Add<Bivector_e31> for f64 {
    type Output = Multivector;

    fn add(self, rhs: Bivector_e31) -> Multivector {
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

impl Sub<f64> for Bivector_e12 {
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

impl Sub<Bivector_e12> for f64 {
    type Output = Multivector;

    fn sub(self, rhs: Bivector_e12) -> Multivector {
        Multivector {
            scalar: self,
            vector: Vector::zero(),
            bivector: -1.0 * Bivector::from(rhs),
            pseudoscalar: Pseudoscalar::zero(),
        }
    }
}

impl Sub<f64> for Bivector_e23 {
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

impl Sub<Bivector_e23> for f64 {
    type Output = Multivector;

    fn sub(self, rhs: Bivector_e23) -> Multivector {
        Multivector {
            scalar: self,
            vector: Vector::zero(),
            bivector: -1.0 * Bivector::from(rhs),
            pseudoscalar: Pseudoscalar::zero(),
        }
    }
}

impl Sub<f64> for Bivector_e31 {
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

impl Sub<Bivector_e31> for f64 {
    type Output = Multivector;

    fn sub(self, rhs: Bivector_e31) -> Multivector {
        Multivector {
            scalar: self,
            vector: Vector::zero(),
            bivector: -1.0 * Bivector::from(rhs),
            pseudoscalar: Pseudoscalar::zero(),
        }
    }
}

// MUL

impl Mul<Bivector_e12> for f64 {
    type Output = Bivector_e12;

    fn mul(self, rhs: Bivector_e12) -> Bivector_e12 {
        Bivector_e12(self * rhs.0)
    }
}

impl Mul<f64> for Bivector_e12 {
    type Output = Bivector_e12;

    fn mul(self, rhs: f64) -> Bivector_e12 {
        Bivector_e12(self.0 * rhs)
    }
}

impl Mul<Bivector_e23> for f64 {
    type Output = Bivector_e23;

    fn mul(self, rhs: Bivector_e23) -> Bivector_e23 {
        Bivector_e23(self * rhs.0)
    }
}

impl Mul<f64> for Bivector_e23 {
    type Output = Bivector_e23;

    fn mul(self, rhs: f64) -> Bivector_e23 {
        Bivector_e23(self.0 * rhs)
    }
}

impl Mul<Bivector_e31> for f64 {
    type Output = Bivector_e31;

    fn mul(self, rhs: Bivector_e31) -> Bivector_e31 {
        Bivector_e31(self * rhs.0)
    }
}

impl Mul<f64> for Bivector_e31 {
    type Output = Bivector_e31;

    fn mul(self, rhs: f64) -> Bivector_e31 {
        Bivector_e31(self.0 * rhs)
    }
}

impl Mul<f64> for Bivector {
    type Output = Bivector;

    fn mul(self, rhs: f64) -> Bivector {
        Self {
            e12: Bivector_e12(self.e12.0 * rhs),
            e23: Bivector_e23(self.e23.0 * rhs),
            e31: Bivector_e31(self.e31.0 * rhs),
        }
    }
}

impl Mul<Bivector> for f64 {
    type Output = Bivector;

    fn mul(self, rhs: Bivector) -> Bivector {
        Bivector {
            e12: Bivector_e12(rhs.e12.0 * self),
            e23: Bivector_e23(rhs.e23.0 * self),
            e31: Bivector_e31(rhs.e31.0 * self),
        }
    }
}
