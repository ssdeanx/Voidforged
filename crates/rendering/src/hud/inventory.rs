//! Inventory UI — grid bag view (5 columns × 4 rows) with item icons
//! colored by rarity, stack counts, select/equip interaction, and gold display.
//!
//! Toggled with the 'I' key. Drives EquipItemEvent on right-click and tracks
//! a selected slot for contextual actions.

use crate::hud::components::*;
use bevy::prelude::*;
use ir_core::*;
use crate::ui_textures::{UiTextureAssets, make_9slice_node};

const INVENTORY_COLS: usize = 5;
const INVENTORY_ROWS: usize = 4;
const _INVENTORY_SLOTS: usize = INVENTORY_COLS * INVENTORY_ROWS; // 20

// ── Helper ───────────────────────────────────────────────────────────────────

fn label(s: &str, size: f32, color: Color) -> impl Bundle {
    (
        Text::new(s.to_string()),
        TextFont { font_size: size, ..default() },
        TextColor(color),
    )
}

/// Returns a colour for the slot background based on item rarity (or a dim default for empty).
fn slot_bg_color(rarity: &ItemRarity) -> Color {
    let c = rarity.color().to_linear();
    Color::srgba(c.red, c.green, c.blue, 0.30)
}

// ── Spawn ───────────────────────────────────────────────────────────────────

/// Spawns the full inventory panel as a child of the HUD root.
/// Starts hidden (Display::None); toggled by `toggle_inventory`.
pub fn spawn_inventory(parent: &mut ChildBuilder, assets: &UiTextureAssets) {
    let panel_width = 480.0;
    let slot_size = 72.0;
    let gap = 6.0;
    let total_grid_width = INVENTORY_COLS as f32 * slot_size + (INVENTORY_COLS as f32 - 1.0) * gap;

    parent
        .spawn((
            Node {
                width: Val::Px(panel_width),
                height: Val::Auto,
                position_type: PositionType::Absolute,
                top: Val::Px(60.0),
                right: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(2.0)),
                display: Display::None, // hidden by default
                ..default()
            },
            // 9-slice border + dark panel background
            make_9slice_node(assets.border_default.clone(), 2.0),
            ImageNode::new(assets.panel_dark.clone()),
            HudInventory,
        ))
        .with_children(|panel| {
            // ── Title ────────────────────────────────────────────────
            panel.spawn((label("INVENTORY", 16.0, Color::srgb(0.8, 0.8, 0.9)), HudInventory));

            // ── Gold row ─────────────────────────────────────────────
            panel.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexStart,
                    column_gap: Val::Px(6.0),
                    ..default()
                },
                HudInventory,
            )).with_children(|gold_row| {
                gold_row.spawn((label("Gold:", 13.0, Color::srgb(1.0, 0.84, 0.0)), HudInventory));
                gold_row.spawn((label("0", 13.0, Color::srgb(1.0, 0.84, 0.0)), HudInventoryGold));
            });

            // ── Grid container ───────────────────────────────────────
            panel.spawn((
                Node {
                    width: Val::Px(total_grid_width),
                    height: Val::Auto,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(gap),
                    ..default()
                },
                HudInventory,
            )).with_children(|grid| {
                for row in 0..INVENTORY_ROWS {
                    grid.spawn((
                        Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(gap),
                            ..default()
                        },
                        HudInventory,
                    )).with_children(|row_node| {
                        for col in 0..INVENTORY_COLS {
                            let idx = row * INVENTORY_COLS + col;
                            spawn_inventory_slot(row_node, idx, assets);
                        }
                    });
                }
            });

            // ── Instructions ─────────────────────────────────────────
            panel.spawn((
                label("Left-click: select  |  Right-click: equip  |  I: toggle", 10.0, Color::srgb(0.5, 0.5, 0.55)),
                HudInventory,
            ));
        });
}

