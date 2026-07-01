//! Notifications: damage numbers, wave announcements, enemy health bar cleanup.
//!
//! Damage numbers now use proper animated floating text with pop-up motion.
//! Enemy health bars are now handled by nameplates.rs instead of text bars.

use bevy::prelude::*;
use ir_core::*;
use crate::hud::components::*;

fn label(s: &str, size: f32, color: Color) -> impl Bundle {
    (
        Text::new(s.to_string()),
        TextFont {
            font_size: size,
            ..default()
        },
        TextColor(color),
    )
}

// ── Damage Numbers ──────────────────────────────────────────────────────────

/// Spawns floating damage numbers with animation (pop-up + fade).
/// Uses damage type for color differentiation:
/// - Physical: white
/// - Magic: purple
/// - True: orange
pub fn spawn_damage_numbers(
    mut commands: Commands,
    mut events: EventReader<DamageNumberEvent>,
) {
    for event in events.read() {
        let is_crit = event.is_crit;
        let color = if is_crit {
            Color::srgb(1.0, 0.8, 0.0)
        } else {
            match event.damage_type {
                DamageType::Physical => Color::srgb(1.0, 1.0, 1.0),
                DamageType::Magic => Color::srgb(0.8, 0.4, 1.0),
                DamageType::True => Color::srgb(1.0, 0.6, 0.0),
            }
        };
        let text = if is_crit {
            format!("{}!", event.amount)
        } else {
            format!("{}", event.amount)
        };
        let font_size = if is_crit { 28.0 } else { 18.0 };

        // Crit text gets a slightly larger pop
        let vel_y = if is_crit { 1.8 } else { 1.2 };

        // Spawn as an animated floating text in 3D world space
        commands.spawn((
            Text2d::new(text),
            TextFont {
                font_size,
                ..default()
            },
            TextColor(color),
            Transform::from_translation(event.position + Vec3::Y * 0.5),
            DamageNumberAnim {
                velocity: Vec3::new(
                    (rand::random::<f32>() - 0.5) * 0.8,
                    vel_y,
                    (rand::random::<f32>() - 0.5) * 0.8,
                ),
                lifetime: 1.2,
            },
        ));
    }
}

/// Animation state for a floating damage number.
#[derive(Component)]
pub struct DamageNumberAnim {
    pub velocity: Vec3,
    pub lifetime: f32,
}

/// Updates damage number positions and despawns expired ones.
pub fn update_damage_numbers(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DamageNumberAnim, &mut Transform, &mut TextColor)>,
) {
    let dt = time.delta_secs();
    for (entity, mut anim, mut transform, mut color) in query.iter_mut() {
        anim.lifetime -= dt;
        if anim.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }
        // Float upward
        transform.translation += anim.velocity * dt;
        anim.velocity.y -= 0.5 * dt; // gravity
        // Fade out
        let alpha = (anim.lifetime / 1.2).max(0.0);
        color.0 = Color::srgba(
            color.0.to_srgba().red,
            color.0.to_srgba().green,
            color.0.to_srgba().blue,
            alpha,
        );
    }
}

// ── Wave Announcements ──────────────────────────────────────────────────────

pub fn spawn_wave_announcements(
    mut commands: Commands,
    mut wave_start_events: EventReader<WaveStartEvent>,
) {
    for event in wave_start_events.read() {
        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                WaveAnnouncement(2.5),
            ))
            .with_children(|root| {
                root.spawn((
                    label(
                        &format!("WAVE {}", event.wave_number),
                        48.0,
                        Color::srgb(1.0, 0.7, 0.1),
                    ),
                    WaveAnnouncement(2.5),
                ));
                root.spawn((
                    label(
                        &format!("{} enemies", event.enemy_count),
                        22.0,
                        Color::srgb(0.8, 0.6, 0.3),
                    ),
                    WaveAnnouncement(2.5),
                ));
            });
    }
}

pub fn update_wave_announcements(
    mut commands: Commands,
    time: Res<Time>,
    mut announcements: Query<(Entity, &mut WaveAnnouncement)>,
) {
    for (entity, mut ann) in announcements.iter_mut() {
        ann.0 -= time.delta_secs();
        if ann.0 <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn spawn_wave_cleared(
    mut commands: Commands,
    mut wave_cleared_events: EventReader<WaveClearedEvent>,
) {
    for event in wave_cleared_events.read() {
        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                WaveAnnouncement(2.0),
            ))
            .with_children(|root| {
                root.spawn((
                    label(
                        &format!("Wave {} CLEARED!", event.wave_number),
                        40.0,
                        Color::srgb(0.2, 1.0, 0.3),
                    ),
                    WaveAnnouncement(2.0),
                ));
            });
    }
}

// ── Enemy Health Bars (delegated to nameplates.rs) ──────────────────────────
// Enemy health bar text spawning has been removed.
// 3D world-space nameplates in nameplates.rs handle this now.
