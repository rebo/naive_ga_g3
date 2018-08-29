use super::bivector::Bivector;
use super::multivector::Multivector;
use super::pseudoscalar::Pseudoscalar;
use super::vector::Vector;

use float_cmp::ApproxEq;

// A Rotor is a multivector with only scalar and bivector part (and magnitute 1)
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
        (v.normalize() * u.normalize()).into()
    }

    pub fn rev(self) -> Rotor {
        Self {
            scalar: self.scalar,
            bivector: self.bivector.rev(),
        }
    }

    pub fn from_exp(half_angle: f64, bivector: Bivector) -> Self {
        assert!(
            bivector.mag2().approx_eq(&1.0, 2.0 * std::f64::EPSILON, 2),
            "Bivector not unit size"
        );

        (half_angle.cos() - half_angle.sin() * bivector).into()
    }

    pub fn half_angle(self) -> f64 {
        self.scalar.acos()
    }
}

impl std::convert::From<Multivector> for Rotor {
    fn from(m: Multivector) -> Self {
        println!("{:#?}", m);
        assert!(m.is_unit_size(), "Multivector has non unit size");
        assert!(m.vector.is_zero(), "Vector part not zero");
        assert!(m.pseudoscalar.is_zero(), "Pseudoscalar part not zero");
        Rotor {
            scalar: m.scalar,
            bivector: m.bivector,
        }
    }
}

// conversions to a general multivector
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
