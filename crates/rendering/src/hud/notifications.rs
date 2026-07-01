//! Notifications: damage numbers, wave announcements, enemy health bar cleanup.
//!
//! Damage numbers now use proper animated floating text with pop-up motion:
//! - Float upward + fade out
//! - Crit shown larger + bold with golden color
//! - Physical: white, Magic: purple, True: orange
//! Enemy health bars are handled by nameplates.rs.

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

/// Spawns floating damage numbers with pop-up + fade animation.
/// - Physical: white
/// - Magic: purple
/// - True: orange
/// - Crits: large + bold golden text with "!" suffix, more upward velocity
/// Respects GameConfig.damage_numbers toggle.
pub fn spawn_damage_numbers(
    mut commands: Commands,
    mut events: EventReader<DamageNumberEvent>,
    config: Res<GameConfig>,
) {
    if !config.damage_numbers {
        // Drain events without spawning
        for _ in events.read() {}
        return;
    }
    for event in events.read() {
        let is_crit = event.is_crit;
        let color = if is_crit {
            Color::srgb(1.0, 0.8, 0.0) // golden crit
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
        let font_size = if is_crit { 32.0 } else { 20.0 };
        let vel_y = if is_crit { 2.2 } else { 1.4 };
        let lifetime = if is_crit { 1.5 } else { 1.2 };

        // Spawn as animated floating text in 3D world space
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
                lifetime,
                is_crit,
            },
        ));
    }
}

/// Updates damage number positions, fades them out, and despawns expired ones.
/// Crit numbers float higher and fade slower.
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
        // Float upward with slight gravity
        transform.translation += anim.velocity * dt;
        anim.velocity.y -= 0.3 * dt; // gentle gravity

        // Fade out over last 40% of lifetime
        let total_lifetime = if anim.is_crit { 1.5 } else { 1.2 };
        let fade_start = total_lifetime * 0.6;
        let alpha = if anim.lifetime > fade_start {
            1.0
        } else {
            (anim.lifetime / fade_start).max(0.0)
        };

        // Preserve original RGB, only modify alpha
        let rgba = color.0.to_srgba();
        color.0 = Color::srgba(rgba.red, rgba.green, rgba.blue, alpha);
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

// ── Level-Up Popup ──────────────────────────────────────────────────────────

/// Spawns a golden \"LEVEL UP!\" announcement when the player levels up.
/// Scales up from small to large over 0.5s, holds for 1.0s, then fades.
/// Shows new max HP and damage bonus info.
pub fn spawn_level_up_popup(
    mut commands: Commands,
    mut level_up_events: EventReader<LevelUpEvent>,
    player_query: Query<(&Health, &CombatStats), With<Player>>,
) {
    for event in level_up_events.read() {
        let (hp_text, dmg_text) = if let Ok((health, stats)) = player_query.get_single() {
            (
                format!("New max HP: {:.0}", health.max),
                format!("Damage +{:.0}", stats.damage_bonus),
            )
        } else {
            ("New max HP: +10".to_string(), "Damage +2".to_string())
        };

        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(8.0),
                    ..default()
                },
                HudLevelUpPopup { timer: 3.0 },
                ir_core::tween::Tween::scale(0.5, 1.0, 0.5, ir_core::tween::easing::bounce),
            ))
            .with_children(|root| {
                // LEVEL UP! heading
                root.spawn((
                    Text::new(format!("LEVEL UP! (Lv. {})", event.new_level)),
                    TextFont {
                        font_size: 48.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 0.85, 0.0)), // golden
                    TextLayout::new_with_no_wrap(),
                    HudLevelUpText,
                ));
                // HP bonus text
                root.spawn((
                    Text::new(hp_text),
                    TextFont {
                        font_size: 22.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.3, 0.9, 0.4)),
                    HudLevelUpText,
                ));
                // Damage bonus text
                root.spawn((
                    Text::new(dmg_text),
                    TextFont {
                        font_size: 22.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 0.5, 0.2)),
                    HudLevelUpText,
                ));
            });
    }
}

/// Updates the level-up popup animation timer and despawns expired popups.
pub fn update_level_up_popups(
    mut commands: Commands,
    time: Res<Time>,
    mut popups: Query<(Entity, &mut HudLevelUpPopup)>,
) {
    let dt = time.delta_secs();
    for (entity, mut popup) in popups.iter_mut() {
        popup.timer -= dt;
        if popup.timer <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
