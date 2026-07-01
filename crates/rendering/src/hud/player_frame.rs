//! Player unit frame – WoW-style portrait + health bar + class resource + stamina.
//!
//! Spawned inside the HUD root at top-left.

use bevy::prelude::*;
use ir_core::CharacterClass;
use crate::hud::components::*;

// ── Class color helpers ──────────────────────────────────────────────────────

pub fn class_portrait_color(class: &CharacterClass) -> Color {
    match class {
        CharacterClass::Warrior => Color::srgb(0.77, 0.12, 0.23),
        CharacterClass::Paladin => Color::srgb(0.96, 0.55, 0.73),
        CharacterClass::Rogue => Color::srgb(1.0, 0.96, 0.41),
        CharacterClass::Hunter => Color::srgb(0.67, 0.83, 0.45),
        CharacterClass::Mage => Color::srgb(0.41, 0.80, 0.94),
    }
}

pub fn class_border_color(class: &CharacterClass) -> Color {
    match class {
        CharacterClass::Warrior => Color::srgb(0.77, 0.12, 0.23),
        CharacterClass::Paladin => Color::srgb(1.0, 0.84, 0.0),
        CharacterClass::Rogue => Color::srgb(0.0, 0.72, 0.0),
        CharacterClass::Hunter => Color::srgb(0.0, 0.65, 0.21),
        CharacterClass::Mage => Color::srgb(0.30, 0.50, 1.0),
    }
}

pub fn class_resource_color(class: &CharacterClass) -> Color {
    match class {
        CharacterClass::Warrior => Color::srgb(0.77, 0.12, 0.23),
        CharacterClass::Paladin => Color::srgb(1.0, 0.84, 0.0),
        CharacterClass::Rogue => Color::srgb(1.0, 0.96, 0.41),
        CharacterClass::Hunter => Color::srgb(0.67, 0.83, 0.45),
        CharacterClass::Mage => Color::srgb(0.41, 0.80, 0.94),
    }
}

/// Helper to build a text label bundle for UI text.
fn label(s: &str, size: f32, color: Color) -> impl Bundle {
    (
        Text::new(s.to_string()),
        TextFont { font_size: size, ..default() },
        TextColor(color),
    )
}

/// Spawns the complete player unit frame at top-left of the HUD.
pub fn spawn_player_frame(parent: &mut ChildBuilder) {
    // Outer frame container – class-colored border
    parent
        .spawn((
            Node {
                width: Val::Px(310.0),
                height: Val::Px(110.0),
                position_type: PositionType::Absolute,
                top: Val::Px(8.0),
                left: Val::Px(8.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::FlexStart,
                padding: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            HudPlayerFrame,
        ))
        .with_children(|frame| {
            // ── Portrait ────────────────────────────────────────────
            frame.spawn((
                Node {
                    width: Val::Px(56.0),
                    height: Val::Px(56.0),
                    border: UiRect::all(Val::Px(2.0)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    margin: UiRect::right(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.12, 0.12, 0.15)),
                BorderColor(Color::srgb(0.4, 0.4, 0.5)),
                HudPlayerPortrait,
            )).with_children(|portrait| {
                // Class icon placeholder — a filled square
                portrait.spawn((
                    Node {
                        width: Val::Px(44.0),
                        height: Val::Px(44.0),
                        ..default()
                    },
                    BackgroundColor(class_portrait_color(&CharacterClass::Warrior)),
                    HudPlayerPortrait,
                ));
            });

            // ── Info column ─────────────────────────────────────────
            frame
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        row_gap: Val::Px(2.0),
                        width: Val::Px(230.0),
                        ..default()
                    },
                    HudPlayerFrame,
                ))
                .with_children(|info| {
                    // Name + Level row
                    info.spawn((
                        Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(8.0),
                            ..default()
                        },
                        HudPlayerFrame,
                    )).with_children(|nl| {
                        nl.spawn((label("Player", 15.0, Color::WHITE), HudPlayerNameText));
                        nl.spawn((label("Lv. 1", 13.0, Color::srgb(0.7, 0.5, 1.0)), HudPlayerLevelText));
                    });

                    // Health bar
                    info.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(18.0),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        BorderColor(Color::srgb(0.3, 0.3, 0.35)),
                        BackgroundColor(Color::srgb(0.15, 0.04, 0.04)),
                        HudHealthBarBorder,
                    )).with_children(|hp_bg| {
                        hp_bg.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.0, 0.72, 0.12)),
                            HudHealthBar,
                        ));
                        // HP text overlay centered
                        hp_bg.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            HudHealthBarText,
                        )).with_children(|hp_text| {
                            hp_text.spawn((
                                label("100/100", 13.0, Color::WHITE),
                                HudHealthBarText,
                            ));
                        });
                    });

                    // Class resource bar
                    info.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(10.0),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        BorderColor(Color::srgb(0.25, 0.25, 0.3)),
                        BackgroundColor(Color::srgb(0.08, 0.08, 0.1)),
                        HudResourceBar,
                    )).with_children(|res_bg| {
                        res_bg.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.77, 0.12, 0.23)), // warrior rage default
                            HudResourceBarFill,
                        ));
                        res_bg.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            HudResourceBarText,
                        )).with_children(|res_text| {
                            res_text.spawn((
                                label("Rage 0/100", 9.0, Color::WHITE),
                                HudResourceBarText,
                            ));
                        });
                    });

                    // Stamina bar
                    info.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(8.0),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        BorderColor(Color::srgb(0.3, 0.3, 0.25)),
                        BackgroundColor(Color::srgb(0.1, 0.08, 0.03)),
                        HudStaminaBar,
                    )).with_children(|stam_bg| {
                        stam_bg.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(1.0, 0.75, 0.05)),
                            HudStaminaBarFill,
                        ));
                        stam_bg.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            HudStaminaBarText,
                        )).with_children(|stam_text| {
                            stam_text.spawn((
                                label("100/100", 8.0, Color::WHITE),
                                HudStaminaBarText,
                            ));
                        });
                    });
                });
        });
}
