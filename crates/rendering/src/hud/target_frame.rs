//! Target / Enemy unit frame – shows when targeting an enemy.
//!
//! Positioned at top-center of the HUD. Displays enemy name, level,
//! health bar with percentage, and border tinted by enemy type.

use bevy::prelude::*;
use crate::hud::components::*;

fn label(s: &str, size: f32, color: Color) -> impl Bundle {
    (
        Text::new(s.to_string()),
        TextFont { font_size: size, ..default() },
        TextColor(color),
    )
}

/// Spawns the target unit frame (hidden by default; shown via visibility).
pub fn spawn_target_frame(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Node {
                width: Val::Px(250.0),
                height: Val::Px(60.0),
                position_type: PositionType::Absolute,
                top: Val::Px(8.0),
                left: Val::Percent(50.0),
                margin: UiRect::left(Val::Px(-125.0)),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(2.0)),
                display: Display::None, // hidden until targeting
                ..default()
            },
            HudTargetFrame,
        ))
        .with_children(|frame| {
            // Name + Level row
            frame
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    HudTargetFrame,
                ))
                .with_children(|nl| {
                    nl.spawn((
                        label("Enemy", 14.0, Color::srgb(1.0, 0.7, 0.3)),
                        HudTargetNameText,
                    ));
                    nl.spawn((
                        label("Lv. 1", 12.0, Color::srgb(0.8, 0.6, 0.4)),
                        HudTargetLevelText,
                    ));
                });

            // Health bar
            frame
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(20.0),
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    BorderColor(Color::srgb(0.35, 0.35, 0.4)),
                    BackgroundColor(Color::srgb(0.12, 0.03, 0.03)),
                    HudTargetHealthBar,
                ))
                .with_children(|hp_bg| {
                    hp_bg.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.0, 0.65, 0.1)),
                        HudTargetHealthBarFill,
                    ));
                    hp_bg
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            HudTargetHealthPctText,
                        ))
                        .with_children(|pct_text| {
                            pct_text.spawn((
                                label("100%", 14.0, Color::WHITE),
                                HudTargetHealthPctText,
                            ));
                        });
                });
        });
}
