use bevy::{prelude::*, render::render_resource::PrimitiveTopology};

use crate::catan::utils::vec::vec3_to_vec;

use super::circle::_build_circle_indices;

#[derive(Component)]
pub struct WorldCircle {}

// TODO generalize this into perfect polygon helpers
// TODO search for a way to do this in bevy, especially the mesh
pub fn _spawn_3d_quad(
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

    let indices = _build_circle_indices(num_points);

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
