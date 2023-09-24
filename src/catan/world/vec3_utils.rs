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

pub fn remove_vec3_duplicates(all_filtered_vertices: &Vec<Vec3>, eps: f32) -> Vec<Vec3> {
    let mut city_points_iter = all_filtered_vertices.iter();

    let mut unique_city_positions = Vec::new();

    let mut prev_city_pos = city_points_iter.next().unwrap();

    unique_city_positions.push(prev_city_pos.clone());

    for city_pos in city_points_iter {
        let ord = sort_positions(prev_city_pos, city_pos, eps);

        if ord != Ordering::Equal {
            unique_city_positions.push(city_pos.clone());
        }

        prev_city_pos = city_pos;
    }
    unique_city_positions
}

pub fn print_vec_vec3(arr: &Vec<Vec3>) {
    for pt in arr.iter() {
        println!("{:.2},{:.2}", pt.x, pt.y);
    }
}
