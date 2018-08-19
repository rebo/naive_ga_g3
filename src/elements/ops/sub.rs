use super::super::{Bivector, Multivector, Vector};

use std::ops::Sub;
// Multiplication operations

// Scalars

impl Sub<Bivector> for f64 {
    type Output = Multivector;

    fn sub(self, rhs: Bivector) -> Multivector {
        Multivector {
            scalar: self,
            vector: Vector::zero(),
            bivector: -rhs,
        }
    }
}

// Vectors

impl Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Vector {
        Vector {
            e1: self.e1 - rhs.e1,
            e2: self.e2 - rhs.e2,
        }
    }
}

// Bivectors

impl Sub<f64> for Bivector {
    type Output = Multivector;

    fn sub(self, rhs: f64) -> Multivector {
        Multivector {
            scalar: -rhs,
            vector: Vector::zero(),
            bivector: self,
        }
    }
}

impl Sub<Bivector> for Bivector {
    type Output = Bivector;

    fn sub(self, rhs: Bivector) -> Bivector {
        Bivector(self.0 - rhs.0)
    }
}

impl Sub for Multivector {
    type Output = Multivector;

    fn sub(self, rhs: Multivector) -> Self {
        Multivector {
            scalar: self.scalar - rhs.scalar,
            vector: self.vector - rhs.vector,
            bivector: self.bivector - rhs.bivector,
        }
    }
}

impl Sub<f64> for Multivector {
    type Output = Multivector;

    fn sub(self, rhs: f64) -> Self {
        Multivector {
            scalar: self.scalar - rhs,
            vector: self.vector,
            bivector: self.bivector,
        }
    }
}

impl Sub<Multivector> for f64 {
    type Output = Multivector;

    fn sub(self, rhs: Multivector) -> Multivector {
        Multivector {
            scalar: self - rhs.scalar,
            vector: -rhs.vector,
            bivector: -rhs.bivector,
        }
    }
}
