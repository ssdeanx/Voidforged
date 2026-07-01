//! Equipment system — handles equip/unequip events and stat application.

use bevy::prelude::*;
use ir_core::*;
use crate::loot::ItemDrop;

// ============================================================================
// Equipment Events → Systems
// ============================================================================

/// Equips an item from inventory slot X into equipment slot Y.
/// Recalculates stats after the swap.
pub fn handle_equip_event(
    mut commands: Commands,
    item_db: Res<ir_core::ItemDatabase>,
    mut events: EventReader<EquipItemEvent>,
    mut player_query: Query<(Entity, &mut Inventory, &mut Equipment, &mut CombatStats), With<Player>>,
) {
    let Ok((_player, mut inventory, mut equipment, mut stats)) = player_query.get_single_mut() else {
        return;
    };

    for event in events.read() {
        let Some(item) = inventory.remove_item(event.inventory_slot) else {
            warn!("Equip failed: no item in inventory slot {}", event.inventory_slot);
            continue;
        };

        if let Some(old) = equipment.equip(item, event.equip_slot) {
            if !inventory.add_item(old) {
                commands.spawn((
                    ItemDrop { def_id: "unknown".to_string() },
                    Transform::from_translation(Vec3::ZERO),
                ));
            }
        }
        info!("Equipped item in slot {:?}", event.equip_slot);

        // Recalculate stats from all equipped items
        recalc_equipment_stats(&item_db, &equipment, &mut *stats);
    }
}

/// Unequips an item from an equipment slot back to inventory.
/// Recalculates stats after the swap.
pub fn handle_unequip_event(
    mut events: EventReader<UnequipItemEvent>,
    item_db: Res<ir_core::ItemDatabase>,
    mut inventory_query: Query<&mut Inventory, With<Player>>,
    mut equipment_query: Query<(&mut Equipment, &mut CombatStats), With<Player>>,
) {
    let Ok(mut inventory) = inventory_query.get_single_mut() else { return };
    let Ok((mut equipment, mut stats)) = equipment_query.get_single_mut() else { return };

    for event in events.read() {
        let Some(item) = equipment.unequip(event.equip_slot) else {
            warn!("Unequip failed: nothing in slot {:?}", event.equip_slot);
            continue;
        };
        if !inventory.add_item(item) {
            warn!("Unequip failed: inventory full");
        }

        // Recalculate stats from remaining equipped items
        recalc_equipment_stats(&item_db, &equipment, &mut *stats);
    }
}

/// Adds equipment stat bonuses to CombatStats WITHOUT zeroing existing stats.
/// This preserves class base stats (set by CharacterClass::base_stats) and
/// meta-progression bonuses — only the equipment contribution is added.
/// Safe to call repeatedly as long as the baseline stats are correct.
pub fn recalc_equipment_stats(
    item_db: &ir_core::ItemDatabase,
    equip: &Equipment,
    stats: &mut CombatStats,
) {
    let _changes = equip.apply_stats(item_db, stats);
}
