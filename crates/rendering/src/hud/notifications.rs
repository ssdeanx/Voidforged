//! Notifications: damage numbers, wave announcements, enemy health bars.

use bevy::prelude::*;
use ir_core::*;
use crate::hud::components::*;

// ── Damage Numbers ──────────────────────────────────────────────────────────

pub fn spawn_damage_numbers(
    mut commands: Commands,
    mut events: EventReader<DamageNumberEvent>,
) {
    for event in events.read() {
        let color = if event.is_crit {
            Color::srgb(1.0, 0.8, 0.0)
        } else {
            Color::srgb(1.0, 0.6, 0.6)
        };
        let text = if event.is_crit {
            format!("{}!", event.amount)
        } else {
            format!("{}", event.amount)
        };
        commands.spawn((
            Text2d::new(text),
            TextFont { font_size: if event.is_crit { 26.0 } else { 18.0 }, ..default() },
            TextColor(color),
            Transform::from_translation(event.position),
            Lifetime { remaining: 0.9 },
        ));
    }
}

// ── Wave Announcements ──────────────────────────────────────────────────────

pub fn spawn_wave_announcements(
    mut commands: Commands,
    mut wave_start_events: EventReader<WaveStartEvent>,
) {
    for event in wave_start_events.read() {
        commands.spawn((
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
        )).with_children(|root| {
            root.spawn((
                label(&format!("WAVE {}", event.wave_number), 48.0, Color::srgb(1.0, 0.7, 0.1)),
                WaveAnnouncement(2.5),
            ));
            root.spawn((
                label(&format!("{} enemies", event.enemy_count), 22.0, Color::srgb(0.8, 0.6, 0.3)),
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
        commands.spawn((
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
        )).with_children(|root| {
            root.spawn((
                label(&format!("Wave {} CLEARED!", event.wave_number), 40.0, Color::srgb(0.2, 1.0, 0.3)),
                WaveAnnouncement(2.0),
            ));
        });
    }
}

fn label(s: &str, size: f32, color: Color) -> impl Bundle {
    (
        Text::new(s.to_string()),
        TextFont { font_size: size, ..default() },
        TextColor(color),
    )
}

// ── Enemy Health Bars ───────────────────────────────────────────────────────

use std::collections::HashSet;

#[derive(Component)]
pub struct EnemyHealthBar(pub Entity);

pub fn update_enemy_health_bars(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Health, &Transform), (With<Enemy>, Without<Player>)>,
    existing_bars: Query<(Entity, &EnemyHealthBar)>,
) {
    let mut to_remove: HashSet<Entity> = existing_bars.iter().map(|(e, _)| e).collect();

    for (enemy_entity, health, transform) in enemy_query.iter() {
        if let Some((bar_entity, _)) =
            existing_bars.iter().find(|(_, p)| p.0 == enemy_entity)
        {
            to_remove.remove(&bar_entity);
            continue;
        }
        let pct = health.fraction();
        let bar_len = (pct * 10.0) as usize;
        let color = if pct > 0.5 {
            Color::srgb(0.0, 0.8, 0.1)
        } else if pct > 0.25 {
            Color::srgb(0.8, 0.8, 0.0)
        } else {
            Color::srgb(0.9, 0.2, 0.1)
        };
        let bar_text = "█".repeat(bar_len.max(1).min(10));
        commands.spawn((
            Text2d::new(bar_text),
            TextFont { font_size: 10.0, ..default() },
            TextColor(color),
            Transform::from_translation(transform.translation + Vec3::Y * 1.5),
            Lifetime { remaining: 0.1 },
            EnemyHealthBar(enemy_entity),
        ));
    }

    for entity in to_remove {
        commands.entity(entity).despawn();
    }
}
