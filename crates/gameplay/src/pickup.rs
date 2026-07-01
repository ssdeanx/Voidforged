//! Pickup systems — XP gem magnet, health/gold collection, item pickup from ground.

use bevy::prelude::*;
use ir_core::*;
use crate::loot::ItemDrop;

/// Moves experience gems toward player when within magnet radius.
pub fn gem_magnet(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut gem_query: Query<(Entity, &mut ExperienceGem, &mut Transform), Without<Player>>,
    config: Res<GameConfig>,
) {
    let player_pos = match player_query.get_single() {
        Ok(t) => t.translation,
        Err(_) => return,
    };

    for (_entity, gem, mut transform) in gem_query.iter_mut() {
        let to_player = player_pos - transform.translation;
        let dist = to_player.length();

        if dist < config.xp_magnet_radius {
            let direction = to_player.normalize_or_zero();
            transform.translation += direction * gem.magnet_speed * time.delta_secs();
        }
    }
}

/// Collects health pickups — restores player HP on contact.
pub fn collect_health_pickups(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    pickup_query: Query<(Entity, &Transform, &Pickup), Without<Player>>,
    mut health_query: Query<&mut Health>,
) {
    let (player_entity, player_pos) = match player_query.get_single() {
        Ok(p) => (p.0, p.1.translation),
        Err(_) => return,
    };

    for (pickup_entity, pickup_transform, pickup) in pickup_query.iter() {
        if pickup.kind != PickupKind::Health {
            continue;
        }
        let dist = pickup_transform.translation.distance(player_pos);
        if dist < 1.5 {
            // Heal player
            if let Ok(mut health) = health_query.get_mut(player_entity) {
                health.heal(25.0);
            }
            commands.entity(pickup_entity).despawn();
        }
    }
}

/// Collects gold pickups — adds to meta-progression gold.
pub fn collect_gold_pickups(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    pickup_query: Query<(Entity, &Transform, &Pickup), Without<Player>>,
    mut meta: ResMut<MetaProgression>,
) {
    let player_pos = match player_query.get_single() {
        Ok(t) => t.translation,
        Err(_) => return,
    };

    for (pickup_entity, pickup_transform, pickup) in pickup_query.iter() {
        if pickup.kind != PickupKind::Gold {
            continue;
        }
        let dist = pickup_transform.translation.distance(player_pos);
        if dist < 1.5 {
            meta.gold += 10;
            commands.entity(pickup_entity).despawn();
        }
    }
}

/// Picks up nearby ItemDrop entities when the player presses F.
/// Adds the item to the player's inventory and despawns the drop.
pub fn item_pickup_interaction(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Transform, &mut Inventory), With<Player>>,
    drop_query: Query<(Entity, &Transform, &ItemDrop)>,
) {
    if !keyboard.just_pressed(KeyCode::KeyF) {
        return;
    }

    let Ok((player_tf, mut inventory)) = player_query.get_single_mut() else {
        return;
    };

    let player_pos = player_tf.translation;
    let pickup_range = 2.0;
    let mut nearest: Option<(Entity, f32)> = None;

    // Find the nearest pickup within range
    for (drop_entity, drop_tf, _drop) in drop_query.iter() {
        let dist = drop_tf.translation.distance(player_pos);
        if dist < pickup_range {
            match nearest {
                Some((_, d)) if dist < d => nearest = Some((drop_entity, dist)),
                None => nearest = Some((drop_entity, dist)),
                _ => {}
            }
        }
    }

    if let Some((target, _)) = nearest {
        if let Ok((_, _, drop)) = drop_query.get(target) {
            let item = ItemInstance::new(&drop.def_id);
            if inventory.add_item(item) {
                commands.entity(target).despawn();
                info!("Picked up: {}", drop.def_id);
            } else {
                warn!("Inventory full — cannot pick up {}", drop.def_id);
            }
        }
    }
}
