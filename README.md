# Naive Geometric Algebra G3

This is a naive implementation of the Geometric Algebra for G3. It is just a toy for practise working with ops overloads
and to use as a GA calculator.

I have mostly used the rust type system to use sensible objects rather than full multivector representations when possible.

It is not used for anything serious, in particular no effort has been made to make any of this efficient. It is a straightforward implementation with functions written by hand (not via a macro). Therefore it only implements G3.

Of course you can restrict oneself to G2 by ignoring the e3, e31 and e 123 basis elements.

I have tried to implement most needed operations, including projection, rejection, reversion and rotation by Rotor etc.

This is not fully tested so there is almost certainly errors in the mathematics so if you spot anything let me know.

It is not set up to be used as a crate so experiments are just in main.rs at the moment.

It uses Rust edition 2018 style, well cos it's the future!

Example:

    // create vectors u and v, then project u onto v finding the reflection of u in v

    let u = Vector { e1: 1.0, e2: 1.0, e3: 0.0,};
    let v = Vector { e1: 10.0, e2: 1.0, e3: 0.0,};

    // project u onto v
    println!("u proj v {:#?}", u.proj(v));

    // calculate the rejection of u onto v
    println!("u rej v {:#?}", u.rej(v));

    // to reflect u in v we do u_proj - u_rej
    println!("reflect u in v {:#?}", u.proj(v) - u.rej(v));

    // or use the built in reflect method (which does the same thing)
    println!("reflect u in v {:#?}", u.reflect(v));

Example with Rotors:

    // use the dual of v to define a bivector (plane) which another vector is then another vector 
    // is then rotated about by 45 degrees or π/4.

    // axis of rotation
    let v = Vector { e1: 1.0, e2: 1.0, e3: 1.0,};

    // bivector formed from dual of v
    let bivector = v.normalize() * Pseudoscalar::unit();

    /// half angle of rotation
    let half_angle = std::f64::consts::PI/8.0;

    // rotor created by use of e^(half-angle x B)
    let rotor = Rotor::from_exp(half_angle, bivector);

    // vector to rotate
    let r = Vector { e1: 0.0, e2: 0.0, e3: 1.0,};

    let rotated_r = r.apply_rotor(rotor); 
    // or could use rotated_r = rotor.rev() * r * rotor;

    println!("Rotated_r : {:#?} ", rotated_r);
