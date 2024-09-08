use rasterization::{Rasterization, SemicircleFilled};

#[test]
fn test_semicircle_new_empty() {
    let semicircle_iter = SemicircleFilled::<i32>::new(0_usize);
    let vec = semicircle_iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![]);
}

#[test]
fn test_semicircle_new_debug() {
    let semicircle = SemicircleFilled::<i32>::new(0_u32);
    assert_eq!(
        format!("{:?}", semicircle),
        "SemicircleFilled { x: 0, y: 0, err: 2 }"
    );
}

#[test]
fn test_semicircle_new_clone() {
    let semicircle_iter = SemicircleFilled::<i32>::new(10_u32);
    let semicircle_iter_clone = semicircle_iter.clone();
    assert_eq!(semicircle_iter, semicircle_iter_clone);
}

#[test]
fn test_semicircle_circle_empty() {
    let circle_iter = SemicircleFilled::<isize>::new(0_u8).circle();
    let circle_vec = circle_iter.collect::<Vec<_>>();
    assert_eq!(circle_vec, vec![]);
}

#[test]
fn test_semicircle_circle_debug() {
    let circle_iter = SemicircleFilled::<isize>::new(7_u16).circle();
    assert_eq!(format!("{:?}", circle_iter), "FlatMap { inner: FlattenCompat { iter: Fuse { iter: Some(\
            Map { iter: SemicircleFilled { x: -7, y: 0, err: -12 } }) }, frontiter: None, backiter: None } }");
}

#[test]
fn test_semicircle_circle_clone() {
    let circle_iter = SemicircleFilled::<isize>::new(6_u32).circle();
    let circle_iter_clone = circle_iter.clone();
    let circle_vec = circle_iter.collect::<Vec<_>>();
    let circle_clone_vec = circle_iter_clone.collect::<Vec<_>>();
    assert_eq!(circle_vec, circle_clone_vec);
}

#[test]
fn test_semicircle_circle_three() {
    let iter = SemicircleFilled::<i32>::new(3_u64).circle();
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, 
        vec![(-3, -1), (-3, 0), (-2, -1), (-2, 0), (-1, -1), (-1, 0),
        (0, -1), (0, 0), (1, -1), (1, 0), (2, -1), (2, 0), (-3, -2),
        (-3, 1), (-2, -2), (-2, 1), (-1, -2), (-1, 1), (0, -2),
        (0, 1), (1, -2), (1, 1), (2, -2), (2, 1), (-2, -3), (-2, 2),
        (-1, -3), (-1, 2), (0, -3), (0, 2), (1, -3), (1, 2)]);
}
#[test]
fn test_semicircle_circle_one() {
    let iter = SemicircleFilled::<i32>::new(1_usize).circle();
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, 
        vec![(-1, -1), (-1, 0), (0, -1), (0, 0)]);
}

#[test]
fn test_semicircle_circle_long_empty() {
    let circle_long_iter = SemicircleFilled::<i64>::new(0_u8).circle_long(0, 0);
    let circle_long_vec = circle_long_iter.collect::<Vec<_>>();
    assert_eq!(circle_long_vec, vec![]);
    let circle_long_iter = SemicircleFilled::<i64>::new(0_u8).circle_long(-10, 10);
    let circle_long_vec = circle_long_iter.collect::<Vec<_>>();
    assert_eq!(circle_long_vec, vec![]);
}

#[test]
fn test_semicircle_circle_long_debug() {
    let circle_long_iter = SemicircleFilled::<i64>::new(7_u16).circle_long(1, 1);
    assert_eq!(format!("{:?}", circle_long_iter), "FlatMap { inner: FlattenCompat { iter: Fuse { iter: Some(\
        Map { iter: SemicircleFilled { x: -7, y: 0, err: -12 } }) }, frontiter: None, backiter: None } }");
}

#[test]
fn test_semicircle_circle_long_clone() {
    let circle_long_iter = SemicircleFilled::<i64>::new(6_u32).circle_long(-1, 1);
    let circle_long_iter_clone = circle_long_iter.clone();
    let circle_long_vec = circle_long_iter.collect::<Vec<_>>();
    let circle_long_clone_vec = circle_long_iter_clone.collect::<Vec<_>>();
    assert_eq!(circle_long_vec, circle_long_clone_vec);
}

