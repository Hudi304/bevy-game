use bevy::{
    prelude::{shape::Circle, *},
    render::mesh::Indices,
};

use super::quad::WorldCircle;

/// Returns an array with length 3 * num_of_edges \
/// that represents the order in which the edges of a \
/// mesh should be constructed.
///
/// # Examples
///
/// use bevy::prelude::*;
///
/// let indices : Indices = build_circle_indices(4);
///
/// assert_eq!(indices.len(), 0);
///
pub fn _build_circle_indices(num_of_edges: u32) -> Indices {
    let mut result: Vec<u32> = Vec::with_capacity(3 * num_of_edges as usize);

    for i in 1..num_of_edges {
        result.push(0);
        result.push(i);
        result.push(i + 1);
    }

    return Indices::U32(result);
}

pub fn spawn_circ_bevy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let circ = Circle::new(0.5);
    let circ_mesh: Mesh = circ.into();

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(circ_mesh),
            material: materials.add(Color::PURPLE.into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.1),
            ..default()
        },
        WorldCircle {},
    ));
}
