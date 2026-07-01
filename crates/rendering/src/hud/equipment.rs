//! Equipment screen — paperdoll layout showing 8 equipped gear slots with
//! placeholder icons, GearScore total, and click-to-unequip interaction.
//!
//! Positioned beside or below the inventory panel. Uses the same 'I' toggle.

use bevy::prelude::*;
use ir_core::*;
use crate::hud::components::*;
use crate::ui_textures::{UiTextureAssets, make_9slice_node};

// ── Helper ───────────────────────────────────────────────────────────────────

fn label(s: &str, size: f32, color: Color) -> impl Bundle {
    (
        Text::new(s.to_string()),
        TextFont { font_size: size, ..default() },
        TextColor(color),
    )
}

/// All eight equipment slots in display order.
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

// ── Spawn ───────────────────────────────────────────────────────────────────

/// Spawns the equipment paperdoll as a child of the HUD root.
/// Starts hidden (same toggle as inventory).
pub fn spawn_equipment(parent: &mut ChildBuilder, assets: &UiTextureAssets) {
    parent
        .spawn((
            Node {
                width: Val::Px(440.0),
                height: Val::Auto,
                position_type: PositionType::Absolute,
                top: Val::Px(60.0),
                right: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(2.0)),
                display: Display::None, // hidden by default; same toggle as inventory
                ..default()
            },
            // 9-slice border + dark panel background
            make_9slice_node(assets.border_default.clone(), 2.0),
            ImageNode::new(assets.panel_dark.clone()),
            HudEquipment,
        ))
        .with_children(|panel| {
            // ── Title ────────────────────────────────────────────────
            panel.spawn((
                label("EQUIPMENT", 16.0, Color::srgb(0.8, 0.8, 0.9)),
                HudEquipment,
            ));

            // ── Paperdoll grid: 2 columns × 4 rows ──────────────────
            // Two columns: Left (MainHand, Helmet, Boots, Ring) and Right (OffHand, Chest, Amulet, Trinket)
            // This gives a nice symmetrical paperdoll layout.

            let slot_size = 80.0;
            let gap = 8.0;

            // Split the 8 slots into two columns of 4
            let left_slots = [ALL_SLOTS[0], ALL_SLOTS[2], ALL_SLOTS[4], ALL_SLOTS[6]];
            let right_slots = [ALL_SLOTS[1], ALL_SLOTS[3], ALL_SLOTS[5], ALL_SLOTS[7]];

            let col_width = slot_size + 60.0; // slot + label area

            panel
                .spawn((
                    Node {
                        width: Val::Px(col_width * 2.0 + gap),
                        height: Val::Auto,
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(gap),
                        ..default()
                    },
                    HudEquipment,
                ))
                .with_children(|paperdoll| {
                    // Left column
                    paperdoll.spawn((
                        Node {
                            width: Val::Px(col_width),
                            height: Val::Auto,
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(gap),
                            ..default()
                        },
                        HudEquipment,
                    )).with_children(|col| {
                        for slot in &left_slots {
                            spawn_equip_slot(col, *slot, slot_size, assets);
                        }
                    });

                    // Right column
                    paperdoll.spawn((
                        Node {
                            width: Val::Px(col_width),
                            height: Val::Auto,
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(gap),
                            ..default()
                        },
                        HudEquipment,
                    )).with_children(|col| {
                        for slot in &right_slots {
                            spawn_equip_slot(col, *slot, slot_size, assets);
                        }
                    });
                });

            // ── GearScore total ──────────────────────────────────────
            panel.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(6.0),
                    padding: UiRect::all(Val::Px(4.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BorderColor(Color::srgb(0.25, 0.25, 0.3)),
                BackgroundColor(Color::srgba(0.12, 0.12, 0.16, 1.0)),
                HudEquipment,
            )).with_children(|gs_row| {
                gs_row.spawn((
                    label("GearScore:", 14.0, Color::srgb(0.7, 0.8, 1.0)),
                    HudEquipment,
                ));
                gs_row.spawn((
                    label("0", 14.0, Color::srgb(1.0, 0.84, 0.4)),
                    HudGearScoreText,
                ));
            });

            // ── Instructions ─────────────────────────────────────────
            panel.spawn((
                label("Click item: unequip  |  I: toggle", 10.0, Color::srgb(0.5, 0.5, 0.55)),
                HudEquipment,
            ));
        });
}

