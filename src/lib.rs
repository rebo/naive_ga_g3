// 2018 is the future!
#![feature(uniform_paths)]
// needed as clippy complains about unknown lints when tests are present
#![allow(unknown_lints)]
//lots of u's and v's
#![allow(many_single_char_names)]
// used for some skipping of rust fmt
#![feature(tool_attributes)]

mod elements;
pub use elements::*;
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
    fn reflect_in_vector() {
        let u = Vector::new(1.0, 0.0, 0.0);
        let v = Vector::new(1.0, 1.0, 0.0);

        let u_dash = u.reflect_in_vector(v);

        assert!(u_dash.e1.approx_eq(&0.0, 2.0 * std::f64::EPSILON, 2));
        assert!(u_dash.e2.approx_eq(&1.0, 2.0 * std::f64::EPSILON, 2));
        print!("u_dash {:#?}", u_dash);
    }

    #[test]
    fn reflect_bivector_in_plane_with_normal_in_plane() {
        let bv = Bivector::e12();
        let n = Vector::new(-1.0, 1.0, 0.0);

        let bv_dash = bv.reflect_in_plane_with_normal(n);
        // print!("u_dash {:#?}", bv_dash);
        assert!(bv_dash.e12.0.approx_eq(&-1.0, 2.0 * std::f64::EPSILON, 2));
        assert!(bv_dash.e23.0.approx_eq(&0.0, 2.0 * std::f64::EPSILON, 2));
        assert!(bv_dash.e31.0.approx_eq(&0.0, 2.0 * std::f64::EPSILON, 2));
    }

    #[test]
    fn reflect_bivector_in_plane_with_normal() {
        let bv = Bivector::e12() + Bivector::e31();
        let n = Vector::new(0.0, 1.0, 0.0);

        let bv_dash = bv.reflect_in_plane_with_normal(n);
        // print!("u_dash {:#?}", bv_dash);
        assert!(bv_dash.e12.0.approx_eq(&-1.0, 2.0 * std::f64::EPSILON, 2));
        assert!(bv_dash.e23.0.approx_eq(&0.0, 2.0 * std::f64::EPSILON, 2));
        assert!(bv_dash.e31.0.approx_eq(&1.0, 2.0 * std::f64::EPSILON, 2));
    }

    #[test]
    fn reflect_in_plane_with_normal() {
        let u = Vector::new(1.0, 0.0, 0.0);
        let n = Vector::new(-1.0, 1.0, 0.0);

        let u_dash = u.reflect_in_plane_with_normal(n);
        print!("u_dash {:#?}", u_dash);
        assert!(u_dash.e1.approx_eq(&0.0, 2.0 * std::f64::EPSILON, 2));
        assert!(u_dash.e2.approx_eq(&1.0, 2.0 * std::f64::EPSILON, 2));
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
    fn slerp() {
        let u_initial = Vector::new(1.0, 0.0, 0.0);
        let u_finish = Vector::new(0.0, 1.0, 0.0);

        let u_s = u_initial.slerp(u_finish, 0.5);

        let frac_1_sqrt_2 = std::f64::consts::FRAC_1_SQRT_2;
        assert!(u_s.e1.approx_eq(&frac_1_sqrt_2, 2.0 * std::f64::EPSILON, 2));
        assert!(u_s.e2.approx_eq(&frac_1_sqrt_2, 2.0 * std::f64::EPSILON, 2));
        let u_s = u_initial.slerp(u_finish, 1.0);
        assert!(u_s.e2.approx_eq(&1.0, 2.0 * std::f64::EPSILON, 2));
        let u_s = u_initial.slerp(u_finish, 0.0);
        assert!(u_s.e1.approx_eq(&1.0, 2.0 * std::f64::EPSILON, 2));
    }
    // fn slerp_two_vectors(){
    //     let u =
    // }

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

    #[test]
    fn lerp_test() {
        let u = Vector::new(1.0, 0.0, 0.0);
        let v = Vector::new(0.0, 5.0, 0.0);

        let u_s = u.lerp(v, 0.5);

        assert!(u_s.e1.approx_eq(&0.5, 2.0 * std::f64::EPSILON, 2));
        assert!(u_s.e2.approx_eq(&2.5, 2.0 * std::f64::EPSILON, 2));
    }

    #[test]
    fn dist_to_plane() {
        let origin = Vector::new(5.0, 0.0, 0.0);
        let eastish = Vector::new(1.0, 0.0, 0.0);

        let plane = Plane::new(origin, Bivector::e12(), eastish);
        let dist = plane.dist_to_point(Vector::new(2.0, 7.0, 2.0));

        assert!(dist.approx_eq(&2.0, 2.0 * std::f64::EPSILON, 2));

        println!(
            "Distance to plane {:#?}",
            plane.dist_to_point(Vector::new(1.0, 2.0, 2.0))
        );

        println!(
            "Vector to plane {:#?}",
            plane.vector_to_point(Vector::new(1.0, 2.0, 2.0))
        );
    }

    #[test]
    fn line_plane_intersection() {
        let line = Line {
            origin: Vector::new(0.0, 10.0, 0.0),
            direction: Vector::new(1.0, 2.0, 1.0),
        };

        let plane_origin = Vector::new(0.0, -2.0, 0.0);
        let eastish = Vector::new(1.0, 0.0, 0.0);
        let plane = Plane::new(plane_origin, Bivector::from(BivectorE31::unit()), eastish);

        let s = line.intersection_with_plane_parameter(plane);
        assert!(s.unwrap().approx_eq(&-6.0, 2.0 * std::f64::EPSILON, 2));
    }

    #[test]
    fn line_plane_intersection_param_is_zero() {
        let line = Line {
            origin: Vector::new(0.0, 10.0, 0.0),
            direction: Vector::new(1.0, 2.0, 1.0),
        };

        let plane_origin = Vector::new(0.0, 10.0, 0.0);
        let eastish = Vector::new(1.0, 0.0, 0.0);
        let plane = Plane::new(plane_origin, Bivector::from(BivectorE31::unit()), eastish);
        let s = line.intersection_with_plane_parameter(plane);
        assert!(s.unwrap().approx_eq(&0.0, 2.0 * std::f64::EPSILON, 2));
    }

    #[test]
    fn rotor_direction() {
        let r1 = Rotor::from_exp(std::f64::consts::FRAC_PI_4, Bivector::e12());
        let r2 = Rotor::from_exp(std::f64::consts::PI, Bivector::e12());

        let r = r2 * r1;

        // need to check that r == -r1
        assert!(
            r.scalar()
                .approx_eq(&-r1.scalar(), 2.0 * std::f64::EPSILON, 2)
        );

        assert!(
            r.bivector()
                .e12
                .0
                .approx_eq(&-r1.bivector().e12.0, 2.0 * std::f64::EPSILON, 2)
        );
    }

    #[test]
    fn rotor_x_rotor() {
        let r1 = Rotor::from_exp(std::f64::consts::FRAC_PI_2, Bivector::e12());
        let r2 = Rotor::from_exp(std::f64::consts::FRAC_PI_4, Bivector::e12());
        let r3 = Rotor::from_exp(std::f64::consts::FRAC_PI_4 * 3.0, Bivector::e12());

        let r = r2 * r1;

        // need to check that r == -r1
        assert!(
            r.scalar()
                .approx_eq(&r3.scalar(), 2.0 * std::f64::EPSILON, 2)
        );

        assert!(
            r.bivector()
                .e12
                .0
                .approx_eq(&r3.bivector().e12.0, 2.0 * std::f64::EPSILON, 2)
        );
    }
}
