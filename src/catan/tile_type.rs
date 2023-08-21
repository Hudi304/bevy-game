use bevy::prelude::Color;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TileType {
    /// 4
    WHEAT,
    /// 3
    STONE,
    /// 4
    SHEEP,
    /// 3
    CLAY,
    /// 4
    WOOD,
    /// 1
    DESERT,
    /// ANY
    WATER,
}

impl TileType {
    pub fn into_color(&self) -> Color {
        match self {
            TileType::WHEAT => Color::YELLOW,
            TileType::CLAY => Color::ORANGE,
            TileType::STONE => Color::DARK_GRAY,
            TileType::SHEEP => Color::LIME_GREEN,
            TileType::WOOD => Color::DARK_GREEN,
            TileType::DESERT => Color::YELLOW_GREEN,
            TileType::WATER => Color::BLUE,
        }
    }
}