#[test]
fn test_semicircle_circle_long_five() {
    let circle_long_iter = SemicircleFilled::<i64>::new(5_usize).circle_long(-2, 2);
    let circle_long_vec = circle_long_iter.collect::<Vec<_>>();
    assert_eq!(circle_long_vec,
        vec![(-7, -1), (-7, 0), (-6, -1), (-6, 0), (-5, -1), (-5, 0),
        (-4, -1), (-4, 0), (-3, -1), (-3, 0), (-2, -1), (-2, 0),
        (-1, -1), (-1, 0), (0, -1), (0, 0), (1, -1), (1, 0), (2, -1),
        (2, 0), (3, -1), (3, 0), (4, -1), (4, 0), (5, -1), (5, 0),
        (6, -1), (6, 0), (-7, -2), (-7, 1), (-6, -2), (-6, 1),
        (-5, -2), (-5, 1), (-4, -2), (-4, 1), (-3, -2), (-3, 1),
        (-2, -2), (-2, 1), (-1, -2), (-1, 1), (0, -2), (0, 1),
        (1, -2), (1, 1), (2, -2), (2, 1), (3, -2), (3, 1), (4, -2),
        (4, 1), (5, -2), (5, 1), (6, -2), (6, 1), (-6, -3), (-6, 2),
        (-5, -3), (-5, 2), (-4, -3), (-4, 2), (-3, -3), (-3, 2),
        (-2, -3), (-2, 2), (-1, -3), (-1, 2), (0, -3), (0, 2),
        (1, -3), (1, 2), (2, -3), (2, 2), (3, -3), (3, 2), (4, -3),
        (4, 2), (5, -3), (5, 2), (-5, -4), (-5, 3), (-4, -4),
        (-4, 3), (-3, -4), (-3, 3), (-2, -4), (-2, 3), (-1, -4),
        (-1, 3), (0, -4), (0, 3), (1, -4), (1, 3), (2, -4), (2, 3),
        (3, -4), (3, 3), (4, -4), (4, 3), (-4, -5), (-4, 4), (-3, -5),
        (-3, 4), (-2, -5), (-2, 4), (-1, -5), (-1, 4), (0, -5),
        (0, 4), (1, -5), (1, 4), (2, -5), (2, 4), (3, -5), (3, 4)]);
}

#[test]
fn test_semicircle_semicircle_top_empty() {
    let semicircle_top_iter = SemicircleFilled::<i64>::new(0_u64).semicircle_top();
    let semicircle_top_vec = semicircle_top_iter.collect::<Vec<_>>();
    assert_eq!(semicircle_top_vec, vec![]);
}

#[test]
fn test_semicircle_semicircle_top_debug() {
    let semicircle_top_iter = SemicircleFilled::<i64>::new(20_u128).semicircle_top();
    assert_eq!(format!("{:?}", semicircle_top_iter), "FlatMap { inner: FlattenCompat { iter: Fuse { iter: Some(\
            Map { iter: SemicircleFilled { x: -20, y: 0, err: -38 } }) }, frontiter: None, backiter: None } }");
}

#[test]
fn test_semicircle_semicircle_top_clone() {
    let semicircle_top_iter = SemicircleFilled::<isize>::new(6_usize).semicircle_top();
    let semicircle_top_iter_clone = semicircle_top_iter.clone();
    let semicircle_top_vec = semicircle_top_iter.collect::<Vec<_>>();
    let semicircle_top_clone_vec = semicircle_top_iter_clone.collect::<Vec<_>>();
    assert_eq!(semicircle_top_vec, semicircle_top_clone_vec);
}

#[test]
fn test_semicircle_semicircle_top_four() {
    let semicircle_top_iter = SemicircleFilled::<isize>::new(4_u64).semicircle_top();
    let semicircle_top_vec = semicircle_top_iter.collect::<Vec<_>>();
    assert_eq!(semicircle_top_vec,
        vec![(-4, -1), (-3, -1), (-2, -1), (-1, -1), (0, -1),
        (1, -1), (2, -1), (3, -1), (-4, -2), (-3, -2), (-2, -2),
        (-1, -2), (0, -2), (1, -2), (2, -2), (3, -2), (-3, -3),
        (-2, -3), (-1, -3), (0, -3), (1, -3), (2, -3), (-2, -4),
        (-1, -4), (0, -4), (1, -4)]);
}

