mod bivector;
mod multivector;
mod pseudoscalar;
mod vector;

mod line;

mod ops;

// exporting the following structs and elements.
pub use self::bivector::{Bivector, BivectorE12, BivectorE23, BivectorE31};
pub use self::line::{Line, Plane};
pub use self::multivector::{Multivector, Rotor};
pub use self::pseudoscalar::Pseudoscalar;
pub use self::vector::Vector;
