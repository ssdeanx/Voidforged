//! HUD update systems: health, xp, level, gold, dash, zone, prompt.

use bevy::prelude::*;
use ir_core::*;
use crate::hud::components::*;

pub fn update_health_bar(
    player_query: Query<&Health, With<Player>>,
    mut bar_query: Query<&mut Node, With<HudHealthBar>>,
    mut hp_text: Query<&mut Text, With<HudHpText>>,
) {
    let health = match player_query.get_single() {
        Ok(h) => h,
        Err(_) => return,
    };
    let pct = health.fraction();
    for mut node in bar_query.iter_mut() {
        node.width = Val::Percent(pct * 100.0);
    }
    for mut text in hp_text.iter_mut() {
        text.0 = format!("{}/{}", health.current as u32, health.max as u32);
    }
}

pub fn update_xp_bar(
    player_query: Query<&Player>,
    mut bar_query: Query<&mut Node, With<HudXpBar>>,
) {
    let player = match player_query.get_single() {
        Ok(p) => p,
        Err(_) => return,
    };
    let pct = if player.xp_to_next > 0 {
        player.experience as f32 / player.xp_to_next as f32
    } else {
        0.0
    };
    for mut node in bar_query.iter_mut() {
        node.width = Val::Percent(pct.min(1.0) * 100.0);
    }
}

pub fn update_level_text(
    player_query: Query<&Player>,
    mut text_query: Query<&mut Text, With<HudLevelText>>,
) {
    let level = match player_query.get_single() {
        Ok(p) => p.level,
        Err(_) => return,
    };
    for mut text in text_query.iter_mut() {
        text.0 = format!("Lv. {}", level);
    }
}

pub fn update_gold_text(
    progression: Res<RunProgression>,
    mut text_query: Query<&mut Text, With<HudGoldText>>,
) {
    for mut text in text_query.iter_mut() {
        text.0 = format!("Gold: {}", progression.gold_collected);
    }
}

pub fn update_dash_text(
    player_query: Query<&DashCooldown, With<Player>>,
    mut text_query: Query<&mut Text, With<HudDashText>>,
) {
    let dash = match player_query.get_single() {
        Ok(d) => d,
        Err(_) => return,
    };
    for mut text in text_query.iter_mut() {
        if dash.active {
            text.0 = "Dash: dodging".to_string();
        } else if dash.timer > 0.0 {
            text.0 = format!("Dash: {:.1}s", dash.timer);
        } else {
            text.0 = "Dash: ready".to_string();
        }
    }
}

pub fn update_zone_text(
    zone: Res<DungeonState>,
    current_zone: Res<ir_world::map::CurrentZone>,
    mut text_query: Query<&mut Text, With<HudZoneText>>,
) {
    for mut text in text_query.iter_mut() {
        if let Some(ref zone_id) = current_zone.0 {
            text.0 = zone_id.display_name().to_string();
        } else if let Some(ref instance) = zone.current {
            text.0 = format!("Dungeon: {}", instance.name);
        } else {
            text.0 = String::new();
        }
    }
}

pub fn update_prompt_text(
    player_query: Query<&Transform, With<Player>>,
    entrances: Query<(&ir_world::zone::DungeonEntrance, &Transform)>,
    mut text_query: Query<&mut Text, With<HudPromptText>>,
) {
    let player_pos = match player_query.get_single() {
        Ok(t) => t.translation,
        Err(_) => return,
    };
    let mut msg = String::new();
    for (entrance, entrance_tf) in entrances.iter() {
        if player_pos.distance(entrance_tf.translation) < 2.5 {
            msg = format!("[F] Enter: {}", entrance.name);
            break;
        }
    }
    for mut text in text_query.iter_mut() {
        text.0 = msg.clone();
    }
}

/// Updates stamina bar width from player's Stamina component.
pub fn update_stamina_bar(
    player_query: Query<&Stamina, With<Player>>,
    mut bar_query: Query<&mut Node, With<HudStaminaBar>>,
    mut text_query: Query<&mut Text, With<HudStaminaText>>,
) {
    let stamina = match player_query.get_single() {
        Ok(s) => s,
        Err(_) => return,
    };
    let pct = stamina.fraction();
    for mut node in bar_query.iter_mut() {
        node.width = Val::Percent(pct * 100.0);
    }
    for mut text in text_query.iter_mut() {
        text.0 = format!("{:.0}/{:.0}", stamina.current, stamina.max);
    }
}
