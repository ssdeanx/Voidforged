use bevy::prelude::*;
use ir_core::*;

/// Moves experience gems toward player when within magnet radius.
pub fn gem_magnet(
    _commands: Commands,
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
            // Attract toward player
            let direction = to_player.normalize_or_zero();
            transform.translation += direction * gem.magnet_speed * time.delta_secs();
        }
    }
}
