//! In-game HUD layout: health bar, xp bar, level, gold, dash, zone, prompt, hotbar.

use bevy::prelude::*;
use crate::hud::components::*;

fn label(s: &str, size: f32, color: Color) -> impl Bundle {
    (
        Text::new(s.to_string()),
        TextFont { font_size: size, ..default() },
        TextColor(color),
    )
}

/// Spawns the full in-game HUD (top bar, xp bar, zone, prompt, hotbar).
pub fn spawn_hud(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            HudRoot,
        ))
        .with_children(|parent| {
            // ── Top bar: health, level, wave/gold/dash ──────────────
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(44.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    padding: UiRect::new(Val::Px(12.0), Val::Px(12.0), Val::Px(8.0), Val::Px(0.0)),
                    ..default()
                },
                HudRoot,
            )).with_children(|top| {
                // Health section
                top.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(8.0),
                        ..default()
                    },
                    HudRoot,
                )).with_children(|health_sec| {
                    health_sec.spawn((
                        Node {
                            width: Val::Px(220.0),
                            height: Val::Px(22.0),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        BorderColor(Color::srgb(0.3, 0.3, 0.3)),
                        BackgroundColor(Color::srgb(0.2, 0.05, 0.05)),
                        HudRoot,
                    )).with_children(|hp_bg| {
                        hp_bg.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.0, 0.7, 0.15)),
                            HudHealthBar,
                            HudRoot,
                        ));
                    });
                    health_sec.spawn((
                        label("100/100", 15.0, Color::WHITE),
                        HudHpText,
                        HudRoot,
                    ));
                });

                // Center: Level
                top.spawn((
                    label("Lv. 1", 22.0, Color::srgb(0.7, 0.5, 1.0)),
                    HudLevelText,
                    HudRoot,
                ));

                // Right: Wave + Gold + Dash
                top.spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::End,
                        row_gap: Val::Px(2.0),
                        ..default()
                    },
                    HudRoot,
                )).with_children(|right| {
                    right.spawn((label("Wave 1", 20.0, Color::srgb(1.0, 0.7, 0.2)), HudWaveText, HudRoot));
                    right.spawn((label("Gold: 0", 14.0, Color::srgb(1.0, 0.85, 0.0)), HudGoldText, HudRoot));
                    right.spawn((label("Dash: ready", 13.0, Color::srgb(0.5, 0.8, 1.0)), HudDashText, HudRoot));
                });
            });

            // ── Bottom XP bar ──────────────────────────────────────
            parent.spawn((
                Node {
                    width: Val::Percent(70.0),
                    height: Val::Px(14.0),
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(56.0),
                    left: Val::Percent(15.0),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BorderColor(Color::srgb(0.3, 0.3, 0.4)),
                BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
                HudRoot,
            )).with_children(|xp_bg| {
                xp_bg.spawn((
                    Node {
                        width: Val::Percent(0.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.5, 1.0)),
                    HudXpBar,
                    HudRoot,
                ));
            });

            // ── Zone name ───────────────────────────────────────────
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(40.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                HudRoot,
            )).with_children(|zone| {
                zone.spawn((label("", 18.0, Color::srgb(0.6, 0.8, 0.4)), HudZoneText, HudRoot));
            });

            // ── Interaction prompt ──────────────────────────────────
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    bottom: Val::Percent(50.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                HudRoot,
            )).with_children(|prompt| {
                prompt.spawn((label("", 20.0, Color::srgb(1.0, 0.9, 0.4)), HudPromptText, HudRoot));
            });

            // ── Ability Hotbar ──────────────────────────────────────
            spawn_hotbar(parent);
        });
}

pub fn despawn_hud(mut commands: Commands, hud: Query<Entity, With<HudRoot>>) {
    for entity in hud.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Spawns the ability hotbar at the bottom center.
pub fn spawn_hotbar(parent: &mut ChildBuilder) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(44.0),
            position_type: PositionType::Absolute,
            bottom: Val::Px(4.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: Val::Px(6.0),
            ..default()
        },
        HudRoot,
    )).with_children(|bar| {
        for (key, label_text, color) in [
            ("LMB", "Attack", Color::srgb(0.4, 0.7, 1.0)),
            ("RMB", "Spread", Color::srgb(0.9, 0.6, 0.2)),
            ("Q", "Cast", Color::srgb(0.8, 0.3, 1.0)),
            ("Shift", "Dash", Color::srgb(0.3, 0.8, 1.0)),
        ] {
            bar.spawn((
                Node {
                    width: Val::Px(80.0),
                    height: Val::Px(40.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BorderColor(Color::srgb(0.3, 0.3, 0.35)),
                BackgroundColor(Color::srgb(0.12, 0.12, 0.15)),
                HudRoot,
                HudHotbarSlot,
            )).with_children(|slot| {
                slot.spawn((label(&format!("[{key}]"), 11.0, color), HudRoot));
                slot.spawn((label(label_text, 13.0, Color::srgb(0.7, 0.7, 0.8)), HudRoot));
            });
        }
    });
}