/// Spawns a single inventory slot as an interactive Button.
fn spawn_inventory_slot(parent: &mut ChildBuilder, index: usize, assets: &UiTextureAssets) {
    parent
        .spawn((
            Node {
                width: Val::Px(72.0),
                height: Val::Px(72.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            // Base slot texture (dynamic colors are applied via BackgroundColor in update_inventory)
            ImageNode::new(assets.slot_bg.clone()),
            make_9slice_node(assets.slot_border.clone(), 1.0),
            BorderColor(Color::srgb(0.25, 0.25, 0.3)),
            BackgroundColor(Color::srgba(0.12, 0.12, 0.16, 1.0)),
            HudInventorySlot(index),
            Button,
        ))
        .with_children(|slot| {
            // Icon placeholder — tinted by rarity (updated by system)
            slot.spawn((
                Node {
                    width: Val::Px(48.0),
                    height: Val::Px(40.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.2, 0.2, 0.25, 1.0)),
                HudInventorySlot(index),
            ));

            // Stack count label (hidden when 0/1)
            slot.spawn((
                label("", 10.0, Color::srgb(0.7, 0.7, 0.7)),
                HudInventorySlot(index),
            ));
        });
}

// ── Update System ────────────────────────────────────────────────────────────

/// Updates all inventory slot visuals — icon colour, border, stack text — from
/// the player's Inventory component. Runs every frame when inventory is visible.
pub fn update_inventory(
    player_query: Query<&Inventory, With<Player>>,
    mut slot_query: Query<(&HudInventorySlot, &mut BackgroundColor, &mut BorderColor)>,
    _item_db: Res<ItemDatabase>,
) {
    let Ok(inventory) = player_query.get_single() else { return };

    for (slot_idx, mut bg, mut border) in slot_query.iter_mut() {
        let item = inventory.get(slot_idx.0);
        match item {
            None => {
                bg.0 = Color::srgba(0.12, 0.12, 0.16, 1.0);
                border.0 = Color::srgb(0.25, 0.25, 0.3);
            }
            Some(inst) => {
                let rarity = &inst.rarity;
                bg.0 = slot_bg_color(rarity);
                border.0 = rarity.color();
            }
        }
    }
}

/// Updates stack count labels on each slot.
pub fn update_inventory_stack_text(
    player_query: Query<&Inventory, With<Player>>,
    mut slot_text_query: Query<(&HudInventorySlot, &mut Text)>,
) {
    let Ok(inventory) = player_query.get_single() else { return };

    for (slot_idx, mut text) in slot_text_query.iter_mut() {
        let item = inventory.get(slot_idx.0);
        match item {
            None => text.0.clear(),
            Some(inst) => {
                if inst.stack_count > 1 {
                    text.0 = format!("{}", inst.stack_count);
                } else {
                    text.0.clear();
                }
            }
        }
    }
}

/// Toggles inventory panel visibility when 'I' is pressed.
pub fn toggle_inventory(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut inv_query: Query<&mut Node, With<HudInventory>>,
) {
    if keyboard.just_pressed(KeyCode::KeyI) {
        for mut node in inv_query.iter_mut() {
            node.display = match node.display {
                Display::None => Display::Flex,
                _ => Display::None,
            };
        }
    }
}

/// Handles left-click selection of inventory slots. Tracks the selected slot
/// by adding/removing `HudInventorySelected`.
pub fn handle_inventory_left_click(
    mut commands: Commands,
    interaction_query: Query<(Entity, &Interaction, &HudInventorySlot), Changed<Interaction>>,
    selected_query: Query<Entity, With<HudInventorySelected>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    for (entity, interaction, _slot) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            // Clear previous selection
            for prev in selected_query.iter() {
                commands.entity(prev).remove::<HudInventorySelected>();
            }

            commands.entity(entity).insert(HudInventorySelected);
            info!("Selected inventory slot {}", _slot.0);
        }
    }
}

/// Handles right-click on an inventory slot → sends EquipItemEvent.
pub fn handle_inventory_right_click(
    mut events: EventWriter<EquipItemEvent>,
    player_query: Query<&Inventory, With<Player>>,
    interaction_query: Query<(&Interaction, &HudInventorySlot)>,
    item_db: Res<ItemDatabase>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if !mouse.just_pressed(MouseButton::Right) {
        return;
    }

    let Ok(inventory) = player_query.get_single() else { return };

    for (interaction, slot) in interaction_query.iter() {
        if *interaction == Interaction::Hovered || *interaction == Interaction::Pressed {
            let Some(item) = inventory.get(slot.0) else { continue };
            let Some(def) = item_db.get(&item.def_id) else { continue };
            if let Some(equip_slot) = def.slot {
                events.send(EquipItemEvent {
                    inventory_slot: slot.0,
                    equip_slot,
                });
                info!("Equipping item {} from slot {}", def.name, slot.0);
            }
        }
    }
}

/// Updates the gold text inside the inventory panel.
pub fn update_inventory_gold(
    player_query: Query<&Inventory, With<Player>>,
    mut gold_text: Query<&mut Text, With<HudInventoryGold>>,
) {
    let Ok(inventory) = player_query.get_single() else { return };
    for mut text in gold_text.iter_mut() {
        text.0 = format!("{}", inventory.gold);
    }
}
