#![allow(suspicious_arithmetic_impl)]
use super::super::{
    Bivector, Bivector_e12, Bivector_e23, Bivector_e31, Multivector, Pseudoscalar, Rotor, Vector,
};

use std::ops::Mul;
// Multiplication operations

// bivector mul scalar

// vectors mul bivector

// vector mul Rotor

impl Mul<Rotor> for Vector {
    type Output = Multivector;

    fn mul(self, rhs: Rotor) -> Multivector {
        let rhs: Multivector = rhs.into();

        self * rhs
    }
}

impl Mul<Vector> for Rotor {
    type Output = Multivector;

    fn mul(self, rhs: Vector) -> Multivector {
        let m: Multivector = self.into();

        m * rhs
    }
}

//pseudoscalar mul vector

// bivector mul multivector

// Vectors mul vectors

// multivector mul multivector

// multivector mul multivector
impl Mul<Rotor> for Multivector {
    type Output = Multivector;

    fn mul(self, rhs: Rotor) -> Self {
        let rhs: Multivector = rhs.into();
        self * rhs
    }
}

impl Mul<Multivector> for Rotor {
    type Output = Multivector;

    fn mul(self, rhs: Multivector) -> Multivector {
        let m: Multivector = self.into();
        rhs * m
    }
}
