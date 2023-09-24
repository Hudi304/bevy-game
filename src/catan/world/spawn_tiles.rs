use std::{ cmp::Ordering, f32::consts::PI, ops::Range };

use bevy::prelude::{ shape::{ Circle, Quad }, * };

use bevy_mod_picking::{ prelude::{ On, Out, Over, Pointer, RaycastPickTarget }, PickableBundle };

use crate::catan::{
    cubic_coords::cube_coordinates::CubicCoord,
    world::vec3_utils::{ remove_vec3_duplicates, print_vec_vec3 },
};

use super::{
    land_tile::{ build_tile_mesh, LandTile, NUMBER_OF_TILES },
    randomize_richness::build_random_tile_richness_array,
    randomize_tiles::build_random_tile_type_array,
    tile_type::TileType,
    water_tile::WaterTile,
    vec3_utils::sort_positions,
};

pub const TILE_OFFSET_ANGLE_RAD: f32 = PI / 6.0; // 30deg

// I think you can also compute this with some kind of tree
// or trying not to overlap the city positions
// but I have no idea if that's worth it or faster

pub fn spawn_land_tiles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // let h = 3_f32.sqrt() / 2. * 1.01;

    println!("spawn_land_tiles");

    let cub_coords_arr: Vec<CubicCoord> = build_cub_coord_hex_gird(7);
    let eps = 0.01;

    let mut i = 0;
    let tile_type_arr = build_random_tile_type_array(NUMBER_OF_TILES);
    let richness_arr = build_random_tile_richness_array(NUMBER_OF_TILES);

    let mut land_tile_arr = Vec::with_capacity(NUMBER_OF_TILES);

    for cub_coord in cub_coords_arr {
        // TODO refactor this so the HexWorldTile contains the material asset and the mesh asset
        if cub_coord.ring < 3 {
            // Land tile
            let tile_type = tile_type_arr[i];
            let tile_richness = richness_arr[i];

            let material = materials.add(tile_type.into_color().into());

            let mesh: Handle<Mesh> = meshes.add(build_tile_mesh(TILE_OFFSET_ANGLE_RAD));

            let (prb_bundle, land_tile) = LandTile::build(
                cub_coord,
                material,
                mesh,
                tile_type,
                tile_richness,
                TILE_OFFSET_ANGLE_RAD
            );

            land_tile_arr.push(land_tile.clone());

            // let quad = Quad::new(Vec2 { x: 0.15, y: 0.6 });
            // let mut angle = PI / 3.;
            // for vertex in component.edges.iter() {
            //     commands.spawn(PbrBundle {
            //         mesh: meshes.add(quad.into()),
            //         material: materials.add(Color::WHITE.into()),
            //         transform: Transform::from_translation(Vec3 {
            //             x: vertex.x,
            //             y: vertex.y,
            //             z: 0.15,
            //         })
            //         .with_rotation(Quat::from_euler(
            //             EulerRot::XYZ,
            //             0.0,
            //             0.0,
            //             angle,
            //         )),
            //         ..default()
            //     });

            //     angle += PI / 3.;
            // }

            commands.spawn((
                prb_bundle,
                land_tile.clone(),
                PickableBundle::default(),
                On::<Pointer<Over>>::target_component_mut::<Transform>(|_, transform| {
                    let mut old_translation = transform.translation;
                    old_translation.z = 0.2;
                    transform.translation = old_translation;
                }),
                On::<Pointer<Out>>::target_component_mut::<Transform>(|_, transform| {
                    let mut old_translation = transform.translation;
                    old_translation.z = 0.0;
                    transform.translation = old_translation;
                }),
                RaycastPickTarget::default(),
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

    // All the hex vertices
    let mut all_tile_vertices: Vec<Vec3> = land_tile_arr
        .iter()
        .flat_map(|tile| tile.vertices.iter().cloned())
        .collect();

    // sorted array of hex vertices, sorted by x,y
    all_tile_vertices.sort_by(|v1, v2| sort_positions(v1, v2, eps));
    // sorted array of hex vertices without duplicates within eps
    let unique_city_positions = remove_vec3_duplicates(&all_tile_vertices, eps);
    print_vec_vec3(&unique_city_positions);

    let circle = Circle::new(0.2);

    for vertex in unique_city_positions.iter() {
        commands.spawn(PbrBundle {
            mesh: meshes.add(circle.into()),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_translation(Vec3 {
                x: vertex.x,
                y: vertex.y,
                z: 0.15,
            }),
            ..default()
        });
    }

    //? ROADS

    // All the hex vertices
    let mut all_tile_edge_centers: Vec<Vec3> = land_tile_arr
        .iter()
        .flat_map(|tile| tile.edges.iter().cloned())
        .collect();

    let unique_road_positions = remove_vec3_duplicates(&all_tile_edge_centers, eps);

    println!("{}, {}", unique_city_positions.len(), unique_road_positions.len());
}

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
