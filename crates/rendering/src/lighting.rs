use bevy::prelude::*;

/// Sets up a hemispheric ambient light + directional light for isometric 3D.
pub fn setup_lighting(mut commands: Commands) {
    // Directional light (simulates sun)
    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(
            Quat::from_euler(EulerRot::XYZ, -0.8, 0.5, 0.0),
        ),
    ));

    // Ambient light to soften shadows
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.2, 0.2, 0.3),
        brightness: 0.3,
    });
}
