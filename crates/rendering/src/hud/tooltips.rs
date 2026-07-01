//! Item tooltip system — shows a detailed popup when hovering over inventory
//! or equipment slots. Tooltip displays item name (colored by rarity), item type,
//! base stats (green +X), description, required level, and durability.
//!
//! The tooltip follows the mouse cursor and stays visible while hovering.

use crate::hud::components::*;
use bevy::prelude::*;
use ir_core::*;

/// Number of stat lines we pre-allocate in the tooltip.
const MAX_TOOLTIP_LINES: usize = 12;

/// All eight equipment slots for scanning.
const ALL_SLOTS: &[EquipSlot] = &[
    EquipSlot::MainHand,
    EquipSlot::OffHand,
    EquipSlot::Helmet,
    EquipSlot::Chest,
    EquipSlot::Boots,
    EquipSlot::Ring,
    EquipSlot::Amulet,
    EquipSlot::Trinket,
];

// ── Helper ───────────────────────────────────────────────────────────────────

fn label(s: &str, size: f32, color: Color) -> impl Bundle {
    (
        Text::new(s.to_string()),
        TextFont { font_size: size, ..default() },
        TextColor(color),
    )
}

fn category_name(cat: ItemCategory) -> &'static str {
    match cat {
        ItemCategory::Weapon => "Weapon",
        ItemCategory::Armor => "Armor",
        ItemCategory::Accessory => "Accessory",
        ItemCategory::Consumable => "Consumable",
        ItemCategory::Material => "Material",
        ItemCategory::Quest => "Quest",
    }
}

/// Returns a list of tooltip text lines for a given item instance + definition.
fn build_tooltip_lines(inst: &ItemInstance, def: &ItemDef) -> Vec<(String, Color)> {
    let mut lines: Vec<(String, Color)> = Vec::with_capacity(MAX_TOOLTIP_LINES);

    // 1. Item name (rarity coloured)
    lines.push((def.name.to_string(), def.rarity.color()));

    // 2. Item category + type
    let slot_name = def
        .slot
        .map(|s| s.display_name().to_string())
        .unwrap_or_else(|| "—".to_string());
    lines.push((
        format!("{} — {}", category_name(def.category), slot_name),
        Color::srgb(0.6, 0.6, 0.7),
    ));

    // 3. Rarity label
    lines.push((def.rarity.label().to_string(), def.rarity.color()));

    // 4. Blank separator
    lines.push((String::new(), Color::WHITE));

    // 5. Base stats
    if !def.base_stats.is_empty() {
        for stat_mod in &def.base_stats {
            let value_str = stat_mod.stat.format_value(stat_mod.value);
            lines.push((
                format!("{} {}", value_str, stat_mod.stat.display_name()),
                Color::srgb(0.3, 0.9, 0.3), // green for positive
            ));
        }
        lines.push((String::new(), Color::WHITE));
    }

    // 6. Description
    if !def.description.is_empty() {
        lines.push((def.description.to_string(), Color::srgb(0.7, 0.7, 0.75)));
        lines.push((String::new(), Color::WHITE));
    }

    // 7. Required level
    if def.required_level > 1 {
        lines.push((
            format!("Requires Level {}", def.required_level),
            Color::srgb(0.8, 0.6, 0.3),
        ));
    }

    // 8. Durability
    let current_durability = (inst.durability * inst.max_durability * 100.0).round() / 100.0;
    lines.push((
        format!("Durability: {:.0} / {:.0}", current_durability, inst.max_durability),
        if inst.durability > 0.3 {
            Color::srgb(0.6, 0.6, 0.6)
        } else {
            Color::srgb(1.0, 0.3, 0.3)
        },
    ));

    // 9. Vendor price
    if def.vendor_price > 0 {
        lines.push((
            format!("Sell Price: {}g", def.vendor_price),
            Color::srgb(1.0, 0.84, 0.0),
        ));
    }

    lines
}

// ── Spawn ───────────────────────────────────────────────────────────────────

/// Spawns the tooltip container (hidden by default) as a child of the HUD root.
/// The tooltip updates its text and position every frame.
pub fn spawn_tooltip(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Node {
                width: Val::Auto,
                height: Val::Auto,
                position_type: PositionType::Absolute,
                left: Val::Px(0.0), // updated by cursor-follow system
                top: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(8.0)),
                border: UiRect::all(Val::Px(1.0)),
                display: Display::None, // hidden until hovering
                ..default()
            },
            BorderColor(Color::srgb(0.35, 0.35, 0.45)),
            BackgroundColor(Color::srgba(0.06, 0.06, 0.10, 0.97)),
            HudTooltip,
        ))
        .with_children(|tip| {
            // Pre-spawn max tooltip lines (most will be invisible/empty)
            for i in 0..MAX_TOOLTIP_LINES {
                tip.spawn((label("", 12.0, Color::WHITE), HudTooltipLine(i)));
            }
        });
}

