//! Enemy nameplates – 3D world-space UI over each enemy.
//!
//! Shows enemy name, health bar (colored by HP %), and HP % text.
//! Only shown for enemies within a certain range. Uses billboard-style
//! Text2d entities positioned above enemies.

use bevy::prelude::*;
use ir_core::*;
use crate::hud::components::EnemyNameplate;

const NAMEPLATE_RANGE: f32 = 25.0;

// ── Health colour helpers ───────────────────────────────────────────────────

fn health_bar_color(pct: f32) -> Color {
    if pct > 0.6 {
        Color::srgb(0.0, 0.75, 0.15)
    } else if pct > 0.3 {
        Color::srgb(0.85, 0.75, 0.05)
    } else {
        Color::srgb(0.9, 0.15, 0.05)
    }
}

fn bar_text_color(pct: f32) -> Color {
    if pct > 0.6 {
        Color::srgb(0.6, 1.0, 0.6)
    } else if pct > 0.3 {
        Color::srgb(1.0, 0.9, 0.3)
    } else {
        Color::srgb(1.0, 0.4, 0.3)
    }
}

/// Spawns fresh nameplates for each enemy and despawns stale ones every frame.
/// Only shows nameplates for enemies within NAMEPLATE_RANGE of the player.
pub fn update_enemy_nameplates(
    mut commands: Commands,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    enemy_query: Query<(Entity, &Health, &Transform, Option<&Enemy>), Without<Player>>,
    existing_query: Query<(Entity, &EnemyNameplate)>,
) {
    use std::collections::HashSet;

    let Ok(player_tf) = player_query.get_single() else { return };
    let mut to_remove: HashSet<Entity> = existing_query.iter().map(|(e, _)| e).collect();

    for (enemy_entity, health, transform, enemy_opt) in enemy_query.iter() {
        let dist = player_tf.translation.distance(transform.translation);

        // Skip enemies outside range
        if dist > NAMEPLATE_RANGE {
            continue;
        }

        // Update existing nameplate position
        if let Some((np_entity, _)) = existing_query.iter().find(|(_, np)| np.0 == enemy_entity) {
            to_remove.remove(&np_entity);
            if let Some(mut ec) = commands.get_entity(np_entity) {
                let pos = transform.translation + Vec3::Y * 1.8;
                ec.insert(Transform::from_translation(pos));
            }
            continue;
        }

        // Spawn a new nameplate with proper bar + text
        let pct = health.fraction();
        let name = enemy_opt
            .map(|e| format!("{:?}", e.variant))
            .unwrap_or_else(|| "Enemy".to_string());
        let pct_str = format!("{}%", (pct * 100.0).round() as u32);
        let pos = transform.translation + Vec3::Y * 1.8;

        commands
            .spawn((
                Transform::from_translation(pos),
                GlobalTransform::default(),
                EnemyNameplate(enemy_entity),
            ))
            .with_children(|parent| {
                // Enemy name
                parent.spawn((
                    Text2d::new(name),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 0.85, 0.6)),
                    Transform::from_translation(Vec3::Y * 0.5),
                ));

                // Health bar background
                parent.spawn((
                    Text2d::new("██████████"),
                    TextFont {
                        font_size: 9.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.15, 0.15, 0.15)),
                    Transform::from_translation(Vec3::ZERO),
                ));

                // Health bar fill
                let fill_chars = (pct * 10.0).round() as usize;
                let fill = "█".repeat(fill_chars.max(1).min(10));
                parent.spawn((
                    Text2d::new(fill),
                    TextFont {
                        font_size: 9.0,
                        ..default()
                    },
                    TextColor(health_bar_color(pct)),
                    Transform::from_translation(Vec3::ZERO),
                ));

                // HP percentage
                parent.spawn((
                    Text2d::new(pct_str),
                    TextFont {
                        font_size: 10.0,
                        ..default()
                    },
                    TextColor(bar_text_color(pct)),
                    Transform::from_translation(-Vec3::Y * 0.5),
                ));
            });
    }

    // Remove stale nameplates (enemy dead, gone, or out of range)
    for entity in to_remove {
        if let Some(ec) = commands.get_entity(entity) {
            ec.despawn_recursive();
        }
    }
}