#[test]
fn test_semicircle_semicircle_bottom_empty() {
    let semicircle_bottom_iter = SemicircleFilled::<i32>::new(0_u64).semicircle_bottom();
    let semicircle_bottom_vec = semicircle_bottom_iter.collect::<Vec<_>>();
    assert_eq!(semicircle_bottom_vec, vec![]);
}

#[test]
fn test_semicircle_semicircle_bottom_debug() {
    let semicircle_bottom_iter = SemicircleFilled::<i32>::new(20_u128).semicircle_bottom();
    assert_eq!(format!("{:?}", semicircle_bottom_iter), "FlatMap { inner: FlattenCompat { iter: Fuse { iter: Some(\
            Map { iter: SemicircleFilled { x: -20, y: 0, err: -38 } }) }, frontiter: None, backiter: None } }");
}

#[test]
fn test_semicircle_semicircle_bottom_clone() {
    let semicircle_bottom_iter = SemicircleFilled::<i32>::new(6_u8).semicircle_bottom();
    let semicircle_bottom_iter_clone = semicircle_bottom_iter.clone();
    let semicircle_bottom_vec = semicircle_bottom_iter.collect::<Vec<_>>();
    let semicircle_bottom_clone_vec = semicircle_bottom_iter_clone.collect::<Vec<_>>();
    assert_eq!(semicircle_bottom_vec, semicircle_bottom_clone_vec);
}

#[test]
fn test_semicircle_semicircle_bottom_four() {
    let semicircle_bottom_iter = SemicircleFilled::<i32>::new(4_u16).semicircle_bottom();
    let semicircle_bottom_vec = semicircle_bottom_iter.collect::<Vec<_>>();
    assert_eq!(semicircle_bottom_vec,
        vec![(-4, 0), (-3, 0), (-2, 0), (-1, 0), (0, 0), (1, 0),
        (2, 0), (3, 0), (-4, 1), (-3, 1), (-2, 1), (-1, 1), (0, 1),
        (1, 1), (2, 1), (3, 1), (-3, 2), (-2, 2), (-1, 2), (0, 2),
        (1, 2), (2, 2), (-2, 3), (-1, 3), (0, 3), (1, 3)]);
}

#[test]
fn test_semicircle_semicircle_top_long_empty() {
    let iter = SemicircleFilled::<i32>::new(0_u32).semicircle_top_long(0, 0);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![]);
    let iter = SemicircleFilled::<i32>::new(0_u32).semicircle_top_long(-20, 20);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![]);
}

#[test]
fn test_semicircle_semicircle_top_long_debug() {
    let iter = SemicircleFilled::<isize>::new(100_usize).semicircle_top_long(-10, 10);
    assert_eq!(format!("{:?}", iter), "FlatMap { inner: FlattenCompat { iter: Fuse { iter: Some(\
            Map { iter: SemicircleFilled { x: -100, y: 0, err: -198 } }) }, frontiter: None, backiter: None } }");
}

#[test]
fn test_semicircle_semicircle_top_long_four() {
    let iter = SemicircleFilled::<i32>::new(4_u32).semicircle_top_long(-2, 2);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec,
        vec![(-6, -1), (-5, -1), (-4, -1), (-3, -1), (-2, -1),
        (-1, -1), (0, -1), (1, -1), (2, -1), (3, -1), (4, -1),
        (5, -1), (-6, -2), (-5, -2), (-4, -2), (-3, -2), (-2, -2),
        (-1, -2), (0, -2), (1, -2), (2, -2), (3, -2), (4, -2),
        (5, -2), (-5, -3), (-4, -3), (-3, -3), (-2, -3), (-1, -3),
        (0, -3), (1, -3), (2, -3), (3, -3), (4, -3), (-4, -4),
        (-3, -4), (-2, -4), (-1, -4), (0, -4), (1, -4), (2, -4), (3, -4)]);

    let iter = SemicircleFilled::<i32>::new(4_u32).semicircle_top_long(2, -2);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec,
        vec![(-2, -1), (-1, -1), (0, -1), (1, -1), (-2, -2),
        (-1, -2), (0, -2), (1, -2), (-1, -3), (0, -3)]);

    let iter = SemicircleFilled::<i32>::new(4_u16).semicircle_top_long(4, -4);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec,
        vec![]);
}

