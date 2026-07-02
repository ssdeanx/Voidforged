//! WoW-style action bar – 6 keybinded ability slots showing class-specific
//! abilities from the player's `PlayerClass` component.
//!
//! Each slot shows:
//! - Keybind label (1-6)
//! - Ability icon texture (from UiIconAssets, or class-colored fallback)
//! - Ability name
//! - Cooldown overlay sweep (gray sweep from top to bottom)
//! - Highlight/border when ability is usable

use bevy::prelude::*;
use ir_core::{CharacterClass, ClassAbilityId};
use crate::hud::components::*;
use crate::ui_textures::UiTextureAssets;
use crate::ui_icons::UiIconAssets;

fn label(s: &str, size: f32, color: Color) -> impl Bundle {
    (
        Text::new(s.to_string()),
        TextFont { font_size: size, ..default() },
        TextColor(color),
    )
}

/// Maps a ClassAbilityId to the icon ID string used in UiIconAssets.
/// Returns None if no matching icon texture exists.
fn ability_icon_id(ability: ClassAbilityId) -> Option<&'static str> {
    Some(match ability {
        ClassAbilityId::MeleeCleave => "icon_cleave",
        ClassAbilityId::ShieldBlock => return None,
        ClassAbilityId::Charge => return None,
        ClassAbilityId::CombatRoll => return None,
        ClassAbilityId::RighteousStrike => "icon_heal_strike",
        ClassAbilityId::HolyLight => "icon_heal_strike",
        ClassAbilityId::Consecration => "icon_consecration",
        ClassAbilityId::DivineSteed => return None,
        ClassAbilityId::Backstab => "icon_backstab",
        ClassAbilityId::PoisonBlade => "icon_poison",
        ClassAbilityId::Vanish => return None,
        ClassAbilityId::Shadowstep => return None,
        ClassAbilityId::AimedShot => return None,
        ClassAbilityId::MultiShot => return None,
        ClassAbilityId::Trap => "icon_trap",
        ClassAbilityId::Disengage => return None,
        ClassAbilityId::Fireball => "icon_fireball",
        ClassAbilityId::Frostbolt => return None,
        ClassAbilityId::ArcaneBlast => return None,
        ClassAbilityId::Blink => "icon_blink",
        // Utility abilities — no dedicated icons yet
        ClassAbilityId::WarCry => return None,
        ClassAbilityId::Taunt => return None,
        ClassAbilityId::BlessingOfMight => return None,
        ClassAbilityId::HammerOfJustice => return None,
        ClassAbilityId::DeadlyPoison => return None,
        ClassAbilityId::SmokeBomb => return None,
        ClassAbilityId::HuntersMark => return None,
        ClassAbilityId::CallPet => return None,
        ClassAbilityId::Blizzard => return None,
        ClassAbilityId::Combustion => return None,
        // Ultimate abilities — no dedicated icons yet
        ClassAbilityId::BerserkerRage => return None,
        ClassAbilityId::ShieldWall => return None,
        ClassAbilityId::DivineIntervention => return None,
        ClassAbilityId::AvengingWrath => return None,
        ClassAbilityId::Rupture => return None,
        ClassAbilityId::BladeFlurry => return None,
        ClassAbilityId::RapidFire => return None,
        ClassAbilityId::ExplosiveTrap => return None,
        ClassAbilityId::WaterElemental => return None,
        ClassAbilityId::Meteor => return None,
        // None slot (no ability)
        ClassAbilityId::None => return None,
    })
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
            ClassAbilityId::MeleeCleave,
            ClassAbilityId::ShieldBlock,
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
pub fn spawn_action_bar(parent: &mut ChildBuilder, assets: &UiTextureAssets, icons: &UiIconAssets) {
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
            // Spawn ability slots — class abilities set by update system
            // Default to Warrior until the update system overrides
            let slots = class_abilities(CharacterClass::Warrior);
            for (ability, keybind) in &slots {
                spawn_ability_slot(bar, keybind, ability, assets, icons);
            }
        });
}

/// Spawns a single ability slot with keybind label, icon, name, border,
/// and cooldown overlay.
fn spawn_ability_slot(
    parent: &mut ChildBuilder,
    keybind: &str,
    ability: &ClassAbilityId,
    assets: &UiTextureAssets,
    icons: &UiIconAssets,
) {
    let border_color = Color::srgb(0.3, 0.3, 0.35);
    let keybind_color = Color::srgb(0.4, 0.7, 1.0);

    // Look up the icon texture for this ability
    let icon_handle = ability_icon_id(*ability)
        .and_then(|icon_id| icons.get(icon_id))
        .unwrap_or_else(|| assets.slot_bg.clone());

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
            // ── Ability icon ────────────────────────────────────────
            slot.spawn((
                Node {
                    width: Val::Px(40.0),
                    height: Val::Px(28.0),
                    ..default()
                },
                ImageNode::new(icon_handle),
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
