use crate::elements::Pseudoscalar;

use std::ops::Mul;

// vector mul multivector

impl Mul<f64> for Pseudoscalar {
    type Output = Pseudoscalar;

    fn mul(self, rhs: f64) -> Pseudoscalar {
        Pseudoscalar(self.0 * rhs)
    }
}

impl Mul<Pseudoscalar> for f64 {
    type Output = Pseudoscalar;

    fn mul(self, rhs: Pseudoscalar) -> Pseudoscalar {
        Pseudoscalar(self * rhs.0)
    }
}
