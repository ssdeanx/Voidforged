use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use ir_core::{CursorWorldPos, ScreenShake};
use rand::Rng;

/// Sets up the isometric 3D camera.
/// Uses orthographic projection angled ~45° for the classic isometric look.
#[derive(Component)]
pub struct IsometricCamera;

#[derive(Bundle)]
pub struct IsometricCameraBundle {
    pub camera: Camera3d,
    pub projection: Projection,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub isometric: IsometricCamera,
}

pub fn spawn_isometric_camera(mut commands: Commands) {
    let projection = Projection::Orthographic(OrthographicProjection {
        near: 0.1,
        far: 1000.0,
        viewport_origin: Vec2::new(0.5, 0.5),
        scaling_mode: ScalingMode::FixedVertical { viewport_height: 20.0 },
        scale: 1.0,
        area: Rect::new(-1.0, -1.0, 1.0, 1.0),
    });

    commands.spawn(IsometricCameraBundle {
        camera: Camera3d::default(),
        projection,
        transform: Transform::from_xyz(0.0, 20.0, 20.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        global_transform: default(),
        visibility: Visibility::Visible,
        inherited_visibility: default(),
        view_visibility: default(),
        isometric: IsometricCamera,
    });
}

/// System that smoothly follows the player entity.
pub fn follow_player(
    player_query: Query<&Transform, With<ir_core::Player>>,
    mut camera_query: Query<&mut Transform, (With<IsometricCamera>, Without<ir_core::Player>)>,
    mut cam_transform: ResMut<ir_core::CameraTransform>,
    config: Res<ir_core::GameConfig>,
    time: Res<Time>,
) {
    let player_transform = match player_query.get_single() {
        Ok(t) => t,
        Err(_) => return,
    };

    for mut camera_transform in camera_query.iter_mut() {
        let target_pos = Vec3::new(
            player_transform.translation.x,
            camera_transform.translation.y,
            player_transform.translation.z + 2.0,
        );
        camera_transform.translation = camera_transform.translation.lerp(
            target_pos,
            (config.camera_follow_speed * time.delta_secs()).min(1.0),
        );
        camera_transform.look_at(player_transform.translation, Vec3::Y);
        // Share camera transform for camera-relative gameplay
        cam_transform.0 = camera_transform.translation;
        cam_transform.1 = camera_transform.rotation;
    }
}

/// Projects the mouse cursor position onto the ground plane (y=0) in world space.
pub fn cursor_to_world(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<IsometricCamera>>,
    mut cursor_pos: ResMut<CursorWorldPos>,
) {
    let window = match windows.get_single() {
        Ok(w) => w,
        Err(_) => return,
    };
    let cursor = match window.cursor_position() {
        Some(p) => p,
        None => return,
    };
    let (camera, camera_global) = match camera_query.get_single() {
        Ok(c) => c,
        Err(_) => return,
    };
    let ray = match camera.viewport_to_world(camera_global, cursor) {
        Ok(r) => r,
        Err(_) => return,
    };
    // Intersect ray with y=0 plane: t = -ray.origin.y / ray.direction.y
    if ray.direction.y.abs() < f32::EPSILON {
        return;
    }
    let t = -ray.origin.y / ray.direction.y;
    let world_pos = ray.origin + ray.direction * t;
    cursor_pos.0 = world_pos;
}

/// Applies screen shake offset to the camera based on trauma.
pub fn apply_screen_shake(
    time: Res<Time>,
    mut shake: ResMut<ScreenShake>,
    mut camera_query: Query<&mut Transform, With<IsometricCamera>>,
) {
    if shake.trauma <= 0.0 {
        return;
    }
    let mut rng = rand::thread_rng();
    let trauma_sq = shake.trauma * shake.trauma;
    let offset_x = (rng.gen::<f32>() - 0.5) * 2.0 * trauma_sq * 0.5;
    let offset_z = (rng.gen::<f32>() - 0.5) * 2.0 * trauma_sq * 0.5;
    for mut transform in camera_query.iter_mut() {
        transform.translation.x += offset_x;
        transform.translation.z += offset_z;
    }
    shake.trauma = (shake.trauma - shake.decay * time.delta_secs()).max(0.0);
}
