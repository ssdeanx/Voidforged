//! Experience and leveling systems — processes XP gain events and applies level-up bonuses.

use bevy::prelude::*;
use ir_core::*;

/// Processes XP gain events and handles level-ups.
pub fn handle_xp_gain(
    mut xp_events: EventReader<ExperienceGainEvent>,
    mut level_up_events: EventWriter<LevelUpEvent>,
    mut player_query: Query<&mut Player>,
    mut progression: ResMut<RunProgression>,
    meta: Res<MetaProgression>,
) {
    let mut player = match player_query.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };
    let wisdom_mult = crate::upgrades::utility_tier_multiplier(&meta, "xp_boost");

    for event in xp_events.read() {
        let multiplied = (event.amount as f32 * wisdom_mult) as u64;
        player.experience += multiplied;
        progression.xp_earned += multiplied;

        // Check for level-up
        while player.experience >= player.xp_to_next {
            player.experience -= player.xp_to_next;
            player.level += 1;
            player.xp_to_next = (100.0 * 1.3_f64.powi(player.level as i32)) as u64;

            level_up_events.send(LevelUpEvent {
                new_level: player.level,
            });
        }
    }
}

/// Applies level-up bonuses to combat stats.
pub fn apply_level_up(
    mut level_up_events: EventReader<LevelUpEvent>,
    mut player_stats: Query<&mut CombatStats, With<Player>>,
    mut player_health: Query<&mut Health, With<Player>>,
) {
    for _event in level_up_events.read() {
        if let Ok(mut stats) = player_stats.get_single_mut() {
            stats.damage_bonus += 2.0;
        }
        if let Ok(mut health) = player_health.get_single_mut() {
            health.max += 10.0;
            health.current = health.max;
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_xp_to_next_formula() {
        let xp_to_next_l1 = (100.0 * 1.3_f64.powi(1)) as u64;
        assert_eq!(xp_to_next_l1, 130);
        let xp_to_next_l2 = (100.0 * 1.3_f64.powi(2)) as u64;
        assert_eq!(xp_to_next_l2, 169);
    }
}