/// Spawns a single equipment slot with label and icon placeholder.
fn spawn_equip_slot(parent: &mut ChildBuilder, equip_slot: EquipSlot, size: f32, assets: &UiTextureAssets) {
    parent
        .spawn((
            Node {
                width: Val::Px(size + 60.0),
                height: Val::Px(size),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(6.0),
                ..default()
            },
            HudEquipment,
        ))
        .with_children(|row| {
            // Slot label
            row.spawn((
                label(equip_slot.display_name(), 11.0, Color::srgb(0.5, 0.5, 0.6)),
                HudEquipSlot(equip_slot),
            ));

            // Slot icon (interactive button) with base texture + dynamic BackgroundColor
            row.spawn((
                Node {
                    width: Val::Px(size),
                    height: Val::Px(size),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                ImageNode::new(assets.slot_bg.clone()),
                make_9slice_node(assets.slot_border.clone(), 1.0),
                BorderColor(Color::srgb(0.25, 0.25, 0.3)),
                BackgroundColor(Color::srgba(0.12, 0.12, 0.16, 1.0)),
                HudEquipSlot(equip_slot),
                Button,
            )).with_children(|icon| {
                // Placeholder icon (tinted by item rarity, or dim empty)
                icon.spawn((
                    Node {
                        width: Val::Px(size - 12.0),
                        height: Val::Px(size - 12.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.2, 0.2, 0.25, 1.0)),
                    HudEquipSlot(equip_slot),
                ));
            });
        });
}

// ── Update Systems ───────────────────────────────────────────────────────────

/// Updates equipment slot visuals — icon colour and border — from the player's
/// Equipment component.
pub fn update_equipment(
    player_query: Query<&Equipment, With<Player>>,
    mut slot_query: Query<(&HudEquipSlot, &mut BackgroundColor, &mut BorderColor)>,
    _children_query: Query<&Children>,
    _item_db: Res<ItemDatabase>,
) {
    let Ok(equipment) = player_query.get_single() else { return };

    for (slot_marker, mut bg, mut border) in slot_query.iter_mut() {
        let item = equipment.get(slot_marker.0);
        match item {
            None => {
                bg.0 = Color::srgba(0.12, 0.12, 0.16, 1.0);
                border.0 = Color::srgb(0.25, 0.25, 0.3);
            }
            Some(inst) => {
                let rarity = &inst.rarity;
                let c = rarity.color().to_linear();
                bg.0 = Color::srgba(c.red, c.green, c.blue, 0.35);
                border.0 = rarity.color();
            }
        }
    }
}

/// Updates the GearScore total text.
pub fn update_gear_score(
    player_query: Query<&Equipment, With<Player>>,
    item_db: Res<ItemDatabase>,
    mut gs_text: Query<&mut Text, With<HudGearScoreText>>,
) {
    let Ok(equipment) = player_query.get_single() else { return };

    let total_gs: u32 = ALL_SLOTS.iter()
        .filter_map(|slot| equipment.get(*slot))
        .filter_map(|inst| item_db.get(&inst.def_id))
        .map(|def| gear_score(def))
        .sum();

    for mut text in gs_text.iter_mut() {
        text.0 = format!("{}", total_gs);
    }
}

/// Handles clicking on an equipped item → sends UnequipItemEvent.
pub fn handle_equip_slot_click(
    mut events: EventWriter<UnequipItemEvent>,
    interaction_query: Query<(&Interaction, &HudEquipSlot), Changed<Interaction>>,
    mouse: Res<ButtonInput<MouseButton>>,
    player_query: Query<&Equipment, With<Player>>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(equipment) = player_query.get_single() else { return };

    for (interaction, equip_slot_marker) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            // Only send unequip if there's actually an item in that slot
            if equipment.get(equip_slot_marker.0).is_some() {
                events.send(UnequipItemEvent {
                    equip_slot: equip_slot_marker.0,
                });
                info!("Unequipping slot {:?}", equip_slot_marker.0);
            }
        }
    }
}
