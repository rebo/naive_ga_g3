// general bivector blade with basis vectors e12, e23, and e31.
use super::vector::Vector;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bivector {
    pub e12: BivectorE12,
    pub e23: BivectorE23,
    pub e31: BivectorE31,
}

impl Bivector {
    pub fn new(e12: f64, e23: f64, e31: f64) -> Bivector {
        Bivector {
            e12: BivectorE12(e12),
            e23: BivectorE23(e23),
            e31: BivectorE31(e31),
        }
    }

    pub fn zero() -> Self {
        Self {
            e12: BivectorE12(0.0),
            e23: BivectorE23(0.0),
            e31: BivectorE31(0.0),
        }
    }

    pub fn dot(self, rhs: Vector) -> Vector {
        // v dot B = - B dot v
        // vector dot bivector is anti commutative
        -1.0 * rhs.dot(self)
    }

    pub fn mag2(self) -> f64 {
        self.e12.0 * self.e12.0 + self.e23.0 * self.e23.0 + self.e31.0 * self.e31.0
    }

    pub fn rev(self) -> Bivector {
        // let k = 2;
        // (-1.0f64).powi((k * (k - 1)) / 2) * self
        -1.0 * self
    }

    // TODO : Need to update to 3d.
    // pub fn dot_with_vector(self, rhs: Vector) -> Vector {
    //     self * rhs
    // }
}

// Basis bivectors e12, e23, and e31.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BivectorE12(pub f64);
impl BivectorE12 {
    pub fn zero() -> Self {
        BivectorE12(0.0)
    }

    pub fn unit() -> Self {
        BivectorE12(1.0)
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BivectorE23(pub f64);
impl BivectorE23 {
    pub fn zero() -> Self {
        BivectorE23(0.0)
    }
    pub fn unit() -> Self {
        BivectorE23(1.0)
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BivectorE31(pub f64);
impl BivectorE31 {
    pub fn zero() -> Self {
        BivectorE31(0.0)
    }

    pub fn unit() -> Self {
        BivectorE31(1.0)
    }
}

// Conversion methods from basis bivectors to full bivector
impl std::convert::From<BivectorE12> for Bivector {
    fn from(be12: BivectorE12) -> Self {
        Bivector {
            e12: be12,
            e23: BivectorE23::zero(),
            e31: BivectorE31::zero(),
        }
    }
}

impl std::convert::From<BivectorE23> for Bivector {
    fn from(be23: BivectorE23) -> Self {
        Bivector {
            e12: BivectorE12::zero(),
            e23: be23,
            e31: BivectorE31::zero(),
        }
    }
}

impl std::convert::From<BivectorE31> for Bivector {
    fn from(be31: BivectorE31) -> Self {
        Bivector {
            e12: BivectorE12::zero(),
            e23: BivectorE23::zero(),
            e31: be31,
        }
    }
}
