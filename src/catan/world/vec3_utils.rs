use std::cmp::Ordering;

use bevy::prelude::Vec3;

use super::{ f32_cmp::compare_floats, spawn_tiles::TILE_EPSILON };

pub fn sort_positions(v1: &Vec3, v2: &Vec3, eps: f32) -> Ordering {
    let x_comparison = compare_floats(v1.x, v2.x, eps);

    if x_comparison == Ordering::Equal {
        return compare_floats(v1.y, v2.y, eps);
    }

    x_comparison
}

/// Removes duplicate Vec3 vertices from a given vector based on a specified epsilon (eps).
///
/// This function takes a vector of Vec3 vertices (`all_filtered_vertices`) and removes
/// duplicates within the given epsilon tolerance (`eps`). The resulting vector contains
/// only unique vertices.
///
/// # Arguments
///
/// * `all_filtered_vertices` - A reference to a vector of Vec3 vertices from which duplicates
///                            will be removed.
/// * `eps` - A floating-point value representing the tolerance for considering two vertices
///           as duplicates. Vertices with differences within `eps` are considered equal.
///
/// # Returns
///
/// A new vector containing only the unique Vec3 vertices from `all_filtered_vertices`.
///
/// # Example
///
/// ```
/// use your_module::remove_vec3_duplicates;
///
/// let all_filtered_vertices = vec![
///     Vec3 { x: 1.0, y: 2.0, z: 3.0 },
///     Vec3 { x: 1.1, y: 2.2, z: 3.3 },
///     Vec3 { x: 1.0, y: 2.0, z: 3.0 },
///     // Add more Vec3 vertices here...
/// ];
///
/// let eps = 0.01;
/// let unique_vertices = remove_vec3_duplicates(&all_filtered_vertices, eps);
///
/// // Now `unique_vertices` contains only the unique vertices.
/// ```
//TODO maybe consume the vertex array so that is can be sorted directly here
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
