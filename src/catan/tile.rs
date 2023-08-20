use bevy::prelude::*;

use super::map_tile::HexWorldTile;

pub fn spawn_second_tile_row(
    center: Vec3,
    material: &Handle<StandardMaterial>,
    mesh: &Handle<Mesh>,
) -> (PbrBundle, HexWorldTile) {
    return HexWorldTile::build(center, material.clone(), mesh.clone());
}