#[test]
fn test_semicircle_semicircle_bottom_long_empty() {
    let iter = SemicircleFilled::<i32>::new(0_u32).semicircle_bottom_long(0, 0);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![]);
    let iter = SemicircleFilled::<i32>::new(0_u32).semicircle_bottom_long(-20, 20);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![]);
}

#[test]
fn test_semicircle_semicircle_botton_long_debug() {
    let iter = SemicircleFilled::<isize>::new(100_usize).semicircle_bottom_long(-10, 10);
    assert_eq!(format!("{:?}", iter), "FlatMap { inner: FlattenCompat { iter: Fuse { iter: Some(\
            Map { iter: SemicircleFilled { x: -100, y: 0, err: -198 } }) }, frontiter: None, backiter: None } }");
}

#[test]
fn test_semicircle_semicircle_bottom_long_five() {
    let iter = SemicircleFilled::<i32>::new(5_u8).semicircle_bottom_long(-2, 2);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec,
        vec![(-7, 0), (-6, 0), (-5, 0), (-4, 0), (-3, 0), (-2, 0),
        (-1, 0), (0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0),
        (6, 0), (-7, 1), (-6, 1), (-5, 1), (-4, 1), (-3, 1), (-2, 1),
        (-1, 1), (0, 1), (1, 1), (2, 1), (3, 1), (4, 1), (5, 1),
        (6, 1), (-6, 2), (-5, 2), (-4, 2), (-3, 2), (-2, 2), (-1, 2),
        (0, 2), (1, 2), (2, 2), (3, 2), (4, 2), (5, 2), (-5, 3),
        (-4, 3), (-3, 3), (-2, 3), (-1, 3), (0, 3), (1, 3), (2, 3),
        (3, 3), (4, 3), (-4, 4), (-3, 4), (-2, 4), (-1, 4), (0, 4),
        (1, 4), (2, 4), (3, 4)]);

    let iter = SemicircleFilled::<i32>::new(5_u16).semicircle_bottom_long(2, -2);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec,
        vec![(-3, 0), (-2, 0), (-1, 0), (0, 0), (1, 0), (2, 0),
        (-3, 1), (-2, 1), (-1, 1), (0, 1), (1, 1), (2, 1), (-2, 2),
        (-1, 2), (0, 2), (1, 2), (-1, 3), (0, 3)]);

    let iter = SemicircleFilled::<i32>::new(5_u16).semicircle_bottom_long(5, -5);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec,
        vec![]);
}

#[test]
fn test_semicircle_first_quadrant_empty() {
    let iter = SemicircleFilled::<i32>::new(0_u32).first_quadrant(0);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![]);
    let iter = SemicircleFilled::<i32>::new(0_u32).first_quadrant(10);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![]);
    let iter = SemicircleFilled::<i32>::new(0_u32).first_quadrant(-10);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![]);
}

#[test]
fn test_semicircle_first_quadrant_four() {
    let iter = SemicircleFilled::<i32>::new(4_u32).first_quadrant(0);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec,
        vec![(0, -1), (1, -1), (2, -1), (3, -1), (0, -2), (1, -2),
        (2, -2), (3, -2), (0, -3), (1, -3), (2, -3), (0, -4), (1, -4)]);
}

#[test]
fn test_semicircle_first_quadrant_debug() {
    let iter = SemicircleFilled::<isize>::new(8_usize).first_quadrant(0);
    assert_eq!(format!("{:?}", iter), "FlatMap { inner: FlattenCompat { iter: Fuse { iter: Some(\
            Map { iter: SemicircleFilled { x: -8, y: 0, err: -14 } }) }, frontiter: None, backiter: None } }");
}

#[test]
fn test_semicircle_first_quadrant_clone() {
    let iter = SemicircleFilled::<isize>::new(6_usize).first_quadrant(10);
    let iter_clone = iter.clone();
    let vec = iter.collect::<Vec<_>>();
    let clone_vec = iter_clone.collect::<Vec<_>>();
    assert_eq!(vec, clone_vec);
}

