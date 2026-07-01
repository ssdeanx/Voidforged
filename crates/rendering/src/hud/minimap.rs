//! Minimap — small square panel in the top-right corner showing zone regions,
//! player position, dungeon entrances, and nearby enemies.
//!
//! Uses Bevy UI with a fixed-size Node. Each dot is a child entity positioned
//! via percentage-based margins within the minimap container.

use bevy::prelude::*;
use ir_core::*;
use ir_world::zone::{self, ZoneDef};
use crate::hud::components::*;

const MINIMAP_SIZE: f32 = 160.0;
const MINIMAP_WORLD_RADIUS: f32 = 35.0; // world units shown from center
const MINIMAP_SCALE: f32 = MINIMAP_SIZE / (MINIMAP_WORLD_RADIUS * 2.0); // px per world unit

/// Helper to make a simple text label.
fn label(s: &str, size: f32, color: Color) -> impl Bundle {
    (
        Text::new(s.to_string()),
        TextFont { font_size: size, ..default() },
        TextColor(color),
    )
}

/// Spawns the minimap panel (top-right corner), initially empty of dots.
/// Dots are spawned/despawned each frame by update_minimap.
pub fn spawn_minimap(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Node {
                width: Val::Px(MINIMAP_SIZE + 8.0),
                height: Val::Px(MINIMAP_SIZE + 26.0), // extra for label
                position_type: PositionType::Absolute,
                top: Val::Px(8.0),
                right: Val::Px(8.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            HudMinimap,
        ))
        .with_children(|root| {
            // "MAP" label
            root.spawn((
                label("MAP", 10.0, Color::srgb(0.4, 0.5, 0.6)),
                HudMinimap,
            ));
            // Minimap container — square where dots are drawn
            root.spawn((
                Node {
                    width: Val::Px(MINIMAP_SIZE),
                    height: Val::Px(MINIMAP_SIZE),
                    border: UiRect::all(Val::Px(1.0)),
                    overflow: Overflow::clip(),
                    ..default()
                },
                BorderColor(Color::srgb(0.3, 0.3, 0.4)),
                BackgroundColor(Color::srgba(0.05, 0.05, 0.1, 0.85)),
                HudMinimapContainer,
            ));
        });
}

/// World-to-minimap coordinate conversion.
/// Returns a percentage (0.0–100.0) for the left/top offset within the container.
fn world_to_minimap(world_pos: Vec3, player_pos: Vec3) -> (f32, f32) {
    let dx = world_pos.x - player_pos.x;
    let dz = world_pos.z - player_pos.z;
    // Clamp to visible region
    let clamped_x = dx.clamp(-MINIMAP_WORLD_RADIUS, MINIMAP_WORLD_RADIUS);
    let clamped_z = dz.clamp(-MINIMAP_WORLD_RADIUS, MINIMAP_WORLD_RADIUS);
    // Convert to pixel offset from top-left
    let px = (clamped_x + MINIMAP_WORLD_RADIUS) * MINIMAP_SCALE;
    let py = (clamped_z + MINIMAP_WORLD_RADIUS) * MINIMAP_SCALE;
    // As percentage (0–100)
    let pct_x = (px / MINIMAP_SIZE) * 100.0;
    let pct_y = (py / MINIMAP_SIZE) * 100.0;
    (pct_x, pct_y)
}

/// Spawns or updates zone-colored region overlays on the minimap.
/// Each zone within visible range gets a semi-transparent colored rect.
fn spawn_zone_regions(
    commands: &mut Commands,
    container: Entity,
    player_pos: Vec3,
    zones: &[ZoneDef],
) {
    let _player_zone_x = (player_pos.x / 2.0).floor() as i32;
    let _player_zone_z = (player_pos.z / 2.0).floor() as i32;

    for zone in zones {
        let zx_min = zone.offset_x * 2;
        let zx_max = zx_min + zone.tile_w as i32 * 2;
        let zz_min = zone.offset_z * 2;
        let zz_max = zz_min + zone.tile_h as i32 * 2;

        // Check if zone is within visible minimap range
        let zone_cx = (zx_min + zx_max) as f32 / 2.0;
        let zone_cz = (zz_min + zz_max) as f32 / 2.0;
        let dist = Vec2::new(zone_cx, zone_cz).distance(Vec2::new(player_pos.x, player_pos.z));
        if dist > MINIMAP_WORLD_RADIUS * 2.0 + (zone.tile_w.max(zone.tile_h) as f32 * 2.0) {
            continue;
        }

        // World corners of the zone
        let corners = [
            Vec3::new(zx_min as f32, 0.0, zz_min as f32),
            Vec3::new(zx_max as f32, 0.0, zz_min as f32),
            Vec3::new(zx_max as f32, 0.0, zz_max as f32),
            Vec3::new(zx_min as f32, 0.0, zz_max as f32),
        ];

        // Convert to minimap coords
        let mm_corners: Vec<(f32, f32)> = corners
            .iter()
            .map(|&c| world_to_minimap(c, player_pos))
            .collect();

        let min_pct_x = mm_corners.iter().map(|&(x, _)| x).fold(f32::MAX, f32::min);
        let max_pct_x = mm_corners.iter().map(|&(x, _)| x).fold(f32::MIN, f32::max);
        let min_pct_y = mm_corners.iter().map(|&(_, y)| y).fold(f32::MAX, f32::min);
        let max_pct_y = mm_corners.iter().map(|&(_, y)| y).fold(f32::MIN, f32::max);

        // Only spawn if partially visible
        if max_pct_x <= 0.0 || max_pct_y <= 0.0 || min_pct_x >= 100.0 || min_pct_y >= 100.0 {
            continue;
        }

        let color = zone.id.ground_color();
        commands.entity(container).with_children(|c| {
            c.spawn((
                Node {
                    width: Val::Px(((max_pct_x - min_pct_x) / 100.0 * MINIMAP_SIZE).max(1.0)),
                    height: Val::Px(((max_pct_y - min_pct_y) / 100.0 * MINIMAP_SIZE).max(1.0)),
                    position_type: PositionType::Absolute,
                    left: Val::Percent(min_pct_x),
                    top: Val::Percent(min_pct_y),
                    ..default()
                },
                BackgroundColor(Color::srgba(color.to_srgba().red, color.to_srgba().green, color.to_srgba().blue, 0.35)),
                HudMinimap,
            ));
        });
    }
}

