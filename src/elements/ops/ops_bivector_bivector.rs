use super::super::{
    Bivector, BivectorE12, BivectorE23, BivectorE31,  Multivector
};

use std::ops::{Add, Mul, Neg,Sub};


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

impl Neg for BivectorE12 {
    type Output = BivectorE12;

    fn neg(self) -> BivectorE12 {
        BivectorE12(-self.0)
    }
}

impl Neg for BivectorE23 {
    type Output = BivectorE23;

    fn neg(self) -> BivectorE23 {
       BivectorE23(-self.0)
    }
}

impl Neg for BivectorE31 {
    type Output = BivectorE31;

    fn neg(self) -> BivectorE31 {
        BivectorE31(-self.0)
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

impl Add for BivectorE12 {
    type Output = BivectorE12;

    fn add(self, rhs: BivectorE12) -> BivectorE12 {
        BivectorE12(self.0 + rhs.0)
    }
}

impl Add for BivectorE23 {
    type Output = BivectorE23;

    fn add(self, rhs: BivectorE23) -> BivectorE23 {
        BivectorE23(self.0 + rhs.0)
    }
}

impl Add for BivectorE31 {
    type Output = BivectorE31;

    fn add(self, rhs: BivectorE31) -> BivectorE31 {
        BivectorE31(self.0 + rhs.0)
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

impl Mul for BivectorE12  {
    type Output = f64;

    fn mul(self, rhs:BivectorE12) -> f64 {
        -self.0*rhs.0
    }

}

impl Mul for BivectorE23  {
    type Output = f64;

    fn mul(self, rhs:BivectorE23) -> f64 {
        -self.0*rhs.0
    }
    
}

impl Mul for BivectorE31  {
    type Output = f64;

    fn mul(self, rhs:BivectorE31) -> f64 {
        -self.0*rhs.0
    }
}

// e12 mul e23
impl Mul<BivectorE12> for BivectorE23  {
    type Output = BivectorE31;    

    fn mul(self, rhs:BivectorE12) -> BivectorE31 {
        BivectorE31(self.0*rhs.0)
    }
}

impl Mul<BivectorE23> for BivectorE12  {
    type Output = BivectorE31;    

    fn mul(self, rhs:BivectorE23) -> BivectorE31 {
        -1.0 * BivectorE31(self.0*rhs.0)
    }
}


// e31 mul e12
impl Mul<BivectorE31> for BivectorE12  {
    type Output = BivectorE23;

    fn mul(self, rhs:BivectorE31) -> BivectorE23 { 
        BivectorE23(self.0*rhs.0)
    }
}

impl Mul<BivectorE12> for BivectorE31  {
    type Output = BivectorE23;

    fn mul(self, rhs:BivectorE12) -> BivectorE23 {
        -1.0 * BivectorE23(self.0*rhs.0)
    }
}
// e23 mul e31


impl Mul<BivectorE23> for BivectorE31  {
    type Output = BivectorE12;
//     3123
//    -1323
//     1332
//     12 
    fn mul(self, rhs:BivectorE23) -> BivectorE12 {
        BivectorE12(self.0*rhs.0)
    }
}

impl Mul<BivectorE31> for BivectorE23  {
    type Output = BivectorE12;

    fn mul(self, rhs:BivectorE31) -> BivectorE12 { 
        -1.0 * BivectorE12(self.0*rhs.0)
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
impl Mul<Bivector> for BivectorE12  {
    type Output = Multivector;    

    fn mul(self, rhs:Bivector) -> Multivector {
        Bivector::from(self) * rhs
    }
}

impl Mul<BivectorE12> for Bivector  {
    type Output = Multivector;    

    fn mul(self, rhs:BivectorE12) -> Multivector {
        self * Bivector::from(rhs)
    }
}

impl Mul<Bivector> for BivectorE23  {
    type Output = Multivector;    

    fn mul(self, rhs:Bivector) -> Multivector {
        Bivector::from(self) * rhs
    }
}

impl Mul<BivectorE23> for Bivector  {
    type Output = Multivector;    

    fn mul(self, rhs:BivectorE23) -> Multivector {
        self * Bivector::from(rhs)
    }
}

impl Mul<Bivector> for BivectorE31  {
    type Output = Multivector;    

    fn mul(self, rhs:Bivector) -> Multivector {
        Bivector::from(self) * rhs
    }
}

impl Mul<BivectorE31> for Bivector  {
    type Output = Multivector;    

    fn mul(self, rhs:BivectorE31) -> Multivector {
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

impl Sub for BivectorE12  {
    type Output = BivectorE12;

    fn sub(self, rhs:BivectorE12) -> BivectorE12 {
        BivectorE12( self.0 - rhs.0     )
    }

}

impl Sub for BivectorE23  {
    type Output = BivectorE23;

    fn sub(self, rhs:BivectorE23) -> BivectorE23 {
        BivectorE23( self.0 - rhs.0 )
    }
    
}

impl Sub for BivectorE31  {
    type Output = BivectorE31;

    fn sub(self, rhs:BivectorE31) -> BivectorE31 {
        BivectorE31( self.0 - rhs.0 )
    }
}

// e12 sub e23
impl Sub<BivectorE12> for BivectorE23  {
    type Output = Bivector;    

    fn sub(self, rhs:BivectorE12) -> Bivector {
        Bivector::from(self) - Bivector::from(rhs)
    }
}

impl Sub<BivectorE23> for BivectorE12  {
    type Output = Bivector;    

    fn sub(self, rhs:BivectorE23) -> Bivector {
        Bivector::from(self) - Bivector::from(rhs)
    }
}


// e31 sub e12
impl Sub<BivectorE31> for BivectorE12  {
    type Output = Bivector;

    fn sub(self, rhs:BivectorE31) -> Bivector { 
       Bivector::from(self) - Bivector::from(rhs)
    }
}

impl Sub<BivectorE12> for BivectorE31  {
    type Output = Bivector;

    fn sub(self, rhs:BivectorE12) -> Bivector  {
        Bivector::from(self) - Bivector::from(rhs)
    }
}
// e23 sub e31


impl Sub<BivectorE23> for BivectorE31  {
    type Output = Bivector;
//     3123
//    -1323
//     1332
//     12 
    fn sub(self, rhs:BivectorE23) -> Bivector {
        Bivector::from(self) - Bivector::from(rhs)
    }
}

impl Sub<BivectorE31> for BivectorE23  {
    type Output = Bivector;

    fn sub(self, rhs:BivectorE31) -> Bivector { 
        Bivector::from(self) - Bivector::from(rhs)
    }
}
