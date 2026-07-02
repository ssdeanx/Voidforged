//! Hitbox processing for player and enemy damage areas.
use bevy::prelude::*;
use ir_core::*;

pub fn process_hitboxes(
    mut commands: Commands, time: Res<Time>,
    mut hitboxes: Query<(Entity, &mut DamageHitbox, &Transform)>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut damage_events: EventWriter<DamageEvent>,
    mut dmg_num_events: EventWriter<DamageNumberEvent>,
    mut impact_events: EventWriter<SpawnImpactEvent>,
) {
    for (hitbox_entity, mut hitbox, tf) in hitboxes.iter_mut() {
        hitbox.lifetime -= time.delta_secs();
        let origin = tf.translation;
        let facing = *tf.forward();
        for (enemy_entity, enemy_tf) in enemies.iter() {
            if hitbox.hit_enemies.contains(&enemy_entity) { continue; }
            let to_enemy = enemy_tf.translation - origin;
            let dist = to_enemy.length();
            let dir = to_enemy.normalize_or_zero();
            let hit = match hitbox.shape {
                HitboxShape::Cone { range, half_angle } => dist <= range && facing.dot(dir) >= half_angle.cos(),
                HitboxShape::Circle { radius } => dist <= radius,
                HitboxShape::Rect { width, length } => {
                    let dot = facing.dot(dir);
                    let lateral = (dir - facing * dot).length();
                    dist <= length && lateral * dist <= width * 0.5
                }
                HitboxShape::Point { range } => dist <= range,
            };
            if hit {
                hitbox.hit_enemies.push(enemy_entity);
                let kb_dir = (enemy_tf.translation - origin).normalize_or_zero() * Vec3::new(1.0, 0.0, 1.0);
                if hitbox.knockback > 0.0 && kb_dir.length_squared() > 0.1 {
                    commands.entity(enemy_entity).insert(Knockback::new(kb_dir * hitbox.knockback * 5.0, 6.0));
                }
                if hitbox.hit_stun_duration > 0.0 { commands.entity(enemy_entity).insert(HitStun::new(hitbox.hit_stun_duration)); }
                if hitbox.hit_stop_duration > 0.0 { commands.entity(enemy_entity).insert(HitStop::new(hitbox.hit_stop_duration)); }
                if hitbox.hit_flash_duration > 0.0 { commands.entity(enemy_entity).insert(HitFlash::new(hitbox.hit_flash_duration)); }
                let damage_type = hitbox.damage_type.clone();
                let hit_pos = enemy_tf.translation + Vec3::Y * 1.0;
                damage_events.send(DamageEvent { target: enemy_entity, source: hitbox.source, amount: hitbox.damage, is_critical: false, damage_type: damage_type.clone(), hit_position: Some(hit_pos) });
                dmg_num_events.send(DamageNumberEvent { position: hit_pos, amount: hitbox.damage as i32, is_crit: false, damage_type: damage_type.clone() });
                let impact_color = match damage_type {
                    DamageType::Physical => Some(Vec4::new(1.0, 1.0, 1.0, 1.0)),
                    DamageType::Magic => Some(Vec4::new(0.8, 0.4, 1.0, 1.0)),
                    DamageType::True => Some(Vec4::new(1.0, 0.6, 0.0, 1.0)),
                };
                impact_events.send(SpawnImpactEvent { position: hit_pos, color: impact_color });
            }
        }
        if hitbox.lifetime <= 0.0 { commands.entity(hitbox_entity).despawn(); }
    }
}

pub fn process_enemy_hitboxes(
    mut commands: Commands, time: Res<Time>,
    mut hitboxes: Query<(Entity, &mut DamageHitbox, &Transform), With<EnemyHitbox>>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut damage_events: EventWriter<DamageEvent>,
    mut dmg_num_events: EventWriter<DamageNumberEvent>,
) {
    let Ok((player_entity, player_tf)) = player_query.get_single() else { return };
    for (hitbox_entity, mut hitbox, tf) in hitboxes.iter_mut() {
        hitbox.lifetime -= time.delta_secs();
        let origin = tf.translation;
        let to_player = player_tf.translation - origin;
        let dist = to_player.length();
        let hit = match hitbox.shape {
            HitboxShape::Circle { radius } => dist <= radius,
            HitboxShape::Point { range } => dist <= range,
            _ => dist <= 2.0,
        };
        if hit && !hitbox.hit_enemies.contains(&player_entity) {
            hitbox.hit_enemies.push(player_entity);
            let kb_dir = (player_tf.translation - origin).normalize_or_zero() * Vec3::new(1.0, 0.0, 1.0);
            if hitbox.knockback > 0.0 && kb_dir.length_squared() > 0.1 {
                commands.entity(player_entity).insert(Knockback::new(kb_dir * hitbox.knockback * 4.0, 5.0));
            }
            if hitbox.hit_stun_duration > 0.0 { commands.entity(player_entity).insert(HitStun::new(hitbox.hit_stun_duration)); }
            if hitbox.hit_stop_duration > 0.0 { commands.entity(player_entity).insert(HitStop::new(hitbox.hit_stop_duration)); }
            damage_events.send(DamageEvent { target: player_entity, source: hitbox.source, amount: hitbox.damage, is_critical: false, damage_type: hitbox.damage_type.clone(), hit_position: Some(player_tf.translation + Vec3::Y * 1.0) });
            dmg_num_events.send(DamageNumberEvent { position: player_tf.translation + Vec3::Y * 1.0, amount: hitbox.damage as i32, is_crit: false, damage_type: hitbox.damage_type.clone() });
        }
        if hitbox.lifetime <= 0.0 { commands.entity(hitbox_entity).despawn(); }
    }
}