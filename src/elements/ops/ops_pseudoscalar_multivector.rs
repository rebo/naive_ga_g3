use crate::elements::{Multivector, Pseudoscalar};

use std::ops::Add;

impl Add<Multivector> for Pseudoscalar {
    type Output = Multivector;

    fn add(self, rhs: Multivector) -> Multivector {
        Multivector::from(self) + rhs
    }
}

impl Add<Pseudoscalar> for Multivector {
    type Output = Multivector;

    fn add(self, rhs: Pseudoscalar) -> Multivector {
        self + Multivector::from(rhs)
    }
}