#[test]
fn test_semicircle_first_quadrant_four_long() {
    let iter = SemicircleFilled::<i32>::new(4_u32).first_quadrant(4);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec,
        vec![(0, -1), (1, -1), (2, -1), (3, -1), (4, -1), (5, -1),
        (6, -1), (7, -1), (0, -2), (1, -2), (2, -2), (3, -2),
        (4, -2), (5, -2), (6, -2), (7, -2), (0, -3), (1, -3),
        (2, -3), (3, -3), (4, -3), (5, -3), (6, -3), (0, -4),
        (1, -4), (2, -4), (3, -4), (4, -4), (5, -4)]);

    let iter = SemicircleFilled::<i32>::new(4_u32).first_quadrant(-2);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec,
        vec![(0, -1), (1, -1), (0, -2), (1, -2), (0, -3)]);

    let iter = SemicircleFilled::<i32>::new(4_u32).first_quadrant(-4);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec,
        vec![]);
}

#[test]
fn test_semicircle_second_quadrant_empty() {
    let iter = SemicircleFilled::<i32>::new(0_u32).second_quadrant(0);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![]);
    let iter = SemicircleFilled::<i32>::new(0_u32).second_quadrant(10);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![]);
    let iter = SemicircleFilled::<i32>::new(0_u32).second_quadrant(-10);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![]);
}

#[test]
fn test_semicircle_second_quadrant_five() {
    let iter = SemicircleFilled::<i32>::new(5_u32).second_quadrant(0);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec,
        vec![(-5, -1), (-4, -1), (-3, -1), (-2, -1), (-1, -1),
        (-5, -2), (-4, -2), (-3, -2), (-2, -2), (-1, -2), (-4, -3),
        (-3, -3), (-2, -3), (-1, -3), (-3, -4), (-2, -4), (-1, -4),
        (-2, -5), (-1, -5)]);
}

#[test]
fn test_semicircle_second_quadrant_debug() {
    let iter = SemicircleFilled::<isize>::new(8_usize).second_quadrant(0);
    assert_eq!(format!("{:?}", iter), "FlatMap { inner: FlattenCompat { iter: Fuse { iter: Some(\
            Map { iter: SemicircleFilled { x: -8, y: 0, err: -14 } }) }, frontiter: None, backiter: None } }");
}

#[test]
fn test_semicircle_second_quadrant_clone() {
    let iter = SemicircleFilled::<isize>::new(6_usize).second_quadrant(10);
    let iter_clone = iter.clone();
    let vec = iter.collect::<Vec<_>>();
    let clone_vec = iter_clone.collect::<Vec<_>>();
    assert_eq!(vec, clone_vec);
}

#[test]
fn test_semicircle_second_quadrant_five_long() {
    let iter = SemicircleFilled::<i32>::new(5_u32).second_quadrant(-2);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec,
        vec![(-5, -1), (-4, -1), (-3, -1), (-5, -2), (-4, -2),
        (-3, -2), (-4, -3), (-3, -3), (-3, -4)]);

    let iter = SemicircleFilled::<i32>::new(5_u32).second_quadrant(-5);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec,
        vec![]);

    let iter = SemicircleFilled::<i32>::new(5_u32).second_quadrant(1);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec,
        vec![(-5, -1), (-4, -1), (-3, -1), (-2, -1), (-1, -1),
        (0, -1), (-5, -2), (-4, -2), (-3, -2), (-2, -2), (-1, -2),
        (0, -2), (-4, -3), (-3, -3), (-2, -3), (-1, -3), (0, -3),
        (-3, -4), (-2, -4), (-1, -4), (0, -4), (-2, -5), (-1, -5), (0, -5)]);
}

#[test]
fn test_semicircle_third_quadrant_empty() {
    let iter = SemicircleFilled::<i32>::new(0_u32).third_quadrant(0);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![]);
    let iter = SemicircleFilled::<i32>::new(0_u32).third_quadrant(10);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![]);
    let iter = SemicircleFilled::<i32>::new(0_u32).third_quadrant(-10);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![]);
}

