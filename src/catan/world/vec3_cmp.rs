use std::cmp::Ordering;

use bevy::prelude::Vec3;

use super::f32_cmp::compare_floats;

pub fn sort_positions(v1: &Vec3, v2: &Vec3, eps: f32) -> Ordering {
    let x_comparison = compare_floats(v1.x, v2.x, eps);

    if x_comparison == Ordering::Equal {
        return compare_floats(v1.y, v2.y, eps);
    }

    x_comparison
}
