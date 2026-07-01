//! HUD update systems – keeps all UI elements in sync with game state.
//!
//! Each function queries player/enemy state and updates the corresponding
//! UI node dimensions, text content, colors, and animations.
//!
//! Key features:
//! - Health bar smooth lerp (HudHealthDisplay tracks displayed width)
//! - Resource bar with class-colored fill
//! - Ability bar with cooldown sweep overlay
//! - Target frame with elite border
use bevy::prelude::*;
use ir_core::*;
use crate::hud::components::*;

const HP_LERP_SPEED: f32 = 8.0;
const RESOURCE_LERP_SPEED: f32 = 10.0;
const TARGET_HP_LERP_SPEED: f32 = 6.0;

// ═══════════════════════════════════════════════════════════════════════════
// Player Frame Updates
// ═══════════════════════════════════════════════════════════════════════════

/// Updates health bar width with smooth lerp, HP text, and player level text.
pub fn update_player_health(
    player_query: Query<(&Health, &Player), With<Player>>,
    mut hp_fill_query: Query<(&mut Node, &mut HudHealthDisplay), With<HudHealthBar>>,
    mut hp_text_query: Query<&mut Text, With<HudHealthBarText>>,
    mut level_text_query: Query<&mut Text, With<HudPlayerLevelText>>,
    mut name_text_query: Query<&mut Text, With<HudPlayerNameText>>,
    name_query: Query<&PlayerName, With<Player>>,
    time: Res<Time>,
) {
    let Ok((health, player)) = player_query.get_single() else { return };
    let target_pct = health.fraction();
    let dt = time.delta_secs();

    for (mut node, mut display) in hp_fill_query.iter_mut() {
        // Smooth lerp toward target
        display.display_pct += (target_pct - display.display_pct) * HP_LERP_SPEED * dt;
        display.display_pct = display.display_pct.clamp(0.0, 1.0);
        node.width = Val::Percent(display.display_pct * 100.0);
    }
    for mut text in hp_text_query.iter_mut() {
        text.0 = format!("{:.0}/{:.0}", health.current, health.max);
    }
    for mut text in level_text_query.iter_mut() {
        text.0 = format!("Lv. {}", player.level);
    }
    // Player name
    if let Ok(name) = name_query.get_single() {
        for mut text in name_text_query.iter_mut() {
            text.0 = name.0.clone();
        }
    }
}

/// Updates the class resource bar (Rage/Energy/Mana/Focus/Holy Power) with lerp.
pub fn update_resource_bar(
    player_query: Query<(&PlayerClass, Option<&ClassResource>), With<Player>>,
    mut res_fill_query: Query<(&mut Node, &mut HudResourceDisplay), With<HudResourceBarFill>>,
    mut res_text_query: Query<&mut Text, With<HudResourceBarText>>,
    mut res_border_query: Query<&mut BorderColor, With<HudResourceBar>>,
    mut res_color_query: Query<&mut BackgroundColor, With<HudResourceBarFill>>,
    time: Res<Time>,
) {
    let Ok((class, resource_opt)) = player_query.get_single() else { return };
    let dt = time.delta_secs();

    let (resource_pct, current, max) = if let Some(res) = resource_opt {
        (res.fraction(), res.current as u32, res.max as u32)
    } else {
        (0.8, 80, 100) // placeholder fallback
    };

    let color = resource_bar_color(class.0);
    for (mut node, mut display) in res_fill_query.iter_mut() {
        display.display_pct += (resource_pct - display.display_pct) * RESOURCE_LERP_SPEED * dt;
        display.display_pct = display.display_pct.clamp(0.0, 1.0);
        node.width = Val::Percent(display.display_pct * 100.0);
    }
    // Set resource fill color to class-specific color
    for mut bc in res_color_query.iter_mut() {
        bc.0 = color;
    }
    for mut text in res_text_query.iter_mut() {
        text.0 = format!(
            "{} {}/{}",
            class.0.resource_name(),
            current,
            max,
        );
    }
    for mut border in res_border_query.iter_mut() {
        border.0 = color;
    }
}

/// Updates the stamina bar below the resource bar.
pub fn update_stamina_bar(
    player_query: Query<&Stamina, With<Player>>,
    mut stam_fill_query: Query<&mut Node, With<HudStaminaBarFill>>,
    mut stam_text_query: Query<&mut Text, With<HudStaminaBarText>>,
) {
    let Ok(stamina) = player_query.get_single() else { return };
    let pct = stamina.fraction();

    for mut node in stam_fill_query.iter_mut() {
        node.width = Val::Percent(pct * 100.0);
    }
    for mut text in stam_text_query.iter_mut() {
        text.0 = format!("{:.0}/{:.0}", stamina.current, stamina.max);
    }
}

