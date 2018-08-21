use float_cmp::ApproxEq;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pseudoscalar(pub f64);

impl Pseudoscalar {
    pub fn unit() -> Self {
        Pseudoscalar(1.0)
    }
    pub fn zero() -> Self {
        Pseudoscalar(0.0)
    }

    pub fn rev(self) -> Self {
        -1.0 * self
    }

    pub fn is_zero(self) -> bool {
        self.0.approx_eq(&0.0, 2.0 * std::f64::EPSILON, 2)
    }
}
