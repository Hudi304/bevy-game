use std::f32::consts::PI;

use bevy::{input::mouse::MouseMotion, prelude::*};
/// Tags an entity as capable of panning and orbiting.
#[derive(Component)]
pub struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub upside_down: bool,
    pub last_motion_delta: Vec2,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            upside_down: false,
            last_motion_delta: Vec2::ZERO,
        }
    }
}

/// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
pub fn pan_orbit_camera(
    input_mouse: Res<Input<MouseButton>>,
    mut ev_motion: EventReader<MouseMotion>,
    mut camera_query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Right;

    let (mut orbit_cam, mut camera_transform) = camera_query.single_mut();

    if input_mouse.just_pressed(orbit_button) {
        let mouse_motion = ev_motion.iter().last();

        if let None = mouse_motion {
            return;
        }

        orbit_cam.last_motion_delta = mouse_motion.unwrap().delta;
    }

    // RIGHT CLICK
    if input_mouse.pressed(orbit_button) {
        for mouse_motion in ev_motion.iter() {
            // if orbit_cam.last_motion_delta == Vec2::ZERO {
            //     orbit_cam.last_motion_delta = mouse_motion.delta;
            // }

            let delta_pos = orbit_cam.last_motion_delta - mouse_motion.delta;

            let angle_const = PI / 180. / 10.;

            let quat_x = Quat::from_rotation_y(delta_pos.x * angle_const);
            let quat_y = Quat::from_rotation_x(delta_pos.y * angle_const);
            let quat_z = Quat::from_rotation_z(0.0);

            camera_transform.rotate(quat_x * quat_y * quat_z);
            orbit_cam.last_motion_delta = mouse_motion.delta;
        }
    }

    ev_motion.clear();
}

/// Spawn a camera like this
pub fn spawn_pan_camera(mut commands: Commands) {
    let translation = Vec3::new(0.0, 0.0, 15.0);

    let camera_transform = Transform::from_translation(translation)
        .looking_at(Vec3::ZERO, Vec3::Y)
        // .with_rotation(Quat::from_rotation_x(PI / 18.))  // looking up
        // .with_rotation(Quat::from_rotation_x(-PI / 18.)) // looking down
        // .with_rotation(Quat::from_rotation_y(-PI / 18.)) // looking right
        // .with_rotation(Quat::from_rotation_y(PI / 18.)) // looking left
        // .with_rotation(Quat::from_rotation_z(PI / 18.)) // rotated 10 deg right
        // .with_rotation(Quat::from_rotation_z(- PI / 18.)) // rotated 10 deg left 
        ;

    commands.spawn((
        Camera3dBundle {
            transform: camera_transform,
            ..Default::default()
        },
        PanOrbitCamera {
            ..Default::default()
        },
    ));
}
