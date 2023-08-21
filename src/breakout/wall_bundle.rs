use bevy::{
    prelude::{default, Bundle, Color, Transform},
    sprite::{Sprite, SpriteBundle},
};

use super::{collider::Collider, wall_location::WallLocation};

const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

// This bundle is a collection of the components that define a "wall" in our game
#[derive(Bundle)]
pub struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    pub fn new(location: WallLocation) -> WallBundle {
        let transform = Transform {
            // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
            // This is used to determine the order of our sprites
            translation: location.position().extend(0.0),
            // The z-scale of 2D objects must always be 1.0,
            // or their ordering will be affected in surprising ways.
            // See https://github.com/bevyengine/bevy/issues/4149
            scale: location.size().extend(1.0),
            ..default()
        };

        return WallBundle {
            sprite_bundle: SpriteBundle {
                transform: transform,
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        };
    }
}