#[test]
fn test_semicircle_third_quadrant_five() {
    let iter = SemicircleFilled::<i32>::new(5_u32).third_quadrant(0);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec,
        vec![(-5, 0), (-4, 0), (-3, 0), (-2, 0), (-1, 0), (-5, 1),
        (-4, 1), (-3, 1), (-2, 1), (-1, 1), (-4, 2), (-3, 2),
        (-2, 2), (-1, 2), (-3, 3), (-2, 3), (-1, 3), (-2, 4), (-1, 4)]);
}

#[test]
fn test_semicircle_third_quadrant_debug() {
    let iter = SemicircleFilled::<isize>::new(8_usize).third_quadrant(0);
    assert_eq!(format!("{:?}", iter), "FlatMap { inner: FlattenCompat { iter: Fuse { iter: Some(\
            Map { iter: SemicircleFilled { x: -8, y: 0, err: -14 } }) }, frontiter: None, backiter: None } }");
}

#[test]
fn test_semicircle_third_quadrant_clone() {
    let iter = SemicircleFilled::<isize>::new(6_usize).third_quadrant(10);
    let iter_clone = iter.clone();
    let vec = iter.collect::<Vec<_>>();
    let clone_vec = iter_clone.collect::<Vec<_>>();
    assert_eq!(vec, clone_vec);
}

#[test]
fn test_semicircle_third_quadrant_five_long() {
    let iter = SemicircleFilled::<i32>::new(5_u32).third_quadrant(-3);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec,
        vec![(-5, 0), (-4, 0), (-5, 1), (-4, 1), (-4, 2)]);

    let iter = SemicircleFilled::<i32>::new(5_u32).third_quadrant(-5);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec,
        vec![]);

    let iter = SemicircleFilled::<i32>::new(5_u32).third_quadrant(2);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec,
        vec![(-5, 0), (-4, 0), (-3, 0), (-2, 0), (-1, 0), (0, 0),
        (1, 0), (-5, 1), (-4, 1), (-3, 1), (-2, 1), (-1, 1), (0, 1),
        (1, 1), (-4, 2), (-3, 2), (-2, 2), (-1, 2), (0, 2), (1, 2),
        (-3, 3), (-2, 3), (-1, 3), (0, 3), (1, 3), (-2, 4), (-1, 4),
        (0, 4), (1, 4)]);
}

#[test]
fn test_semicircle_fourth_quadrant_empty() {
    let iter = SemicircleFilled::<i32>::new(0_u32).fourth_quadrant(0);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![]);
    let iter = SemicircleFilled::<i32>::new(0_u32).fourth_quadrant(10);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![]);
    let iter = SemicircleFilled::<i32>::new(0_u32).fourth_quadrant(-10);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, vec![]);
}

#[test]
fn test_semicircle_fourth_quadrant_five() {
    let iter = SemicircleFilled::<i32>::new(5_u32).fourth_quadrant(0);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec,
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (0, 1), (1, 1),
        (2, 1), (3, 1), (4, 1), (0, 2), (1, 2), (2, 2), (3, 2),
        (0, 3), (1, 3), (2, 3), (0, 4), (1, 4)]);
}

#[test]
fn test_semicircle_fourth_quadrant_debug() {
    let iter = SemicircleFilled::<isize>::new(8_usize).fourth_quadrant(0);
    assert_eq!(format!("{:?}", iter), "FlatMap { inner: FlattenCompat { iter: Fuse { iter: Some(\
            Map { iter: SemicircleFilled { x: -8, y: 0, err: -14 } }) }, frontiter: None, backiter: None } }");
}

#[test]
fn test_semicircle_fourth_quadrant_clone() {
    let iter = SemicircleFilled::<isize>::new(6_usize).fourth_quadrant(10);
    let iter_clone = iter.clone();
    let vec = iter.collect::<Vec<_>>();
    let clone_vec = iter_clone.collect::<Vec<_>>();
    assert_eq!(vec, clone_vec);
}

