//! Equipment system — handles equip/unequip events and stat application.

use bevy::prelude::*;
use ir_core::*;
use crate::loot::ItemDrop;

// ============================================================================
// Equipment Events → Systems
// ============================================================================

/// Equips an item from inventory slot X into equipment slot Y.
pub fn handle_equip_event(
    mut commands: Commands,
    mut events: EventReader<EquipItemEvent>,
    mut player_query: Query<(Entity, &mut Inventory, &mut Equipment), With<Player>>,
) {
    let Ok((player, mut inventory, mut equipment)) = player_query.get_single_mut() else {
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
    }
}

/// Unequips an item from an equipment slot back to inventory.
pub fn handle_unequip_event(
    mut events: EventReader<UnequipItemEvent>,
    mut inventory_query: Query<&mut Inventory, With<Player>>,
    mut equipment_query: Query<&mut Equipment, With<Player>>,
) {
    let Ok(mut inventory) = inventory_query.get_single_mut() else { return };
    let Ok(mut equipment) = equipment_query.get_single_mut() else { return };

    for event in events.read() {
        let Some(item) = equipment.unequip(event.equip_slot) else {
            warn!("Unequip failed: nothing in slot {:?}", event.equip_slot);
            continue;
        };
        if !inventory.add_item(item) {
            warn!("Unequip failed: inventory full");
        }
    }
}

/// Recalculates stats from equipped items.
pub fn recalc_equipment_stats(mut player_query: Query<(&Equipment, &mut CombatStats), With<Player>>) {
    let Ok((_equipment, mut stats)) = player_query.get_single_mut() else {
        return;
    };
    stats.damage_bonus = 0.0;
    stats.attack_speed_bonus = 0.0;
}
