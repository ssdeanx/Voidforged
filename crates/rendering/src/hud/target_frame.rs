//! Target / Enemy unit frame – WoW-style enemy target display.
//!
//! Shows enemy name, level, health bar with percentage text, and an
//! elite-style dragon border for elite/boss enemies. Positioned at
//! top-center of the HUD.

use bevy::prelude::*;
use crate::hud::components::*;
use crate::ui_textures::UiTextureAssets;
use crate::ui_textures::make_9slice_node;

fn label(s: &str, size: f32, color: Color) -> impl Bundle {
    (
        Text::new(s.to_string()),
        TextFont { font_size: size, ..default() },
        TextColor(color),
    )
}

/// Spawns the target unit frame (hidden by default; shown via visibility).
pub fn spawn_target_frame(parent: &mut ChildBuilder, assets: &UiTextureAssets) {
    parent
        .spawn((
            Node {
                width: Val::Px(260.0),
                height: Val::Px(70.0),
                position_type: PositionType::Absolute,
                top: Val::Px(8.0),
                left: Val::Percent(50.0),
                margin: UiRect::left(Val::Px(-130.0)),
                border: UiRect::all(Val::Px(2.0)),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(3.0)),
                display: Display::None, // hidden until targeting
                ..default()
            },
            // 9-slice border texture (red default)
            make_9slice_node(assets.border_red.clone(), 2.0),
            BackgroundColor(Color::srgb(0.06, 0.06, 0.10)),
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
                        label("Enemy", 15.0, Color::srgb(1.0, 0.7, 0.3)),
                        HudTargetNameText,
                    ));
                    nl.spawn((
                        label("Lv. 1", 13.0, Color::srgb(0.8, 0.6, 0.4)),
                        HudTargetLevelText,
                    ));
                });

            // Health bar
            frame
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(22.0),
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    BorderColor(Color::srgb(0.35, 0.35, 0.4)),
                    BackgroundColor(Color::srgb(0.12, 0.03, 0.03)),
                    HudTargetHealthBar,
                ))
                .with_children(|hp_bg| {
                    // Fill bar – uses health gradient texture, width driven by update system
                    hp_bg.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        ImageNode::new(assets.bar_health.clone()),
                        HudTargetHealthBarFill,
                        HudTargetHealthDisplay::default(),
                    ));
                    // Percentage text overlay (centered)
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

            // Elite indicator (hidden by default, shown for elite/boss enemies)
            frame.spawn((
                Node {
                    width: Val::Px(0.0),
                    height: Val::Px(0.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(-4.0),
                    right: Val::Px(-4.0),
                    display: Display::None,
                    ..default()
                },
                HudTargetEliteBorder,
            ));
        });
}