#[test]
fn test_semicircle_fourth_quadrant_five_long() {
    let iter = SemicircleFilled::<i32>::new(5_u32).fourth_quadrant(-2);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec,
        vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1), (0, 2),
        (1, 2), (0, 3)]);

    let iter = SemicircleFilled::<i32>::new(5_u32).fourth_quadrant(-5);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec,
        vec![]);

    let iter = SemicircleFilled::<i32>::new(5_u32).fourth_quadrant(3);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec,
        vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0),
        (7, 0), (0, 1), (1, 1), (2, 1), (3, 1), (4, 1), (5, 1),
        (6, 1), (7, 1), (0, 2), (1, 2), (2, 2), (3, 2), (4, 2),
        (5, 2), (6, 2), (0, 3), (1, 3), (2, 3), (3, 3), (4, 3),
        (5, 3), (0, 4), (1, 4), (2, 4), (3, 4), (4, 4)]);
}


#[test]
fn test_semicircle_semicircle_top_empty_rev() {
    let semicircle_top_iter = SemicircleFilled::<i32>::new(0_u32)
        .rev()
        .semicircle_top();
    let semicircle_top_vec = semicircle_top_iter.collect::<Vec<_>>();
    assert_eq!(semicircle_top_vec, vec![]);
}

#[test]
fn test_semicircle_semicircle_top_debug_rev() {
    let semicircle_top_iter = SemicircleFilled::<i64>::new(20_u128)
        .rev()
        .semicircle_top();
    assert_eq!(format!("{:?}", semicircle_top_iter), "FlatMap { inner: FlattenCompat { iter: Fuse { iter: Some(\
        Map { iter: Rev { iter: SemicircleFilled { x: -20, y: 0, err: -38 } } }) }, frontiter: None, backiter: None } }");
}

#[test]
fn test_semicircle_isize_semicircle_top_clone_rev() {
    let semicircle_top_iter = SemicircleFilled::<isize>::new(6_usize)
        .rev()
        .semicircle_top();
    let semicircle_top_iter_clone = semicircle_top_iter.clone();
    let semicircle_top_vec = semicircle_top_iter.collect::<Vec<_>>();
    let semicircle_top_clone_vec = semicircle_top_iter_clone.collect::<Vec<_>>();
    assert_eq!(semicircle_top_vec, semicircle_top_clone_vec);
}

#[test]
fn test_semicircle_i32_semicircle_top_clone_rev() {
    let semicircle_top_iter = SemicircleFilled::<i32>::new(100_usize)
        .rev()
        .semicircle_top();
    let semicircle_top_iter_clone = semicircle_top_iter.clone();
    let semicircle_top_vec = semicircle_top_iter.collect::<Vec<_>>();
    let semicircle_top_clone_vec = semicircle_top_iter_clone.collect::<Vec<_>>();
    assert_eq!(semicircle_top_vec, semicircle_top_clone_vec);
}

#[test]
fn test_semicircle_i64_semicircle_top_clone_rev() {
    let semicircle_top_iter = SemicircleFilled::<i64>::new(5_usize)
        .rev()
        .semicircle_top();
    let semicircle_top_iter_clone = semicircle_top_iter.clone();
    let semicircle_top_vec = semicircle_top_iter.collect::<Vec<_>>();
    let semicircle_top_clone_vec = semicircle_top_iter_clone.collect::<Vec<_>>();
    assert_eq!(semicircle_top_vec, semicircle_top_clone_vec);
}


#[test]
fn test_semicircle_isize_semicircle_top_five_rev() {
    let semicircle_top_iter = SemicircleFilled::<isize>::new(5_u8)
        .rev()
        .semicircle_top();
    let semicircle_top_vec = semicircle_top_iter.collect::<Vec<_>>();
    assert_eq!(semicircle_top_vec,
        vec![(-2, -5), (-1, -5), (0, -5), (1, -5), (-3, -4), (-2, -4),
        (-1, -4), (0, -4), (1, -4), (2, -4), (-4, -3), (-3, -3), (-2, -3), (-1, -3),
        (0, -3), (1, -3), (2, -3), (3, -3), (-5, -2), (-4, -2), (-3, -2), (-2, -2),
        (-1, -2), (0, -2), (1, -2), (2, -2), (3, -2), (4, -2), (-5, -1), (-4, -1),
        (-3, -1), (-2, -1), (-1, -1), (0, -1), (1, -1), (2, -1), (3, -1), (4, -1)]);
}

