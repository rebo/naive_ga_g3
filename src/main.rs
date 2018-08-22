// 2018 is the future!
#![feature(uniform_paths)]
// needed as clippy complains about unknown lints when tests are present
#![allow(unknown_lints)]
//lots of u's and v's
#![allow(many_single_char_names)]
// used for some skipping of rust fmt
#![feature(tool_attributes)]

mod elements;

use elements::*;

fn main() {
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
        e1: 0.0,
        e2: 0.0,
        e3: 1.0,
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

    let v = Vector {
        e1: 1.0,
        e2: 1.0,
        e3: 1.0,
    };

    let bivector = v.normalize() * Pseudoscalar::unit();
    println!("Bivector  {:#?}", bivector);
    let half_angle = std::f64::consts::PI / 8.0;
    let rotor = Rotor::from_exp(half_angle, bivector);
    println!("rotor{:#?}", rotor);

    println!("Half Angle: {:#?} ", rotor.half_angle());

    let r = Vector {
        e1: 0.0,
        e2: 0.0,
        e3: 1.0,
    };

    let rotated_r = r.apply_rotor(rotor);

    println!("Rotated_r : {:#?} ", rotated_r);
}

#[cfg(test)]
mod tests {
    #![allow(unknown_lints)]
    use super::*;
    use float_cmp::ApproxEq;
    #[test]
    fn vector_dot_vector() {
        let u = Vector {
            e1: 1.0,
            e2: 2.0,
            e3: 4.0,
        };

        let v = Vector {
            e1: 5.0,
            e2: 3.0,
            e3: 2.0,
        };
        let dot_product = u.dot(v);

        assert!(dot_product.approx_eq(&19.0, 2.0 * std::f64::EPSILON, 2));
    }

    #[test]
    fn vector_dot_bivector() {
        // (1e1 + 2e2 + 4e3) dot e12
        // 0.5 * ( (1e1 + 2e2 + 4e3) e12 - e12 (1e1 + 2e2 + 4e3) )
        // 0.5 * ( 1e112 + 2e212 + 4e312 - (1e121 + 2e122 + 4 e123))
        // 0.5 * ( 1e2 - 2e1 + 4 e123 + 1e2 - 2e1 - 4e123)
        // 0.5 * (2e2 - 4 e1)
        // 1e2 - 2e1
        // as you can see it projects the vector into the E12 plane.

        let u = Vector {
            e1: 1.0,
            e2: 2.0,
            e3: 4.0,
        };

        let dot_product = u.dot(Bivector::from(BivectorE12::unit()));

        assert!(dot_product.e1.approx_eq(&-2.0, 2.0 * std::f64::EPSILON, 2));
        assert!(dot_product.e2.approx_eq(&1.0, 2.0 * std::f64::EPSILON, 2));
    }

    #[test]
    fn vector_wedge_bivector() {
        // (1e1 + 2e2 + 4e3) dot e12
        // 0.5 * ( (1e1 + 2e2 + 4e3) e12 + e12 (1e1 + 2e2 + 4e3) )
        // 0.5 * ( 1e112 + 2e212 + 4e312 + (1e121 + 2e122 + 4 e123))
        // 0.5 * ( 1e2 - 2e1 + 4 e123 - 1e2 + 2e1 + 4e123)
        // 0.5 * (8e1231)
        // 4e123
        // which should be a pseudoscalar

        let u = Vector {
            e1: 1.0,
            e2: 2.0,
            e3: 4.0,
        };

        let wedge_product = u ^ Bivector::from(BivectorE12::unit());

        assert!(wedge_product.0.approx_eq(&4.0, 2.0 * std::f64::EPSILON, 2));
    }

    #[test]
    fn bivector_dot_vector() {
        let u = Vector {
            e1: 1.0,
            e2: 2.0,
            e3: 4.0,
        };

        let bv = Bivector::from(BivectorE12::unit());

        let dot_product = bv.dot(u);
        assert!(dot_product.e1.approx_eq(&2.0, 2.0 * std::f64::EPSILON, 2));
        assert!(dot_product.e2.approx_eq(&-1.0, 2.0 * std::f64::EPSILON, 2));
    }

    #[test]
    fn bivector_wedge_vector() {
        let u = Vector {
            e1: 1.0,
            e2: 2.0,
            e3: 4.0,
        };

        let bv = Bivector::from(BivectorE12::unit());
        let wedge_product = bv ^ u;
        assert!(wedge_product.0.approx_eq(&4.0, 2.0 * std::f64::EPSILON, 2));
    }

    #[test]
    fn e1e1_multi() {
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
        let m = e1 * e2;

        assert_eq!(m, Multivector::from(Bivector::from(BivectorE12::unit())));
    }

    #[test]
    fn uv_basis() {
        let u = Vector {
            e1: 0.0,
            e2: 1.0,
            e3: 0.0,
        };
        let v = Vector {
            e1: 0.0,
            e2: 0.0,
            e3: 1.0,
        };

        assert_eq!(
            u * v,
            Multivector::from(Bivector::from(BivectorE23::unit()))
        );
    }

    #[test]
    fn uv_general() {
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
        let uv = u * v;

        assert_eq!(
            uv,
            Multivector {
                scalar: -3.5,
                vector: Vector::zero(),
                bivector: Bivector::from(BivectorE12(25.9)),
                pseudoscalar: Pseudoscalar::zero(),
            }
        )
    }

    #[test]
    fn bivector_basis_multi() {
        let result = BivectorE12(3f64) * BivectorE12(2f64);

        assert!(result.approx_eq(&-6.0, 2.0 * std::f64::EPSILON, 2));
    }

    #[test]
    fn unit_m_e12() {
        let e1e12 = Vector::e1() * BivectorE12::unit();

        assert!(e1e12.vector.e1.approx_eq(&0.0, 2.0 * std::f64::EPSILON, 2));
        assert!(e1e12.vector.e2.approx_eq(&1.0, 2.0 * std::f64::EPSILON, 2));
    }
}
