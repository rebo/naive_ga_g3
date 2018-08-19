use super::super::{
    Bivector, Bivector_e12, Bivector_e23, Bivector_e31, Pseudoscalar, Vector, Multivector
};

use std::ops::{Add, Mul, Neg, BitXor,Sub};


impl Neg for Bivector {
    type Output = Bivector;

    fn neg(self) -> Bivector {
        Bivector {
            e12: -self.e12 ,
            e23: -self.e23 ,
            e31: -self.e31 ,
        }
    }
}

impl Neg for Bivector_e12 {
    type Output = Bivector_e12;

    fn neg(self) -> Bivector_e12 {
        Bivector_e12(-self.0)
    }
}

impl Neg for Bivector_e23 {
    type Output = Bivector_e23;

    fn neg(self) -> Bivector_e23 {
       Bivector_e23(-self.0)
    }
}

impl Neg for Bivector_e31 {
    type Output = Bivector_e31;

    fn neg(self) -> Bivector_e31 {
        Bivector_e31(-self.0)
    }
}

// ADD
// ADD
// ADD
// ADD

impl Add for Bivector {
    type Output = Bivector;

    fn add(self, rhs: Bivector) -> Bivector {
        Bivector {
            e12: self.e12 + rhs.e12,
            e23: self.e23 + rhs.e23,
            e31: self.e31 + rhs.e31,
        }
    }
}

impl Add for Bivector_e12 {
    type Output = Bivector_e12;

    fn add(self, rhs: Bivector_e12) -> Bivector_e12 {
        Bivector_e12(self.0 + rhs.0)
    }
}

impl Add for Bivector_e23 {
    type Output = Bivector_e23;

    fn add(self, rhs: Bivector_e23) -> Bivector_e23 {
        Bivector_e23(self.0 + rhs.0)
    }
}

impl Add for Bivector_e31 {
    type Output = Bivector_e31;

    fn add(self, rhs: Bivector_e31) -> Bivector_e31 {
        Bivector_e31(self.0 + rhs.0)
    }
}

////////
/// ////
/// ///
/// ///
/// 
/// 
/// 




//MUL
//MUL
//MUL
//MUL
//MUL
//MUL
//MUL


// Bivectors mu bivectors
impl Mul for Bivector {
    type Output = Multivector;
    #[rustfmt::skip]
    fn mul(self, rhs: Bivector) -> Multivector {
        self.e12 * rhs.e12  +  self.e23  *  rhs.e12  +  self.e31  *  rhs.e12 +
        self.e12 * rhs.e23  +  self.e23  *  rhs.e23  +  self.e31  *  rhs.e23 +
        self.e12 * rhs.e31  +  self.e23  *  rhs.e31  +  self.e31  *  rhs.e31 
    }
}

impl Mul for Bivector_e12  {
    type Output = f64;

    fn mul(self, rhs:Bivector_e12) -> f64 {
        -self.0*rhs.0
    }

}

impl Mul for Bivector_e23  {
    type Output = f64;

    fn mul(self, rhs:Bivector_e23) -> f64 {
        -self.0*rhs.0
    }
    
}

impl Mul for Bivector_e31  {
    type Output = f64;

    fn mul(self, rhs:Bivector_e31) -> f64 {
        -self.0*rhs.0
    }
}

// e12 mul e23
impl Mul<Bivector_e12> for Bivector_e23  {
    type Output = Bivector_e31;    

    fn mul(self, rhs:Bivector_e12) -> Bivector_e31 {
        Bivector_e31(self.0*rhs.0)
    }
}

impl Mul<Bivector_e23> for Bivector_e12  {
    type Output = Bivector_e31;    

    fn mul(self, rhs:Bivector_e23) -> Bivector_e31 {
        -1.0 * Bivector_e31(self.0*rhs.0)
    }
}


// e31 mul e12
impl Mul<Bivector_e31> for Bivector_e12  {
    type Output = Bivector_e23;

    fn mul(self, rhs:Bivector_e31) -> Bivector_e23 { 
        Bivector_e23(self.0*rhs.0)
    }
}

impl Mul<Bivector_e12> for Bivector_e31  {
    type Output = Bivector_e23;

    fn mul(self, rhs:Bivector_e12) -> Bivector_e23 {
        -1.0 * Bivector_e23(self.0*rhs.0)
    }
}
// e23 mul e31


