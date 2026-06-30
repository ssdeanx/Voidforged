use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

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
    config: Res<ir_core::GameConfig>,
    time: Res<Time>,
) {
    let player_transform = match player_query.get_single() {
        Ok(t) => t,
        Err(_) => return,
    };

    for mut camera_transform in camera_query.iter_mut() {
        let target = Vec3::new(
            player_transform.translation.x,
            camera_transform.translation.y,
            player_transform.translation.z,
        );
        camera_transform.translation = camera_transform.translation.lerp(
            target,
            config.camera_follow_speed * time.delta_secs(),
        );
    }
}
