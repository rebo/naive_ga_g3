use crate::elements::{Bivector, Pseudoscalar, Vector};
use float_cmp::ApproxEq;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Multivector {
    pub scalar: f64,
    pub vector: Vector,
    pub bivector: Bivector,
    pub pseudoscalar: Pseudoscalar,
}

impl Multivector {
    pub fn rev(self) -> Multivector {
        Self {
            scalar: self.scalar,
            vector: self.vector.rev(),
            bivector: self.bivector.rev(),
            pseudoscalar: self.pseudoscalar.rev(),
        }
    }
    pub fn zero() -> Multivector {
        Multivector {
            scalar: 0.0,
            vector: Vector::zero(),
            bivector: Bivector::zero(),
            pseudoscalar: Pseudoscalar::zero(),
        }
    }

    pub fn is_bivector(self) -> bool {
        self.scalar_is_zero() && self.vector_is_zero() && self.pseudoscalar_is_zero()
    }

    pub fn scalar_is_zero(self) -> bool {
        self.scalar.approx_eq(&0.0, 2.0 * std::f64::EPSILON, 2)
    }

    pub fn vector_is_zero(self) -> bool {
        self.vector.is_zero()
    }

    pub fn bivector_is_zero(self) -> bool {
        self.bivector.is_zero()
    }

    pub fn pseudoscalar_is_zero(self) -> bool {
        self.pseudoscalar.is_zero()
    }

    pub fn is_unit_size(&self) -> bool {
        self.mag2().approx_eq(&1.0, 2.0 * std::f64::EPSILON, 2)
    }

    pub fn magnitude(&self) -> f64 {
        let s = self.scalar;
        let ve1 = self.vector.e1;
        let ve2 = self.vector.e2;
        let be12 = self.bivector.e12.0;
        let be23 = self.bivector.e23.0;
        let be31 = self.bivector.e31.0;
        let p = self.pseudoscalar.0;

        (s * s + ve1 * ve1 + ve2 * ve2 + be12 * be12 + be23 * be23 + be31 * be31 + p * p).powf(0.5)
    }

    pub fn mag2(&self) -> f64 {
        let s = self.scalar;
        let ve1 = self.vector.e1;
        let ve2 = self.vector.e2;
        let be12 = self.bivector.e12.0;
        let be23 = self.bivector.e23.0;
        let be31 = self.bivector.e31.0;
        let p = self.pseudoscalar.0;

        s * s + ve1 * ve1 + ve2 * ve2 + be12 * be12 + be23 * be23 + be31 * be31 + p * p
    }
}

impl std::convert::From<Vector> for Multivector {
    fn from(vector: Vector) -> Self {
        Self {
            scalar: 0.0,
            vector: vector,
            bivector: Bivector::zero(),
            pseudoscalar: Pseudoscalar::zero(),
        }
    }
}

impl std::convert::From<f64> for Multivector {
    fn from(scalar: f64) -> Self {
        Self {
            scalar: scalar,
            vector: Vector::zero(),
            bivector: Bivector::zero(),
            pseudoscalar: Pseudoscalar::zero(),
        }
    }
}

impl std::convert::From<Bivector> for Multivector {
    fn from(bivector: Bivector) -> Self {
        Self {
            scalar: 0.0,
            vector: Vector::zero(),
            bivector: bivector,
            pseudoscalar: Pseudoscalar::zero(),
        }
    }
}

impl std::convert::From<Pseudoscalar> for Multivector {
    fn from(pseudoscalar: Pseudoscalar) -> Self {
        Self {
            scalar: 0.0,
            vector: Vector::zero(),
            bivector: Bivector::zero(),
            pseudoscalar: pseudoscalar,
        }
    }
}
