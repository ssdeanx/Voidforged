//! WoW-style action bar – 6 keybinded ability slots (1-4, Q, E).
//!
//! Each slot shows keybind, ability name, class-colored border,
//! and a cooldown overlay when the ability is on cooldown.

use bevy::prelude::*;
use crate::hud::components::*;

fn label(s: &str, size: f32, color: Color) -> impl Bundle {
    (
        Text::new(s.to_string()),
        TextFont { font_size: size, ..default() },
        TextColor(color),
    )
}

/// Slot definitions: (keybind label, ability name, unique marker suffix)
const SLOTS: &[(&str, &str)] = &[
    ("1", "Cleave"),
    ("2", "Shield"),
    ("3", "Charge"),
    ("4", "Ultimate"),
    ("Q", "Cast"),
    ("E", "Special"),
];

/// Spawns the full ability action bar at bottom center.
pub fn spawn_action_bar(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(66.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(4.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(6.0),
                ..default()
            },
            HudActionBar,
        ))
        .with_children(|bar| {
            for (keybind, ability_name) in SLOTS {
                spawn_ability_slot(bar, keybind, ability_name);
            }
        });
}

/// Spawns a single ability slot with keybind label, name, border, and cooldown overlay.
fn spawn_ability_slot(parent: &mut ChildBuilder, keybind: &str, ability_name: &str) {
    let border_color = Color::srgb(0.3, 0.3, 0.35); // will be updated per class
    let keybind_color = Color::srgb(0.4, 0.7, 1.0);

    parent
        .spawn((
            Node {
                width: Val::Px(72.0),
                height: Val::Px(60.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(2.0)),
                row_gap: Val::Px(2.0),
                ..default()
            },
            BorderColor(border_color),
            BackgroundColor(Color::srgb(0.10, 0.10, 0.13)),
            HudActionBarSlot,
        ))
        .with_children(|slot| {
            // Keybind label
            slot.spawn((
                label(&format!("[{keybind}]"), 11.0, keybind_color),
                HudActionBarSlot,
            ));
            // Ability name
            slot.spawn((
                label(ability_name, 12.0, Color::srgb(0.75, 0.75, 0.85)),
                HudActionBarSlot,
            ));
            // Cooldown overlay (hidden by default)
            slot.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    display: Display::None,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.55)),
                HudCooldownOverlay,
            ));
        });
}
