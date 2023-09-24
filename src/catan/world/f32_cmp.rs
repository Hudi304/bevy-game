use std::cmp::Ordering;

pub fn compare_floats(a: f32, b: f32, eps: f32) -> Ordering {
    let diff = (a - b).abs();
    if diff < eps {
        Ordering::Equal
    } else if a < b {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}