/// Updates the player frame border and portrait to match class.
pub fn update_player_frame_class(
    player_query: Query<&PlayerClass, With<Player>>,
    mut portrait_query: Query<&mut BackgroundColor, With<HudPlayerPortrait>>,
    mut frame_border_query: Query<&mut BorderColor, (With<HudPlayerFrame>, Without<HudPlayerPortrait>)>,
) {
    let Ok(class) = player_query.get_single() else { return };
    let primary = class_primary_color(class.0);
    let border = class_border_glow(class.0);

    for mut bc in portrait_query.iter_mut() {
        bc.0 = primary;
    }
    for mut bc in frame_border_query.iter_mut() {
        bc.0 = border;
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Ability Bar Updates
// ═══════════════════════════════════════════════════════════════════════════

/// Updates the ability bar to show the correct abilities for the player's class
/// and drives cooldown overlays.
pub fn update_ability_bar(
    player_query: Query<&PlayerClass, With<Player>>,
    mut slot_icon_query: Query<(&mut BackgroundColor, &mut HudActionBarIcon)>,
    mut slot_border_query: Query<&mut BorderColor, With<HudActionBarSlot>>,
    mut cooldown_query: Query<(&mut Node, &mut HudCooldownOverlay)>,
    time: Res<Time>,
) {
    let Ok(class) = player_query.get_single() else { return };
    let color = class_primary_color(class.0);
    let dt = time.delta_secs();

    // Update cooldowns — each frame tick down and update overlay height
    for (mut node, mut cd) in cooldown_query.iter_mut() {
        if cd.remaining > 0.0 {
            cd.remaining = (cd.remaining - dt).max(0.0);
            let pct = cd.remaining / cd.max;
            node.height = Val::Percent(pct * 100.0);
        } else {
            node.height = Val::Px(0.0);
        }
    }

    // Update icon colors and border to class color
    for (mut bc, _icon) in slot_icon_query.iter_mut() {
        bc.0 = color;
    }
    for mut border in slot_border_query.iter_mut() {
        border.0 = color;
    }
    // Note: ability names are set at spawn time from class_abilities;
    // if the player changes class mid-game we'd need to update Text here too.
}

// ═══════════════════════════════════════════════════════════════════════════
// Target Frame Updates
// ═══════════════════════════════════════════════════════════════════════════

/// Finds the nearest enemy to the player and shows/hides the target frame.
/// Also applies elite/boss border styling and smooth health lerp.
pub fn update_target_frame(
    mut target_frame_query: Query<&mut Node, With<HudTargetFrame>>,
    mut target_name_query: Query<&mut Text, With<HudTargetNameText>>,
    mut target_level_query: Query<&mut Text, With<HudTargetLevelText>>,
    mut target_hp_fill_query: Query<(&mut Node, &mut HudTargetHealthDisplay), With<HudTargetHealthBarFill>>,
    mut target_hp_pct_query: Query<&mut Text, With<HudTargetHealthPctText>>,
    mut target_border_query: Query<&mut BorderColor, (With<HudTargetFrame>, Without<HudTargetHealthBar>)>,
    mut target_elite_query: Query<&mut Node, With<HudTargetEliteBorder>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    enemy_query: Query<(Entity, &Health, &Transform, Option<&Enemy>), Without<Player>>,
    time: Res<Time>,
) {
    let Ok(player_tf) = player_query.get_single() else { return };
    let dt = time.delta_secs();

    // Find nearest enemy
    let mut nearest: Option<(f32, &Health, &Transform, Option<&Enemy>)> = None;
    for (_, health, transform, enemy_opt) in enemy_query.iter() {
        let dist = player_tf.translation.distance(transform.translation);
        if dist < 15.0 {
            let is_closer = nearest.map_or(true, |(d, _, _, _)| dist < d);
            if is_closer {
                nearest = Some((dist, health, transform, enemy_opt));
            }
        }
    }

    if let Some((_, health, _transform, enemy_opt)) = nearest {
        let target_pct = health.fraction();
        let name = enemy_opt
            .map(|e| format!("{:?}", e.variant))
            .unwrap_or_else(|| "Enemy".to_string());
        let is_elite = enemy_opt.map_or(false, |e| matches!(e.variant, EnemyVariant::Elite | EnemyVariant::Boss));

        // Show frame
        for mut node in target_frame_query.iter_mut() {
            node.display = Display::Flex;
        }

        // Set border: elite/boss get gold/dragon border, normal get red
        for mut border in target_border_query.iter_mut() {
            border.0 = if is_elite {
                Color::srgb(1.0, 0.7, 0.05) // gold dragon border
            } else {
                Color::srgb(0.5, 0.15, 0.15) // default red border
            };
        }

        // Show/hide elite indicator
        for mut node in target_elite_query.iter_mut() {
            if is_elite {
                node.display = Display::Flex;
                node.width = Val::Px(8.0);
                node.height = Val::Px(8.0);
            } else {
                node.display = Display::None;
                node.width = Val::Px(0.0);
                node.height = Val::Px(0.0);
            }
        }

        for mut text in target_name_query.iter_mut() {
            text.0 = name.clone();
        }
        for mut text in target_level_query.iter_mut() {
            text.0 = format!("Lv. {}", enemy_opt.map_or(1, |e| e.tier));
        }
        for (mut node, mut display) in target_hp_fill_query.iter_mut() {
            display.display_pct += (target_pct - display.display_pct) * TARGET_HP_LERP_SPEED * dt;
            display.display_pct = display.display_pct.clamp(0.0, 1.0);
            node.width = Val::Percent(display.display_pct * 100.0);
        }
        for mut text in target_hp_pct_query.iter_mut() {
            text.0 = format!("{}%", (target_pct * 100.0).round() as u32);
        }
    } else {
        // No enemy nearby — hide target frame
        for mut node in target_frame_query.iter_mut() {
            node.display = Display::None;
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// XP Bar Update
// ═══════════════════════════════════════════════════════════════════════════

pub fn update_xp_bar(
    player_query: Query<&Player, With<Player>>,
    mut xp_fill_query: Query<&mut Node, With<HudXpBarFill>>,
    mut xp_text_query: Query<&mut Text, With<HudXpBarText>>,
) {
    let Ok(player) = player_query.get_single() else { return };
    let pct = if player.xp_to_next > 0 {
        player.experience as f32 / player.xp_to_next as f32
    } else {
        0.0
    };

    for mut node in xp_fill_query.iter_mut() {
        node.width = Val::Percent(pct.min(1.0) * 100.0);
    }
    for mut text in xp_text_query.iter_mut() {
        text.0 = format!("XP: {}/{}", player.experience, player.xp_to_next);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Zone / Minimap Tracker
// ═══════════════════════════════════════════════════════════════════════════

pub fn update_zone_tracker(
    time: Res<Time>,
    zone: Res<DungeonState>,
    current_zone: Res<ir_world::map::CurrentZone>,
    mut zone_text_query: Query<(&mut Text, &mut TextColor), With<HudZoneText>>,
) {
    for (mut text, mut color) in zone_text_query.iter_mut() {
        let zone_name = if let Some(ref zone_id) = current_zone.0 {
            zone_id.display_name().to_string()
        } else if let Some(ref instance) = zone.current {
            format!("Dungeon: {}", instance.name)
        } else {
            String::new()
        };
        text.0 = zone_name.clone();

        // Pulsing color effect
        if !zone_name.is_empty() {
            let pulse = (time.elapsed_secs() * 2.0).sin() * 0.25 + 0.75;
            color.0 = Color::srgba(0.5 * pulse, 0.75 * pulse, 0.35 * pulse, 1.0);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Legacy Updates (kept for compatibility)
// ═══════════════════════════════════════════════════════════════════════════

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
    let Ok(dash) = player_query.get_single() else { return };
    for mut text in text_query.iter_mut() {
        text.0 = if dash.active {
            "Dash: dodging".to_string()
        } else if dash.timer > 0.0 {
            format!("Dash: {:.1}s", dash.timer)
        } else {
            "Dash: ready".to_string()
        };
    }
}

pub fn update_prompt_text(
    player_query: Query<&Transform, With<Player>>,
    entrances: Query<(&ir_world::zone::DungeonEntrance, &Transform)>,
    mut text_query: Query<&mut Text, With<HudPromptText>>,
) {
    let Ok(player_pos) = player_query.get_single() else { return };
    let mut msg = String::new();
    for (entrance, entrance_tf) in entrances.iter() {
        if player_pos.translation.distance(entrance_tf.translation) < 2.5 {
            msg = format!("[F] Enter: {}", entrance.name);
            break;
        }
    }
    for mut text in text_query.iter_mut() {
        text.0 = msg.clone();
    }
}
