//! WoW-style action bar – 6 keybinded ability slots showing class-specific
//! abilities from the player's `PlayerClass` component.
//!
//! Each slot shows:
//! - Keybind label (1-6)
//! - Ability icon placeholder (colored box based on class)
//! - Ability name
//! - Cooldown overlay sweep (gray sweep from top to bottom)
//! - Highlight/border when ability is usable

use bevy::prelude::*;
use ir_core::{CharacterClass, ClassAbilityId};
use crate::hud::components::*;
use crate::ui_textures::UiTextureAssets;

fn label(s: &str, size: f32, color: Color) -> impl Bundle {
    (
        Text::new(s.to_string()),
        TextFont { font_size: size, ..default() },
        TextColor(color),
    )
}

/// Returns the 6 action-bar abilities for a given class.
/// Order: primary, secondary, cast, dash, utility, ultimate.
pub fn class_abilities(class: CharacterClass) -> [(ClassAbilityId, &'static str); 6] {
    let primary = class.primary_ability();
    let secondary = class.secondary_ability();
    let cast = class.cast_ability();
    let dash = class.dash_ability();
    // Extra abilities (utility + ultimate) — class-specific
    let (utility, ultimate) = match class {
        CharacterClass::Warrior => (
            ClassAbilityId::MeleeCleave, // placeholder — extra utility
            ClassAbilityId::ShieldBlock, // placeholder — extra ultimate
        ),
        CharacterClass::Paladin => (
            ClassAbilityId::RighteousStrike,
            ClassAbilityId::HolyLight,
        ),
        CharacterClass::Rogue => (
            ClassAbilityId::Backstab,
            ClassAbilityId::PoisonBlade,
        ),
        CharacterClass::Hunter => (
            ClassAbilityId::AimedShot,
            ClassAbilityId::MultiShot,
        ),
        CharacterClass::Mage => (
            ClassAbilityId::Fireball,
            ClassAbilityId::Frostbolt,
        ),
    };
    [
        (primary, "1"),
        (secondary, "2"),
        (cast, "3"),
        (dash, "4"),
        (utility, "5"),
        (ultimate, "6"),
    ]
}

/// Spawns the full ability action bar at bottom center.
pub fn spawn_action_bar(parent: &mut ChildBuilder, assets: &UiTextureAssets) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(72.0),
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
            // Spawn placeholder slots — class abilities set by update system
            // Default to Warrior until the update system overrides
            let slots = class_abilities(CharacterClass::Warrior);
            for (ability, keybind) in &slots {
                spawn_ability_slot(bar, keybind, ability, assets);
            }
        });
}

/// Spawns a single ability slot with keybind label, icon, name, border,
/// and cooldown overlay.
fn spawn_ability_slot(parent: &mut ChildBuilder, keybind: &str, ability: &ClassAbilityId, assets: &UiTextureAssets) {
    let border_color = Color::srgb(0.3, 0.3, 0.35);
    let keybind_color = Color::srgb(0.4, 0.7, 1.0);

    parent
        .spawn((
            Node {
                width: Val::Px(72.0),
                height: Val::Px(66.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(2.0)),
                row_gap: Val::Px(1.0),
                ..default()
            },
            // Base slot texture (dynamic border via BorderColor from update_ability_bar)
            ImageNode::new(assets.slot_bg.clone()),
            BorderColor(border_color),
            HudActionBarSlot,
        ))
        .with_children(|slot| {
            // ── Icon placeholder (colored box) ──────────────────────
            slot.spawn((
                Node {
                    width: Val::Px(40.0),
                    height: Val::Px(28.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.3, 0.3, 0.5)),
                HudActionBarIcon,
            ));

            // ── Keybind label ───────────────────────────────────────
            slot.spawn((
                label(&format!("[{keybind}]"), 10.0, keybind_color),
                HudKeybindLabel,
            ));

            // ── Ability name ────────────────────────────────────────
            slot.spawn((
                label(ability.display_name(), 10.0, Color::srgb(0.75, 0.75, 0.85)),
                HudActionBarSlot,
            ));

            // ── Cooldown overlay (hidden by default) ────────────────
            slot.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(0.0), // starts at 0%, sweeps down
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    ..default()
                },
                ImageNode::new(assets.cooldown_overlay.clone()),
                HudCooldownOverlay {
                    remaining: 0.0,
                    max: 1.0,
                },
            ));
        });
}
