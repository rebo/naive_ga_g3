use super::bivector::Bivector;
use super::pseudoscalar::Pseudoscalar;
use super::vector::Vector;

use float_cmp::ApproxEq;

#[derive(Clone, Copy, Debug)]
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
}

#[derive(Debug, Clone, Copy)]
pub struct Rotor {
    scalar: f64,
    bivector: Bivector,
}

impl Rotor {
    pub fn scalar(&self) -> f64 {
        self.scalar
    }

    pub fn bivector(&self) -> Bivector {
        self.bivector
    }

    pub fn new_from_u_v(u: Vector, v: Vector) -> Rotor {
        Rotor::from(u.normalize() * v.normalize())
    }

    pub fn rev(self) -> Rotor {
        Self {
            scalar: self.scalar,
            bivector: self.bivector.rev(),
        }
    }
}

impl std::convert::From<Multivector> for Rotor {
    fn from(m: Multivector) -> Self {
        if !(m.magnitude().approx_eq(&1.0, 2.0 * std::f64::EPSILON, 2)) {
            panic!("Multivector has non unit size")
        }

        if !(m.vector.is_zero()) {
            panic!("vector part is not zero")
        }

        if !(m.pseudoscalar.is_zero()) {
            panic!("vector part is not zero")
        }

        Rotor {
            scalar: m.scalar,
            bivector: m.bivector,
        }
    }
}

impl std::convert::From<Rotor> for Multivector {
    fn from(rotor: Rotor) -> Self {
        Self {
            scalar: rotor.scalar,
            vector: Vector::zero(),
            bivector: rotor.bivector,
            pseudoscalar: Pseudoscalar::zero(),
        }
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
