//! Zone transition loading screen overlay.
//!
//! When the player crosses a zone boundary, this shows a full-screen fade-to-black
//! with the zone name displayed:
//!   - 0.5s fade in
//!   - 1.5s hold with zone name text
//!   - 0.5s fade out

use bevy::prelude::*;
use ir_world::map::{ZoneTransitionState, TransitionPhase};

/// Marker component for the zone transition overlay entity.
#[derive(Component)]
pub struct ZoneTransitionOverlay;

/// Marker for the zone transition text entity.
#[derive(Component)]
pub struct ZoneTransitionText;

/// Spawns the zone transition overlay UI when a transition begins.
pub fn spawn_zone_transition_overlay(
    mut commands: Commands,
    transition: Res<ZoneTransitionState>,
    overlay_query: Query<Entity, With<ZoneTransitionOverlay>>,
) {
    if !transition.active {
        return;
    }
    if !overlay_query.is_empty() {
        return; // Already spawned
    }

    // Full-screen dark overlay
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
            ZoneTransitionOverlay,
            Visibility::Visible,
        ))
        .with_children(|parent| {
            // Zone display name
            parent.spawn((
                Text::new(transition.zone_label.clone()),
                TextFont {
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(JustifyText::Center),
                ZoneTransitionText,
            ));
            // Zone subtitle
            parent.spawn((
                Text::new(transition.zone_name.clone()),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgba(0.7, 0.7, 0.7, 1.0)),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
        });
}

/// Updates the zone transition overlay alpha and despawns when complete.
pub fn update_zone_transition(
    mut commands: Commands,
    time: Res<Time>,
    mut transition: ResMut<ZoneTransitionState>,
    mut overlay_query: Query<Entity, With<ZoneTransitionOverlay>>,
    mut bg_query: Query<&mut BackgroundColor, With<ZoneTransitionOverlay>>,
    mut text_query: Query<&mut TextColor, With<ZoneTransitionText>>,
) {
    let overlay_entity = match overlay_query.get_single_mut() {
        Ok(e) => e,
        Err(_) => return,
    };

    if !transition.active {
        // Clean up any leftover overlay
        commands.entity(overlay_entity).despawn_recursive();
        return;
    }

    transition.timer += time.delta_secs();
    let alpha: f32;

    match transition.phase {
        TransitionPhase::FadeIn => {
            // 0.5s: alpha goes 0 → 1
            let t = (transition.timer / 0.5).min(1.0);
            alpha = t;
            if t >= 1.0 {
                transition.phase = TransitionPhase::Hold;
                transition.timer = 0.0;
            }
        }
        TransitionPhase::Hold => {
            // 1.5s: alpha stays at 1
            alpha = 1.0;
            if transition.timer >= 1.5 {
                transition.phase = TransitionPhase::FadeOut;
                transition.timer = 0.0;
            }
        }
        TransitionPhase::FadeOut => {
            // 0.5s: alpha goes 1 → 0
            let t = (transition.timer / 0.5).min(1.0);
            alpha = 1.0 - t;
            if t >= 1.0 {
                transition.active = false;
                transition.phase = TransitionPhase::None;
                commands.entity(overlay_entity).despawn_recursive();
                return;
            }
        }
        TransitionPhase::None => {
            // Not in a transition — should not reach here, but safety cleanup
            commands.entity(overlay_entity).despawn_recursive();
            return;
        }
    }

    // Update background alpha
    if let Ok(mut bg) = bg_query.get_single_mut() {
        bg.0 = Color::srgba(0.0, 0.0, 0.0, alpha);
    }
    // Update text alpha
    for mut tc in text_query.iter_mut() {
        tc.0 = Color::srgba(1.0, 1.0, 1.0, alpha);
    }
}

/// Cleans up the zone transition overlay when leaving the world state.
pub fn despawn_zone_transition(
    mut commands: Commands,
    overlay_query: Query<Entity, With<ZoneTransitionOverlay>>,
) {
    for entity in overlay_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
