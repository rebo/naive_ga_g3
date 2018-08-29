mod bivector;
mod line;
mod multivector;
mod ops;
mod pseudoscalar;
mod rotor;
mod vector;

// exporting the following structs and elements.
pub use self::bivector::{Bivector, BivectorE12, BivectorE23, BivectorE31};
pub use self::line::{Line, Plane};
pub use self::multivector::Multivector;
pub use self::pseudoscalar::Pseudoscalar;
pub use self::rotor::Rotor;
pub use self::vector::Vector;