// ── Update System ────────────────────────────────────────────────────────────

/// Full scan of all interactable slots for hover state. Runs every frame when
/// inventory/equipment panels are visible. Shows/hides and positions the tooltip.
pub fn update_tooltip(
    windows: Query<&Window>,
    player_query: Query<(&Inventory, &Equipment), With<Player>>,
    item_db: Res<ItemDatabase>,
    inv_slots: Query<(Entity, &Interaction, &HudInventorySlot)>,
    equip_slots: Query<(Entity, &Interaction, &HudEquipSlot)>,
    mut tip_node: Query<
        &mut Node,
        (With<HudTooltip>, Without<HudInventory>, Without<HudEquipment>),
    >,
    mut tip_lines: Query<(&HudTooltipLine, &mut Text, &mut TextColor)>,
    inv_state: Query<&Node, (With<HudInventory>, Without<HudTooltip>)>,
    equip_state: Query<&Node, (With<HudEquipment>, Without<HudTooltip>)>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    let Ok((inventory, equipment)) = player_query.get_single() else {
        return;
    };

    let mut tip_node = match tip_node.get_single_mut() {
        Ok(n) => n,
        Err(_) => return,
    };

    // Check visibility of panels
    let any_visible = inv_state
        .iter()
        .chain(equip_state.iter())
        .any(|n| n.display != Display::None);

    if !any_visible {
        tip_node.display = Display::None;
        return;
    }

    // Find a hovered item from inventory or equipment
    let mut hovered_target: Option<(usize, bool)> = None; // (slot index, is_inventory)

    // Check inventory slots
    for (_entity, interaction, slot) in inv_slots.iter() {
        if *interaction == Interaction::Hovered && inventory.get(slot.0).is_some() {
            hovered_target = Some((slot.0, true));
            break;
        }
    }

    // Check equipment slots if no inventory item was found
    if hovered_target.is_none() {
        for (_entity, interaction, equip_slot) in equip_slots.iter() {
            if *interaction == Interaction::Hovered && equipment.get(equip_slot.0).is_some() {
                // Find the index in ALL_SLOTS for lookup
                if let Some(idx) = ALL_SLOTS.iter().position(|s| *s == equip_slot.0) {
                    hovered_target = Some((idx, false));
                }
                break;
            }
        }
    }

    if let Some((idx, is_inventory)) = hovered_target {
        let inst = if is_inventory {
            inventory.get(idx)
        } else {
            ALL_SLOTS.get(idx).and_then(|s| equipment.get(*s))
        };

        let Some(inst) = inst else {
            tip_node.display = Display::None;
            return;
        };

        let Some(def) = item_db.get(&inst.def_id) else {
            tip_node.display = Display::None;
            return;
        };

        let lines = build_tooltip_lines(inst, def);

        // Show tooltip
        tip_node.display = Display::Flex;

        // Position near cursor
        if let Some(cursor_pos) = window.cursor_position() {
            let window_width = window.width();
            let window_height = window.height();

            let estimated_width = 280.0;
            let estimated_height = (lines.len() as f32) * 16.0 + 16.0;

            let mut x = cursor_pos.x + 16.0;
            let mut y = cursor_pos.y - 8.0;

            // Flip to left if near right edge
            if x + estimated_width > window_width {
                x = cursor_pos.x - estimated_width - 8.0;
            }
            // Flip above if near bottom edge
            if y + estimated_height > window_height {
                y = cursor_pos.y - estimated_height - 8.0;
            }

            tip_node.left = Val::Px(x);
            tip_node.top = Val::Px(y);
        }

        // Update tooltip text lines
        for (line_marker, mut text, mut color) in tip_lines.iter_mut() {
            if line_marker.0 < lines.len() {
                let (content, line_color) = &lines[line_marker.0];
                text.0 = content.clone();
                color.0 = *line_color;
            } else {
                text.0.clear();
            }
        }
    } else {
        // Nothing hovered — hide tooltip
        tip_node.display = Display::None;
    }
}
