//! Tween system — advances all active Tween components each frame.
//!
//! Also provides convenience helpers for spawning common tweens.

use bevy::prelude::*;
use ir_core::tween::*;

// ============================================================================
// Core System
// ============================================================================

/// Advances all active `Tween` components by `Time::delta_secs()`.
///
/// Handles each `TweenMode` variant:
/// - `Scale` — sets `Transform::scale`
/// - `Fade` — sets `BackgroundColor` alpha
/// - `Translate` — sets `Transform::translation`
/// - `Color` — interpolates `BackgroundColor`
/// - `Float` — value is read via `Tween::current_float_value()`
///
/// When a non-repeating tween finishes with `Fade { to: 0.0 }`, the entity
/// is despawned automatically (fade-out-complete → despawn).
pub fn advance_tweens(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut Tween,
        Option<&mut Transform>,
        Option<&mut BackgroundColor>,
    )>,
) {
    let dt = time.delta_secs();
    for (entity, mut tween, transform, bg_color) in query.iter_mut() {
        if !tween.playing {
            continue;
        }

        tween.timer += dt;
        let progress = (tween.timer / tween.duration).min(1.0);
        let eased = (tween.easing)(progress);

        match &tween.mode {
            TweenMode::Scale { from, to } => {
                if let Some(mut tf) = transform {
                    let scale = from + (to - from) * eased;
                    tf.scale = Vec3::splat(scale);
                }
            }
            TweenMode::Fade { from, to } => {
                if let Some(mut bg) = bg_color {
                    let alpha = from + (to - from) * eased;
                    let c = bg.0.to_srgba();
                    bg.0 = Color::srgba(c.red, c.green, c.blue, alpha);
                }
            }
            TweenMode::Translate { from, to } => {
                if let Some(mut tf) = transform {
                    tf.translation = *from + (*to - *from) * eased;
                }
            }
            TweenMode::Color { from, to } => {
                if let Some(mut bg) = bg_color {
                    let f = from.to_srgba();
                    let t = to.to_srgba();
                    bg.0 = Color::srgba(
                        f.red + (t.red - f.red) * eased,
                        f.green + (t.green - f.green) * eased,
                        f.blue + (t.blue - f.blue) * eased,
                        f.alpha + (t.alpha - f.alpha) * eased,
                    );
                }
            }
            TweenMode::Float { .. } => {
                // Value accessible via Tween::current_float_value()
            }
        }

        if progress >= 1.0 {
            if tween.repeat {
                tween.timer = 0.0;
            } else {
                tween.playing = false;
                // Auto-despawn if a fade-out reached alpha 0
                if matches!(&tween.mode, TweenMode::Fade { to, .. } if *to == 0.0) {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}

// ============================================================================
// Convenience Helpers
// ============================================================================

/// Spawn a fade-out tween on `entity` over `duration` seconds.
/// The entity will be despawned when the tween completes (alpha reaches 0).
pub fn fade_out_and_despawn(commands: &mut Commands, entity: Entity, duration: f32) {
    commands.entity(entity).insert(Tween {
        timer: 0.0,
        duration,
        easing: easing::ease_in,
        mode: TweenMode::Fade {
            from: 1.0,
            to: 0.0,
        },
        repeat: false,
        playing: true,
    });
}

/// Add a scale tween to `entity` from `from` to `to` over `duration` seconds.
pub fn scale_tween(
    commands: &mut Commands,
    entity: Entity,
    from: f32,
    to: f32,
    duration: f32,
    easing_fn: EasingFn,
) {
    commands.entity(entity).insert(Tween::scale(from, to, duration, easing_fn));
}

/// Add a pulsing scale loop tween to `entity` (repeating ease-in-out).
pub fn pulse_tween(
    commands: &mut Commands,
    entity: Entity,
    from: f32,
    to: f32,
    duration: f32,
) {
    commands.entity(entity).insert(Tween::pulse_scale(from, to, duration));
}
