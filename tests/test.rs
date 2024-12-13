use ramp_gen::ramp;

#[test]
fn test(){
    let f = |x: f32| ramp!([0.0, 1.0], [2.0, 0.0], [4.0, 4.0]);

    assert_eq!(f(-1.0), 1.5);
    assert_eq!(f(0.0), 1.0);
    assert_eq!(f(1.0), 0.5);
    assert_eq!(f(2.0), 0.0);
    assert_eq!(f(3.0), 2.0);
    assert_eq!(f(4.0), 4.0);
    assert_eq!(f(5.0), 6.0);

    let f = |x: f32| ramp!(clamp [0.0, 1.0], [2.0, 0.0], [4.0, 4.0]);

    assert_eq!(f(-1.0), 1.0);
    assert_eq!(f(0.0), 1.0);
    assert_eq!(f(1.0), 0.5);
    assert_eq!(f(2.0), 0.0);
    assert_eq!(f(3.0), 2.0);
    assert_eq!(f(4.0), 4.0);
    assert_eq!(f(5.0), 4.0);

    let f = |x: f32| ramp!(steps [0.0, 1.0], [2.0, 0.0], [4.0, 4.0]);

    assert_eq!(f(-1.0), 1.0);
    assert_eq!(f(0.0), 1.0);
    assert_eq!(f(1.0), 1.0);
    assert_eq!(f(2.0), 0.0);
    assert_eq!(f(3.0), 0.0);
    assert_eq!(f(4.0), 4.0);
    assert_eq!(f(5.0), 4.0);

    let f = |x: f32| ramp!(ease [0.0, 1.0], [2.0, 0.0], [4.0, 4.0]);

    assert_eq!(f(0.0), 1.0);
    assert_eq!(f(1.0), 0.5);
    assert_eq!(f(2.0), 0.0);
    assert_eq!(f(3.0), 2.0);
    assert_eq!(f(4.0), 4.0);

    let f = |x: f32| ramp!(clamp ease [0.0, 1.0], [2.0, 0.0], [4.0, 4.0]);

    assert_eq!(f(-1.0), 1.0);
    assert_eq!(f(0.0), 1.0);
    assert_eq!(f(1.0), 0.5);
    assert_eq!(f(2.0), 0.0);
    assert_eq!(f(3.0), 2.0);
    assert_eq!(f(4.0), 4.0);
    assert_eq!(f(5.0), 4.0);
}