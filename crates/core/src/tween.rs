//! Reusable tween/animation system for UI elements.
//!
//! Provides:
//! - A `Tween` component for per-entity animation
//! - Built-in easing functions (linear, ease_in, ease_out, ease_in_out, bounce, elastic_out)
//! - `TweenMode` for animating scale, fade, translate, color, and custom floats
//!
//! The actual `advance_tweens` system lives in `ir_rendering::tween_system`.

use bevy::prelude::*;

// ============================================================================
// Easing Functions
// ============================================================================

/// Function pointer type for easing curves.
pub type EasingFn = fn(f32) -> f32;

/// Built-in easing functions matching CSS/GSAP conventions.
pub mod easing {
    /// Linear — no easing
    pub fn linear(t: f32) -> f32 {
        t
    }

    /// Quadratic ease-in — starts slow, accelerates
    pub fn ease_in(t: f32) -> f32 {
        t * t
    }

    /// Quadratic ease-out — starts fast, decelerates
    pub fn ease_out(t: f32) -> f32 {
        1.0 - (1.0 - t) * (1.0 - t)
    }

    /// Quadratic ease-in-out — smooth acceleration and deceleration
    pub fn ease_in_out(t: f32) -> f32 {
        if t < 0.5 {
            2.0 * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
        }
    }

    /// Bounce — overshoots and bounces back at the end
    pub fn bounce(t: f32) -> f32 {
        let n1 = 7.5625;
        let d1 = 2.75;
        if t < 1.0 / d1 {
            n1 * t * t
        } else if t < 2.0 / d1 {
            let t = t - 1.5 / d1;
            n1 * t * t + 0.75
        } else if t < 2.5 / d1 {
            let t = t - 2.25 / d1;
            n1 * t * t + 0.9375
        } else {
            let t = t - 2.625 / d1;
            n1 * t * t + 0.984375
        }
    }

    /// Elastic-out — overshoots with a springy oscillation
    pub fn elastic_out(t: f32) -> f32 {
        if t == 0.0 || t == 1.0 {
            t
        } else {
            (t * 10.0 * std::f32::consts::TAU * 0.25).sin() * (2.0_f32).powf(-10.0 * t) + 1.0
        }
    }
}

// ============================================================================
// TweenMode — What to animate
// ============================================================================

/// Which property the tween animates.
#[derive(Debug, Clone)]
pub enum TweenMode {
    /// Scale from `from` to `to` (multiplier applied to Transform.scale).
    Scale { from: f32, to: f32 },
    /// Fade alpha of BackgroundColor from `from` to `to`.
    Fade { from: f32, to: f32 },
    /// Translate from `from` to `to` in local space (Transform.translation).
    Translate { from: Vec3, to: Vec3 },
    /// Interpolate BackgroundColor from `from` to `to`.
    Color { from: Color, to: Color },
    /// Custom float value — read via `Tween::current_float_value()`.
    Float { from: f32, to: f32 },
}

// ============================================================================
// Tween Component
// ============================================================================

/// A tween animation on a UI element.
///
/// Attach this to any entity to animate its transform, color, or opacity.
/// The `advance_tweens` system in `ir_rendering::tween_system` advances all
/// active tweens each frame.
#[derive(Component, Debug, Clone)]
pub struct Tween {
    /// Elapsed time since the tween started (seconds).
    pub timer: f32,
    /// Total duration of one animation cycle (seconds).
    pub duration: f32,
    /// Easing function (see `easing` module).
    pub easing: EasingFn,
    /// What to animate.
    pub mode: TweenMode,
    /// If true, loop forever.
    pub repeat: bool,
    /// If false, the tween is paused / finished.
    pub playing: bool,
}

impl Tween {
    /// Create a new tween with default ease-in-out easing.
    pub fn new(mode: TweenMode, duration: f32) -> Self {
        Self {
            timer: 0.0,
            duration,
            easing: easing::ease_in_out,
            mode,
            repeat: false,
            playing: true,
        }
    }

    /// Convenience: scale tween with a specific easing.
    pub fn scale(from: f32, to: f32, duration: f32, easing: EasingFn) -> Self {
        Self {
            timer: 0.0,
            duration,
            easing,
            mode: TweenMode::Scale { from, to },
            repeat: false,
            playing: true,
        }
    }

    /// Convenience: pulsing scale loop (ease-in-out, repeats).
    pub fn pulse_scale(from: f32, to: f32, duration: f32) -> Self {
        Self {
            timer: 0.0,
            duration,
            easing: easing::ease_in_out,
            mode: TweenMode::Scale { from, to },
            repeat: true,
            playing: true,
        }
    }

    /// Convenience: fade tween.
    pub fn fade(from: f32, to: f32, duration: f32, easing: EasingFn) -> Self {
        Self {
            timer: 0.0,
            duration,
            easing,
            mode: TweenMode::Fade { from, to },
            repeat: false,
            playing: true,
        }
    }

    /// Read the current interpolated value for `TweenMode::Float`.
    /// Returns 0.0 if the mode is not Float.
    pub fn current_float_value(&self) -> f32 {
        if let TweenMode::Float { from, to } = &self.mode {
            let progress = (self.timer / self.duration).min(1.0);
            let eased = (self.easing)(progress);
            from + (to - from) * eased
        } else {
            0.0
        }
    }

    /// Current animation progress [0.0, 1.0].
    pub fn progress(&self) -> f32 {
        (self.timer / self.duration).min(1.0)
    }
}
