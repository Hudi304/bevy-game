#[cfg(test)]
mod spawn_tiles_tests {
    use bevy::prelude::Vec3;

    use crate::{
        catan::world::{
            land_tile::TILE_RADIUS,
            vec3_utils::{ remove_vec3_duplicates, sort_positions },
            spawn_tiles::{ build_cub_coord_hex_gird, TILE_OFFSET_ANGLE_RAD },
        },
        hex::polygon::get_hex_vertices,
    };

    #[test]
    fn build_city_positions_1hex() {
        let eps = 0.01;
        let h = TILE_RADIUS * (3_f32).sqrt();

        let hex_cub_coord = build_cub_coord_hex_gird(0);
        assert_eq!(hex_cub_coord.len(), 1);

        let center_hex = hex_cub_coord.first().unwrap().to_owned();

        let center_cartesian_coord = center_hex.to_cartesian_vec3(h);
        let mut hex_vertices: Vec<Vec3> = get_hex_vertices(TILE_RADIUS, TILE_OFFSET_ANGLE_RAD)
            .iter()
            .map(|vert| vert.clone() + center_cartesian_coord)
            .collect();

        // sorted array of hex vertices, sorted by x,y
        hex_vertices.sort_by(|v1, v2| sort_positions(v1, v2, eps));

        // sorted array of hex vertices without duplicates within eps
        let unique_city_positions = remove_vec3_duplicates(&hex_vertices, eps);

        assert_eq!(unique_city_positions.len(), 6);
    }

    #[test]
    fn build_city_positions_7hex() {
        let eps = 0.01;
        let h = TILE_RADIUS * (3_f32).sqrt();

        let hex_cub_coord = build_cub_coord_hex_gird(1);

        assert_eq!(hex_cub_coord.len(), 7);

        let hex_vertex_cart = hex_cub_coord
            .iter()
            .map(|cub_coord| cub_coord.to_cartesian_vec3(h))
            .collect::<Vec<Vec3>>();

        let mut all_hex_vertices = vec![];

        for hex_vert in hex_vertex_cart {
            let hex_vertices: Vec<Vec3> = get_hex_vertices(TILE_RADIUS, TILE_OFFSET_ANGLE_RAD)
                .iter()
                .map(|vert| vert.clone() + hex_vert)
                .collect();

            all_hex_vertices.extend(hex_vertices);
        }

        // sorted array of hex vertices, sorted by x,y
        all_hex_vertices.sort_by(|v1, v2| sort_positions(v1, v2, eps));

        assert_eq!(all_hex_vertices.len(), 7 * 6);

        // sorted array of hex vertices without duplicates within eps
        let unique_city_positions = remove_vec3_duplicates(&all_hex_vertices, eps);

        // total number 6 (center) + 6 * 3  = 24
        // on the outer ring 2 vertices overlap the center
        // and one overlaps another outer hex
        assert_eq!(unique_city_positions.len(), 6 + 6 * 3);
    }

    #[test]
    fn build_city_positions_19hex() {
        let eps = 0.01;
        let h = TILE_RADIUS * (3_f32).sqrt();

        let hex_cub_coord = build_cub_coord_hex_gird(2);

        assert_eq!(hex_cub_coord.len(), 19);

        let hex_vertex_cart = hex_cub_coord
            .iter()
            .map(|cub_coord| cub_coord.to_cartesian_vec3(h))
            .collect::<Vec<Vec3>>();

        let mut all_hex_vertices = vec![];

        for hex_vert in hex_vertex_cart {
            let hex_vertices: Vec<Vec3> = get_hex_vertices(TILE_RADIUS, TILE_OFFSET_ANGLE_RAD)
                .iter()
                .map(|vert| vert.clone() + hex_vert)
                .collect();

            all_hex_vertices.extend(hex_vertices);
        }

        // sorted array of hex vertices, sorted by x,y
        all_hex_vertices.sort_by(|v1, v2| sort_positions(v1, v2, eps));

        assert_eq!(all_hex_vertices.len(), 19 * 6);

        // sorted array of hex vertices without duplicates within eps
        let unique_city_positions = remove_vec3_duplicates(&all_hex_vertices, eps);

        // on the first ring, all the hexes are special cases.
        // they are all on the vertex of the virtual (big) hex
        // on the second ring, there are 12 hexes
        // 6 of them are on the vertexes and 6 are in the middle of an edge
        // the vertex ones can contribute with 3 cities each, so the previous formula still stands
        // the hexes on the edges can only contribute with 2 new cities.

        // r(0) = 6
        // r(1) = r(0) + 6 * 3 = 24
        // r(2) = r(1) + 6 * 3 + (6*2 - 6) * 2
        // r(2) = 24  + 18 + 12 = 54
        // r(2) = r(1) + 6*(3 + 2(1)) = 24 + 6 * 5 = 54

        // maybe prove this with mathematical induction

        // r(x) = r(x-1) + 6*3 + 6*2*(r-1)
        // r(x) = r(x-1) + 6*(3 + 2*(r-1))

        assert_eq!(unique_city_positions.len(), 6 + 6 * 3 + 6 * 3 + 6 * 2);
    }
}