/// Spawns dungeon entrance dots (purple) on the minimap.
fn spawn_entrance_dots(
    commands: &mut Commands,
    container: Entity,
    player_pos: Vec3,
    zones: &[ZoneDef],
) {
    for zone in zones {
        for (ex, ez, _entrance) in &zone.dungeon_entrances {
            let wx = zone.offset_x as f32 * 2.0 + *ex as f32 * 2.0;
            let wz = zone.offset_z as f32 * 2.0 + *ez as f32 * 2.0;
            let dist = Vec2::new(wx, wz).distance(Vec2::new(player_pos.x, player_pos.z));
            if dist > MINIMAP_WORLD_RADIUS * 1.5 {
                continue;
            }
            let (pct_x, pct_y) = world_to_minimap(Vec3::new(wx, 0.0, wz), player_pos);
            if pct_x < 0.0 || pct_x > 100.0 || pct_y < 0.0 || pct_y > 100.0 {
                continue;
            }
            commands.entity(container).with_children(|c| {
                c.spawn((
                    Node {
                        width: Val::Px(6.0),
                        height: Val::Px(6.0),
                        position_type: PositionType::Absolute,
                        left: Val::Percent(pct_x - 1.88), // center the 6px dot
                        top: Val::Percent(pct_y - 1.88),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.7, 0.2, 0.8)),
                    HudMinimap,
                ));
            });
        }
    }
}

/// Spawns enemy dots (red) on the minimap — only enemies within 30 units.
/// Uses the Enemy component + Transform from the ECS.
fn spawn_enemy_dots(
    commands: &mut Commands,
    container: Entity,
    player_pos: Vec3,
    enemy_query: &Query<&Transform, (With<Enemy>, Without<Player>)>,
) {
    for enemy_tf in enemy_query.iter() {
        let dist = enemy_tf.translation.distance(player_pos);
        if dist > 30.0 {
            continue;
        }
        let (pct_x, pct_y) = world_to_minimap(enemy_tf.translation, player_pos);
        if pct_x < 0.0 || pct_x > 100.0 || pct_y < 0.0 || pct_y > 100.0 {
            continue;
        }
        commands.entity(container).with_children(|c| {
            c.spawn((
                Node {
                    width: Val::Px(4.0),
                    height: Val::Px(4.0),
                    position_type: PositionType::Absolute,
                    left: Val::Percent(pct_x - 1.25),
                    top: Val::Percent(pct_y - 1.25),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.9, 0.15, 0.15)),
                HudMinimapEnemyDot,
            ));
        });
    }
}

/// Updates the minimap every frame: clears old dots and re-spawns them.
pub fn update_minimap(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
    container_query: Query<Entity, With<HudMinimapContainer>>,
    minimap_children: Query<Entity, (With<HudMinimap>, Without<HudMinimapContainer>)>,
    dungeon_state: Res<DungeonState>,
) {
    // Don't show minimap in dungeon
    if dungeon_state.current.is_some() {
        // Despawn any existing minimap children (they're cleaned up with HUD anyway)
        return;
    }

    let Ok(player_tf) = player_query.get_single() else { return };
    let player_pos = player_tf.translation;

    let Ok(container) = container_query.get_single() else { return };

    // Despawn all non-container minimap children (the zone regions and dots from last frame)
    for entity in minimap_children.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Get zone definitions
    let zones = zone::all_zones();

    // Spawn zone-colored regions
    spawn_zone_regions(&mut commands, container, player_pos, &zones);

    // Spawn dungeon entrance dots
    spawn_entrance_dots(&mut commands, container, player_pos, &zones);

    // Spawn enemy dots
    spawn_enemy_dots(&mut commands, container, player_pos, &enemy_query);

    // Spawn player dot (always, at center)
    commands.entity(container).with_children(|c| {
        c.spawn((
            Node {
                width: Val::Px(6.0),
                height: Val::Px(6.0),
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0 - 1.88), // center
                top: Val::Percent(50.0 - 1.88),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.8, 1.0)),
            HudMinimapPlayerDot,
        ));
    });
}
