use std::{ f32::consts::PI, ops::Range };

use bevy::prelude::{ shape::{ Circle, Quad }, * };

use bevy_mod_picking::{
    prelude::{ On, Out, Over, Pointer, RaycastPickTarget, ListenerInput },
    PickableBundle,
};

use crate::catan::{
    cubic_coords::cube_coordinates::CubicCoord,
    world::{
        city::city_placer_mesh::get_city_positions,
        road::road_placer_mesh::get_road_positions,
    },
};

use super::{
    land_tile::{ build_tile_mesh, LandTile, NUMBER_OF_TILES },
    randomize_richness::build_random_tile_richness_array,
    randomize_tiles::build_random_tile_type_array,
    tile_type::TileType,
    water_tile::WaterTile,
};

pub const TILE_OFFSET_ANGLE_RAD: f32 = PI / 6.0; // 30deg
pub const TILE_EPSILON: f32 = 0.01;
pub const MAP_SIZE: i32 = 7;

pub fn build_cub_coord_hex_gird(radius: i32) -> Vec<CubicCoord> {
    let mut hex_arr = vec![];
    let slice: Range<i32> = -radius..radius + 1;

    for q in slice.clone() {
        for r in slice.clone() {
            let s: i32 = 0 - q - r;

            if s.abs() > radius {
                continue;
            }

            hex_arr.push(CubicCoord::from_tuple((q, r, s)));
        }
    }

    return hex_arr;
}

// I think you can also compute this with some kind of tree
// or trying not to overlap the city positions
// but I have no idea if that's worth it or faster

pub fn spawn_land_tiles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let cub_coords_arr: Vec<CubicCoord> = build_cub_coord_hex_gird(MAP_SIZE);

    let mut i = 0;
    let tile_type_arr = build_random_tile_type_array(NUMBER_OF_TILES);
    let richness_arr = build_random_tile_richness_array(NUMBER_OF_TILES);

    let mut land_tile_arr = Vec::with_capacity(NUMBER_OF_TILES);

    let on_over = |_: &ListenerInput<Pointer<Over>>, transform: &mut Transform| {
        let mut old_translation = transform.translation;
        old_translation.z = 0.2;
        transform.translation = old_translation;
    };

    let on_out = |_: &ListenerInput<Pointer<Out>>, transform: &mut Transform| {
        let mut old_translation = transform.translation;
        old_translation.z = 0.0;
        transform.translation = old_translation;
    };

    for cub_coord in cub_coords_arr {
        if cub_coord.ring < 3 {
            // Land tile
            let tile_type = tile_type_arr[i];
            let tile_richness = richness_arr[i];

            let material = materials.add(tile_type.into_color().into());

            let mesh: Handle<Mesh> = meshes.add(build_tile_mesh(TILE_OFFSET_ANGLE_RAD));

            let land_tile = LandTile::build(
                cub_coord,
                tile_type,
                tile_richness,
                TILE_OFFSET_ANGLE_RAD
            );

            land_tile_arr.push(land_tile.clone());

            commands.spawn((
                PbrBundle {
                    mesh,
                    material,
                    transform: Transform::from_translation(land_tile.cart_coord),
                    ..default()
                },
                land_tile.clone(),

                PickableBundle::default(),
                RaycastPickTarget::default(),

                On::<Pointer<Over>>::target_component_mut::<Transform>(on_over),
                On::<Pointer<Out>>::target_component_mut::<Transform>(on_out),
            ));

            i += 1;
        } else {
            // Water tile
            let material = materials.add(Color::BLUE.into());
            let mesh: Handle<Mesh> = meshes.add(build_tile_mesh(TILE_OFFSET_ANGLE_RAD));
            let ent = WaterTile::build(cub_coord, material, mesh, TileType::WATER, 0);
            commands.spawn(ent);
        }
    }

    //? CITIES
    let city_mesh = Circle::new(0.2);
    let unique_city_positions = get_city_positions(&land_tile_arr);

    for vertex in unique_city_positions.iter() {
        let translation = Vec3 {
            x: vertex.x,
            y: vertex.y,
            z: 0.15,
        };

        commands.spawn(PbrBundle {
            mesh: meshes.add(city_mesh.into()),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_translation(translation),
            ..default()
        });
    }

    //? ROADS
    let road_mesh = Quad::new(Vec2 { x: 0.6, y: 0.15 });
    let unique_road_positions = get_road_positions(&land_tile_arr);
    // assert_eq!(unique_road_positions.len(), 72);

    for (vertex, angle) in unique_road_positions.iter() {
        let translation = Vec3 {
            x: vertex.x,
            y: vertex.y,
            z: 0.15,
        };

        let transform = Transform::from_translation(translation);
        let rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, *angle);

        commands.spawn(PbrBundle {
            mesh: meshes.add(road_mesh.into()),
            material: materials.add(Color::WHITE.into()),
            transform: transform.with_rotation(rotation),
            ..default()
        });
    }
}