#[test]
fn test_semicircle_i32_semicircle_top_five_rev() {
    let semicircle_top_iter = SemicircleFilled::<i32>::new(5_u16)
        .rev()
        .semicircle_top();
    let semicircle_top_vec = semicircle_top_iter.collect::<Vec<_>>();
    assert_eq!(semicircle_top_vec,
        vec![(-2, -5), (-1, -5), (0, -5), (1, -5), (-3, -4), (-2, -4),
        (-1, -4), (0, -4), (1, -4), (2, -4), (-4, -3), (-3, -3), (-2, -3), (-1, -3),
        (0, -3), (1, -3), (2, -3), (3, -3), (-5, -2), (-4, -2), (-3, -2), (-2, -2),
        (-1, -2), (0, -2), (1, -2), (2, -2), (3, -2), (4, -2), (-5, -1), (-4, -1),
        (-3, -1), (-2, -1), (-1, -1), (0, -1), (1, -1), (2, -1), (3, -1), (4, -1)]);
}

#[test]
fn test_semicircle_i64_semicircle_top_five_rev() {
    let semicircle_top_iter = SemicircleFilled::<i64>::new(5_u64)
        .rev()
        .semicircle_top();
    let semicircle_top_vec = semicircle_top_iter.collect::<Vec<_>>();
    assert_eq!(semicircle_top_vec,
        vec![(-2, -5), (-1, -5), (0, -5), (1, -5), (-3, -4), (-2, -4),
        (-1, -4), (0, -4), (1, -4), (2, -4), (-4, -3), (-3, -3), (-2, -3), (-1, -3),
        (0, -3), (1, -3), (2, -3), (3, -3), (-5, -2), (-4, -2), (-3, -2), (-2, -2),
        (-1, -2), (0, -2), (1, -2), (2, -2), (3, -2), (4, -2), (-5, -1), (-4, -1),
        (-3, -1), (-2, -1), (-1, -1), (0, -1), (1, -1), (2, -1), (3, -1), (4, -1)]);
}

#[test]
fn test_semicircle_circle_offset_empty() {
    let circle_iter = SemicircleFilled::<isize>::new(0_u8).circle().offset(0, 0);
    let circle_vec = circle_iter.collect::<Vec<_>>();
    assert_eq!(circle_vec, vec![]);
}

#[test]
fn test_semicircle_circle_offset_debug() {
    let circle_iter = SemicircleFilled::<isize>::new(7_u16).circle().offset(7, 7);
    assert_eq!(format!("{:?}", circle_iter), "Map { iter: FlatMap { inner: FlattenCompat { iter: Fuse { iter: Some(\
            Map { iter: SemicircleFilled { x: -7, y: 0, err: -12 } }) }, frontiter: None, backiter: None } } }");
}

#[test]
fn test_semicircle_circle_offset_clone() {
    let circle_iter = SemicircleFilled::<isize>::new(6_u32).circle().offset(6, 6);
    let circle_iter_clone = circle_iter.clone();
    let circle_vec = circle_iter.collect::<Vec<_>>();
    let circle_clone_vec = circle_iter_clone.collect::<Vec<_>>();
    assert_eq!(circle_vec, circle_clone_vec);
}

#[test]
fn test_semicircle_circle_offset_three() {
    let iter = SemicircleFilled::<i32>::new(3_u64).circle().offset(3, 3);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, 
        vec![(0, 2), (0, 3), (1, 2), (1, 3), (2, 2), (2, 3),
        (3, 2), (3, 3), (4, 2), (4, 3), (5, 2), (5, 3), (0, 1),
        (0, 4), (1, 1), (1, 4), (2, 1), (2, 4), (3, 1),
        (3, 4), (4, 1), (4, 4), (5, 1), (5, 4), (1, 0), (1, 5),
        (2, 0), (2, 5), (3, 0), (3, 5), (4, 0), (4, 5)]);
}
#[test]
fn test_semicircle_circle_offset_one() {
    let iter = SemicircleFilled::<i32>::new(1_usize).circle().offset(1, 1);
    let vec = iter.collect::<Vec<_>>();
    assert_eq!(vec, 
        vec![(0, 0), (0, 1), (1, 0), (1, 1)]);
}
