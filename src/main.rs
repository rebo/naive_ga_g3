#![feature(rust_2018_preview, uniform_paths)]
#![allow(many_single_char_names)]
#![feature(tool_attributes)]

mod elements;

use elements::*;

fn main() {
    // Creation of 2D vectors
    let u = Vector {
        e1: 1.0,
        e2: 0.0,
        e3: 0.0,
    };
    let v = Vector {
        e1: 0.0,
        e2: 1.0,
        e3: 0.0,
    };

    // multivector multiplication
    let multivector = u * v;
    println!("uv = {:#?}", multivector);

    // defining e1 and e2 as a mutlivector
    let e1 = Multivector {
        scalar: 0.0,
        vector: Vector {
            e1: 1.0,
            e2: 0.0,
            e3: 0.0,
        },
        bivector: Bivector::zero(),
        pseudoscalar: Pseudoscalar::zero(),
    };

    let e2 = Multivector {
        scalar: 0.0,
        vector: Vector {
            e1: 0.0,
            e2: 1.0,
            e3: 0.0,
        },
        bivector: Bivector::zero(),
        pseudoscalar: Pseudoscalar::zero(),
    };

    let e12 = BivectorE12(1.0);

    //multivector multipliced by multivector
    let e1e2 = e1 * e2;

    println!("e1e2 {:#?}", e1e2);
    // bivector * mutlivector
    println!("e12 * e1 {:#?}", e12 * e1);
    // repeated bivector multiplication of e1 multivector
    println!("e12 * e1 {:#?}", e1 * e12 * e12 * e12 * e12);

    // more vector multiplication
    let u = Vector {
        e1: 3.5,
        e2: -2.8,
        e3: 0.0,
    };
    let v = Vector {
        e1: 3.0,
        e2: 5.0,
        e3: 0.0,
    };

    println!("uv {:#?}", u * v); // should be 3.5 + 25.9 e12

    // checking a bivector * a bivector gives a scalar
    println!("Hello, world! {:#?}", BivectorE12(3f64) * BivectorE12(2f64));

    // checking rotation of e1 and e2 under unit bivector
    println!("e1 * I {:#?}", Vector::e1() * BivectorE12::unit());
    println!("I * e2 {:#?}", BivectorE12::unit() * Vector::e2());

    // u and v for projection and rejection
    let u = Vector {
        e1: 1.0,
        e2: 1.0,
        e3: 0.0,
    };
    let v = Vector {
        e1: 10.0,
        e2: 1.0,
        e3: 0.0,
    };

    println!("u proj v {:#?}", u.proj(v));
    println!("u rej v {:#?}", u.rej(v));

    // checking u = U_proj + u_rej
    println!("get back to u {:#?}", u.proj(v) + u.rej(v));

    // to reflect u in v we do u_proj - u_rej
    println!("reflect u in v {:#?}", u.proj(v) - u.rej(v));

    // manual creation of a rotor
    let q_unit = Vector {
        e1: 1.0,
        e2: 0.0,
        e3: 0.0,
    }.normalize();
    let w_unit = Vector {
        e1: 1.0,
        e2: 1.0,
        e3: 0.0,
    }.normalize();

    // notice the type here is a multivector
    let rotor = q_unit * w_unit;

    let test_vector = Vector {
        e1: 10.0,
        e2: 0.0,
        e3: 0.0,
    };

    // manual rotation by rotor
    let rotated_vector = rotor.rev() * test_vector * rotor;

    println!("test vector = {:#?}", test_vector);
    println!("rotor = {:#?}", rotor);
    println!("rotated vector = {:#?} ", rotated_vector);

    // vectors for rotor creation
    let q = Vector {
        e1: 4.5,
        e2: 0.0,
        e3: 0.0,
    };
    let w = Vector {
        e1: 0.0,
        e2: 10.0,
        e3: 0.0,
    };

    let r = Rotor::new_from_u_v(q, w);

    let t = Vector {
        e1: 5.0,
        e2: 0.0,
        e3: 0.0,
    };

    let rotated_t = t.apply_rotor(r);

    println!("Rotated t {:#?}", rotated_t);

    let u = Vector {
        e1: 1.0,
        e2: 0.0,
        e3: 0.0,
    };
    let v = Vector {
        e1: 1.0,
        e2: 0.5,
        e3: 0.0,
    };
    let t = Vector {
        e1: 10.0,
        e2: 0.0,
        e3: 0.0,
    };

    println!("reflect in v and then w {:#?}", t.reflect(u).reflect(v));
    println!(
        "rotate with rotor {:#?}",
        t.apply_rotor(Rotor::new_from_u_v(u, v))
    );
}
