use super::super::{
    Bivector, Bivector_e12, Bivector_e23, Bivector_e31, Multivector, Pseudoscalar, Vector,
};

use std::ops::{Add, BitXor, Mul, Neg};

impl Mul<Bivector> for Pseudoscalar {
    type Output = Vector;

    fn mul(self, rhs: Bivector) -> Vector {
        self * rhs.e12 + self * rhs.e23 + self * rhs.e31
    }
}

impl Mul<Pseudoscalar> for Bivector {
    type Output = Vector;

    fn mul(self, rhs: Pseudoscalar) -> Vector {
        self.e12 * rhs + self.e23 * rhs + self.e31 * rhs
    }
}

impl Mul<Bivector_e12> for Pseudoscalar {
    type Output = Vector;

    fn mul(self, rhs: Bivector_e12) -> Vector {
        // ABe123e12
        // ABe12312
        // -ABe12321
        // +ABe12231
        // ABe131
        // -ABe113
        Vector {
            e1: 0.0,
            e2: 0.0,
            e3: -self.0 * rhs.0,
        }
    }
}

impl Mul<Bivector_e23> for Pseudoscalar {
    type Output = Vector;

    fn mul(self, rhs: Bivector_e23) -> Vector {
        // ABe123e23
        // ABe12323
        // -ABe12332
        // -ABe1
        Vector {
            e1: -self.0 * rhs.0,
            e2: 0.0,
            e3: 0.0,
        }
    }
}

impl Mul<Bivector_e31> for Pseudoscalar {
    type Output = Vector;

    fn mul(self, rhs: Bivector_e31) -> Vector {
        // ABe123e31
        // ABe12331
        // ABe121
        // -ABe112
        Vector {
            e1: 0.0,
            e2: -self.0 * rhs.0,
            e3: 0.0,
        }
    }
}

impl Mul<Pseudoscalar> for Bivector_e12 {
    type Output = Vector;

    fn mul(self, rhs: Pseudoscalar) -> Vector {
        // ABe12e123
        // AB e12123
        // -AB e21123
        // -AB e3
        Vector {
            e1: 0.0,
            e2: 0.0,
            e3: -self.0 * rhs.0,
        }
    }
}

impl Mul<Pseudoscalar> for Bivector_e23 {
    type Output = Vector;

    fn mul(self, rhs: Pseudoscalar) -> Vector {
        // ABe123e23
        // ABe12323
        // -ABe12332
        // -ABe1
        Vector {
            e1: -self.0 * rhs.0,
            e2: 0.0,
            e3: 0.0,
        }
    }
}

impl Mul<Pseudoscalar> for Bivector_e31 {
    type Output = Vector;

    fn mul(self, rhs: Pseudoscalar) -> Vector {
        // ABe123e31
        // ABe12331
        // ABe121
        // -ABe112
        Vector {
            e1: 0.0,
            e2: -self.0 * rhs.0,
            e3: 0.0,
        }
    }
}