impl Mul<Bivector_e23> for Bivector_e31  {
    type Output = Bivector_e12;
//     3123
//    -1323
//     1332
//     12 
    fn mul(self, rhs:Bivector_e23) -> Bivector_e12 {
        Bivector_e12(self.0*rhs.0)
    }
}

impl Mul<Bivector_e31> for Bivector_e23  {
    type Output = Bivector_e12;

    fn mul(self, rhs:Bivector_e31) -> Bivector_e12 { 
        -1.0 * Bivector_e12(self.0*rhs.0)
    }
}


//
//
//
///
/// ///
/// ////
/// 
/// 


// e12 mul e23
impl Mul<Bivector> for Bivector_e12  {
    type Output = Multivector;    

    fn mul(self, rhs:Bivector) -> Multivector {
        Bivector::from(self) * rhs
    }
}

impl Mul<Bivector_e12> for Bivector  {
    type Output = Multivector;    

    fn mul(self, rhs:Bivector_e12) -> Multivector {
        self * Bivector::from(rhs)
    }
}

impl Mul<Bivector> for Bivector_e23  {
    type Output = Multivector;    

    fn mul(self, rhs:Bivector) -> Multivector {
        Bivector::from(self) * rhs
    }
}

impl Mul<Bivector_e23> for Bivector  {
    type Output = Multivector;    

    fn mul(self, rhs:Bivector_e23) -> Multivector {
        self * Bivector::from(rhs)
    }
}

impl Mul<Bivector> for Bivector_e31  {
    type Output = Multivector;    

    fn mul(self, rhs:Bivector) -> Multivector {
        Bivector::from(self) * rhs
    }
}

impl Mul<Bivector_e31> for Bivector  {
    type Output = Multivector;    

    fn mul(self, rhs:Bivector_e31) -> Multivector {
        self * Bivector::from(rhs)
    }
}

/// /
/// /
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 


impl Sub for Bivector {
    type Output = Bivector;

    fn sub(self, rhs: Bivector) -> Bivector {
        Bivector {
            e12: self.e12 - rhs.e12,
            e23: self.e23 - rhs.e23,
            e31: self.e31 - rhs.e31,
        }
    }
}

impl Sub for Bivector_e12  {
    type Output = Bivector_e12;

    fn sub(self, rhs:Bivector_e12) -> Bivector_e12 {
        Bivector_e12( self.0 - rhs.0     )
    }

}

impl Sub for Bivector_e23  {
    type Output = Bivector_e23;

    fn sub(self, rhs:Bivector_e23) -> Bivector_e23 {
        Bivector_e23( self.0 - rhs.0 )
    }
    
}

impl Sub for Bivector_e31  {
    type Output = Bivector_e31;

    fn sub(self, rhs:Bivector_e31) -> Bivector_e31 {
        Bivector_e31( self.0 - rhs.0 )
    }
}

// e12 sub e23
impl Sub<Bivector_e12> for Bivector_e23  {
    type Output = Bivector;    

    fn sub(self, rhs:Bivector_e12) -> Bivector {
        Bivector::from(self) - Bivector::from(rhs)
    }
}

impl Sub<Bivector_e23> for Bivector_e12  {
    type Output = Bivector;    

    fn sub(self, rhs:Bivector_e23) -> Bivector {
        Bivector::from(self) - Bivector::from(rhs)
    }
}


// e31 sub e12
impl Sub<Bivector_e31> for Bivector_e12  {
    type Output = Bivector;

    fn sub(self, rhs:Bivector_e31) -> Bivector { 
       Bivector::from(self) - Bivector::from(rhs)
    }
}

impl Sub<Bivector_e12> for Bivector_e31  {
    type Output = Bivector;

    fn sub(self, rhs:Bivector_e12) -> Bivector  {
        Bivector::from(self) - Bivector::from(rhs)
    }
}
// e23 sub e31


impl Sub<Bivector_e23> for Bivector_e31  {
    type Output = Bivector;
//     3123
//    -1323
//     1332
//     12 
    fn sub(self, rhs:Bivector_e23) -> Bivector {
        Bivector::from(self) - Bivector::from(rhs)
    }
}

impl Sub<Bivector_e31> for Bivector_e23  {
    type Output = Bivector;

    fn sub(self, rhs:Bivector_e31) -> Bivector { 
        Bivector::from(self) - Bivector::from(rhs)
    }
}
