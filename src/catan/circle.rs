use bevy::prelude::*;

use bevy::prelude::shape::Circle;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;

use crate::hex::polygon::vec3_to_vec;

#[derive(Component)]
pub struct WorldCircle {}

// TODO generalize this into perfect polygon helpers
// TODO search for a way to do this in bevy, especially the mesh
pub fn spawn_3d_quad(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let num_points = 4;
    let radius: f32 = 2.;

    let mut circle_vertices = vec![];
    for i in 0..=num_points {
        let angle = (i as f32 / num_points as f32) * 2.0 * std::f32::consts::PI;
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        circle_vertices.push(Vec3::new(x, y, 0.0));
    }

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();

    for vertex_position in circle_vertices.iter() {
        positions.push(vec3_to_vec(vertex_position));
        normals.push(vec3_to_vec(&Vec3::Z));
        uvs.push([0., 0.]);
    }

    let indices = build_circle_indices(num_points);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(indices));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::PURPLE.into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        WorldCircle {},
    ));
}

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
fn build_circle_indices(num_of_edges: u32) -> Indices {
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
