use super::super::Pseudoscalar;

use std::ops::{Add, Mul};

impl Add for Pseudoscalar {
    type Output = Pseudoscalar;

    fn add(self, rhs: Pseudoscalar) -> Pseudoscalar {
        Pseudoscalar(self.0 + rhs.0)
    }
}

impl Mul for Pseudoscalar {
    type Output = f64;

    fn mul(self, rhs: Pseudoscalar) -> f64 {
        -self.0 * rhs.0
    }
}
