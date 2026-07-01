//! Buff/Debuff indicators — a row of small square icons above the action bar
//! showing active buffs and debuffs on the player with remaining duration text.
//!
//! Buffs with durations <25% remaining become slightly transparent.
//! Debuffs (Frozen, Stun, Poison) are shown with a red border.

use bevy::prelude::*;
use ir_core::*;
use crate::hud::components::*;

const ICON_SIZE: f32 = 28.0;
const ICON_GAP: f32 = 4.0;

/// Helper for text labels.
fn label(s: &str, size: f32, color: Color) -> impl Bundle {
    (
        Text::new(s.to_string()),
        TextFont { font_size: size, ..default() },
        TextColor(color),
    )
}

/// Color for each buff type.
fn buff_color(kind: &AbilityKind) -> Color {
    match kind {
        AbilityKind::SpeedBoost => Color::srgb(0.2, 0.8, 0.4),
        AbilityKind::DamageAura => Color::srgb(1.0, 0.4, 0.2),
        AbilityKind::Shield => Color::srgb(0.3, 0.6, 1.0),
        AbilityKind::Thorns => Color::srgb(0.8, 0.2, 0.6),
        AbilityKind::MultiShot => Color::srgb(1.0, 0.8, 0.1),
        AbilityKind::PierceShot => Color::srgb(0.5, 0.8, 0.9),
    }
}

/// Debuff kinds (status effects that are negative).
/// We detect debuffs by looking for Frozen or Stun components on the player.
fn detect_debuffs(
    player_entity: Entity,
    frozen_query: &Query<&Frozen>,
    stun_query: &Query<&Stun>,
) -> Vec<(String, Color)> {
    let mut debuffs = Vec::new();
    if frozen_query.get(player_entity).is_ok() {
        debuffs.push(("Frozen".into(), Color::srgb(0.3, 0.6, 1.0)));
    }
    if stun_query.get(player_entity).is_ok() {
        debuffs.push(("Stun".into(), Color::srgb(1.0, 1.0, 0.0)));
    }
    debuffs
}

/// Returns display name for an AbilityKind.
fn ability_display_name(kind: &AbilityKind) -> &'static str {
    match kind {
        AbilityKind::SpeedBoost => "Speed",
        AbilityKind::DamageAura => "Dmg Aura",
        AbilityKind::Shield => "Shield",
        AbilityKind::Thorns => "Thorns",
        AbilityKind::MultiShot => "Multi",
        AbilityKind::PierceShot => "Pierce",
    }
}

/// Spawns the buff bar container — an empty row anchored near the ability bar.
pub fn spawn_buff_bar(parent: &mut ChildBuilder) {
    parent.spawn((
        Node {
            width: Val::Px(400.0),
            height: Val::Px(ICON_SIZE + 4.0),
            position_type: PositionType::Absolute,
            bottom: Val::Px(76.0 + 20.0), // above XP bar
            left: Val::Percent(50.0),
            margin: UiRect::left(Val::Px(-200.0)),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            column_gap: Val::Px(ICON_GAP),
            ..default()
        },
        HudBuffBar,
    ));
}

/// Updates the buff bar — rebuilds icons each frame from Ability components on
/// the player, plus debuffs detected by status components.
pub fn update_buff_bar(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    ability_query: Query<&Ability, With<Player>>,
    buff_bar_query: Query<Entity, With<HudBuffBar>>,
    buff_icons: Query<Entity, With<HudBuffIcon>>,
    frozen_query: Query<&Frozen>,
    stun_query: Query<&Stun>,
) {
    let Ok(player_entity) = player_query.get_single() else { return };
    let Ok(buff_bar) = buff_bar_query.get_single() else { return };

    // Despawn old buff icons
    for entity in buff_icons.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Collect buffs from Ability components on the player
    let mut buffs: Vec<(AbilityKind, f32, f32)> = Vec::new(); // (kind, remaining, max)
    for ability in ability_query.iter() {
        if let Some(dur) = ability.duration {
            buffs.push((ability.kind.clone(), dur, dur));
        } else {
            buffs.push((ability.kind.clone(), 999.0, 999.0)); // permanent
        }
    }

    // Detect debuffs
    let debuffs = detect_debuffs(player_entity, &frozen_query, &stun_query);

    // Spawn buff icons
    commands.entity(buff_bar).with_children(|bar| {
        for (kind, remaining, max) in &buffs {
            let color = buff_color(kind);
            let is_low = *max > 0.0 && *remaining / *max < 0.25;
            let alpha = if is_low { 0.5 } else { 1.0 };
            let name = ability_display_name(kind);
            bar.spawn((
                Node {
                    width: Val::Px(ICON_SIZE),
                    height: Val::Px(ICON_SIZE),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BorderColor(Color::srgba(color.to_srgba().red, color.to_srgba().green, color.to_srgba().blue, alpha)),
                BackgroundColor(Color::srgba(
                    color.to_srgba().red * 0.3,
                    color.to_srgba().green * 0.3,
                    color.to_srgba().blue * 0.3,
                    alpha,
                )),
                HudBuffIcon {
                    kind: kind.clone(),
                    remaining: *remaining,
                    max_duration: *max,
                    is_debuff: false,
                },
            )).with_children(|icon| {
                // First letter of buff name
                icon.spawn((
                    label(&name[..1.min(name.len())], 12.0, Color::srgba(1.0, 1.0, 1.0, alpha)),
                    HudBuffIcon { kind: kind.clone(), remaining: *remaining, max_duration: *max, is_debuff: false },
                ));
                // Duration text if finite
                if *max < 500.0 {
                    icon.spawn((
                        label(
                            &format!("{:.0}", remaining.max(0.0)),
                            8.0,
                            Color::srgba(1.0, 1.0, 1.0, alpha * 0.8),
                        ),
                        HudBuffIcon { kind: kind.clone(), remaining: *remaining, max_duration: *max, is_debuff: false },
                    ));
                }
            });
        }

        // Spawn debuff icons
        for (name, color) in &debuffs {
            bar.spawn((
                Node {
                    width: Val::Px(ICON_SIZE),
                    height: Val::Px(ICON_SIZE),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BorderColor(Color::srgb(1.0, 0.15, 0.15)), // red border for debuffs
                BackgroundColor(Color::srgba(
                    color.to_srgba().red * 0.2,
                    color.to_srgba().green * 0.2,
                    color.to_srgba().blue * 0.2,
                    1.0,
                )),
                HudBuffIcon {
                    kind: AbilityKind::SpeedBoost, // placeholder
                    remaining: 0.0,
                    max_duration: 0.0,
                    is_debuff: true,
                },
            )).with_children(|icon| {
                icon.spawn((
                    label(&name[..1.min(name.len())], 12.0, Color::srgb(1.0, 0.5, 0.5)),
                    HudBuffIcon {
                        kind: AbilityKind::SpeedBoost,
                        remaining: 0.0,
                        max_duration: 0.0,
                        is_debuff: true,
                    },
                ));
            });
        }
    });
}

/// Ticks buff timers on Ability components (reduces duration each frame).
pub fn tick_buff_timers(
    time: Res<Time>,
    mut ability_query: Query<&mut Ability, With<Player>>,
) {
    let dt = time.delta_secs();
    for mut ability in ability_query.iter_mut() {
        if let Some(ref mut dur) = ability.duration {
            *dur -= dt;
            if *dur <= 0.0 {
                *dur = 0.0;
                // Note: the ability component removal should happen elsewhere
                // when duration expires — this just ticks the timer down
            }
        }
    }
}
