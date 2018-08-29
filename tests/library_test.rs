use naive_ga_g3::*;

#[test]
fn test_usage() {
    let u = Vector::new(1.0, 0.0, 0.0);
    let v = Vector::new(0.0, 1.0, 0.0);

    let mv = u * v;
    assert!(mv.is_bivector());
}
