use std::{
    cmp::Ordering,
    f32::{consts::PI, EPSILON},
    ops::Range,
};

use bevy::prelude::{
    shape::{Circle, Quad},
    *,
};

use bevy_mod_picking::{
    prelude::{On, Out, Over, Pointer, RaycastPickTarget},
    PickableBundle,
};

use crate::catan::cubic_coords::cube_coordinates::CubCoord;

use super::{
    land_tile::{build_tile_mesh, LandTile, NUMBER_OF_TILES},
    randomize_richness::build_random_tile_richness_array,
    randomize_tiles::build_random_tile_type_array,
    tile_type::TileType,
    water_tile::WaterTile,
};

pub fn spawn_land_tiles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let h = 3_f32.sqrt() / 2. * 1.01;

    let offset_angle = PI / 6.;
    let cub_coords_arr: Vec<CubCoord> = build_cub_coord_hex_gird(7);

    let mut i = 0;
    let tile_type_arr = build_random_tile_type_array(NUMBER_OF_TILES);
    let richness_arr = build_random_tile_richness_array(NUMBER_OF_TILES);

    let mut tile_arr = Vec::with_capacity(NUMBER_OF_TILES);

    for cub_coord in cub_coords_arr {
        // TODO refactor this so the HexWorldTile contains the material asset and the mesh asset
        if cub_coord.ring < 3 {
            let tile_type = tile_type_arr[i];
            let tile_richness = richness_arr[i];

            let material = materials.add(tile_type.into_color().into());

            let mesh: Handle<Mesh> = meshes.add(build_tile_mesh(offset_angle));

            let (prb_bundle, component) = LandTile::build(
                cub_coord,
                material,
                mesh,
                tile_type,
                tile_richness,
                offset_angle,
            );

            tile_arr.push(component.clone());

            // let circ = Circle::new(0.2);
            // for vertex in component.vertices.iter() {
            //     commands.spawn(PbrBundle {
            //         mesh: meshes.add(circ.into()),
            //         material: materials.add(Color::WHITE.into()),
            //         transform: Transform::from_translation(Vec3 {
            //             x: vertex.x,
            //             y: vertex.y,
            //             z: 0.15,
            //         }),
            //         ..default()
            //     });
            // }

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
                component.clone(),
                PickableBundle::default(),
                On::<Pointer<Over>>::target_component_mut::<Transform>(|_, transform| {
                    let mut old_translation = transform.translation;
                    old_translation.z = 0.2;
                    transform.translation = old_translation
                }),
                On::<Pointer<Out>>::target_component_mut::<Transform>(|_, transform| {
                    let mut old_translation = transform.translation;
                    old_translation.z = 0.0;
                    transform.translation = old_translation
                }),
                RaycastPickTarget::default(),
            ));

            i += 1;
        } else {
            let material = materials.add(Color::BLUE.into());

            let mesh: Handle<Mesh> = meshes.add(build_tile_mesh(offset_angle));

            let ent = WaterTile::build(cub_coord, material, mesh, TileType::WATER, 0);

            commands.spawn(ent);
        }
    }

    let mut city_points: Vec<Vec3> = Vec::new();

    for tile in tile_arr {
        for vert in tile.vertices {
            city_points.push(vert);
        }
    }

    let eps = 0.01;

    let sort_fn = |v1: &Vec3, v2: &Vec3| {
        if v1.x - v2.x < eps {
            return Ordering::Less;
        }

        if v2.x - v1.x < eps {
            return Ordering::Greater;
        }

        if v1.y - v2.y < eps {
            return Ordering::Less;
        }

        if v2.y - v1.y < eps {
            return Ordering::Greater;
        }

        println!("Equal x");
        println!("  {}", v1);
        println!("  {}", v2);

        return Ordering::Equal;
    };
    // this sort by does not work as intended
    city_points.sort_by(sort_fn);
    city_points.sort_by(sort_fn);

    for pt in city_points.iter() {
        println!("{:.2},{:.2}", pt.x, pt.y);
    }

    let mut city_points_iter = city_points.iter();

    let prev_el = city_points_iter.next().unwrap();

    let mut single_arr = Vec::new();

    for pt in city_points_iter {
        if prev_el.x - pt.x < eps && prev_el.y - pt.y < eps {
            single_arr.push(pt);
        }
    }

    println!("{}", city_points.len());
    println!("{}", single_arr.len());

    // print_pos_vec(single_arr);
}

fn print_pos_vec(arr: Vec<&Vec3>) {
    for pt in arr.iter() {
        println!("{:.2},{:.2}", pt.x, pt.y);
    }
}

pub fn build_cub_coord_hex_gird(radius: i32) -> Vec<CubCoord> {
    let mut hex_arr = vec![];
    let slice: Range<i32> = -radius..radius + 1;

    for q in slice.clone() {
        for r in slice.clone() {
            let s: i32 = 0 - q - r;

            if s.abs() > radius {
                continue;
            }

            hex_arr.push(CubCoord::from_tuple((q, r, s)));
        }
    }

    return hex_arr;
}
