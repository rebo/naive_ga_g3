use super::vector::Vector;

// pub struct Bivector(pub f64);

#[derive(Clone, Copy, Debug)]
pub struct Bivector_e12(pub f64);
impl Bivector_e12 {
    pub fn zero() -> Self {
        Bivector_e12(0.0)
    }

    pub fn unit() -> Self {
        Bivector_e12(1.0)
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Bivector_e23(pub f64);
impl Bivector_e23 {
    pub fn zero() -> Self {
        Bivector_e23(0.0)
    }
    pub fn unit() -> Self {
        Bivector_e23(1.0)
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Bivector_e31(pub f64);
impl Bivector_e31 {
    pub fn zero() -> Self {
        Bivector_e31(0.0)
    }

    pub fn unit() -> Self {
        Bivector_e31(1.0)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Bivector {
    pub e12: Bivector_e12,
    pub e23: Bivector_e23,
    pub e31: Bivector_e31,
}

impl Bivector {
    // pub fn unit() -> Bivector {
    //     Bivector(1.0)
    // }

    pub fn zero() -> Self {
        Self {
            e12: Bivector_e12(0.0),
            e23: Bivector_e23(0.0),
            e31: Bivector_e31(0.0),
        }
    }

    pub fn rev(self) -> Bivector {
        // let k = 2;
        // (-1.0f64).powi((k * (k - 1)) / 2) * self
        -1.0 * self
    }

    // pub fn dot_with_vector(self, rhs: Vector) -> Vector {
    //     self * rhs
    // }
}

impl std::convert::From<Bivector_e12> for Bivector {
    fn from(be12: Bivector_e12) -> Self {
        Bivector {
            e12: be12,
            e23: Bivector_e23::zero(),
            e31: Bivector_e31::zero(),
        }
    }
}

impl std::convert::From<Bivector_e23> for Bivector {
    fn from(be23: Bivector_e23) -> Self {
        Bivector {
            e12: Bivector_e12::zero(),
            e23: be23,
            e31: Bivector_e31::zero(),
        }
    }
}

impl std::convert::From<Bivector_e31> for Bivector {
    fn from(be31: Bivector_e31) -> Self {
        Bivector {
            e12: Bivector_e12::zero(),
            e23: Bivector_e23::zero(),
            e31: be31,
        }
    }
}
