//! GPU particle effects and custom shader materials for professional VFX.
//!
//! Includes impact bursts, glow effects, dash trails, telegraph pulse,
//! custom GlowMaterial for time-based pulsing emissive effects,
//! and main menu ambient background particles.

use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy_hanabi::*;
use crate::hud::components::MenuBackgroundParticles;

// ============================================================================
// Effects Library — pre-built effect handles stored as a resource
// ============================================================================

/// Resource holding handles to all pre-built particle effects.
#[derive(Resource)]
pub struct EffectsLibrary {
    /// Default impact burst effect for melee/projectile hits.
    pub impact_burst: Handle<EffectAsset>,
    /// Critical hit impact — bigger, more particles.
    pub impact_critical: Handle<EffectAsset>,
    /// Death explosion effect for enemy deaths.
    pub death_explosion: Handle<EffectAsset>,
    /// Small spark effect for projectile trails.
    pub projectile_trail_spark: Handle<EffectAsset>,
    /// Green glow effect (used for XP gems, friendly auras).
    pub glow_green: Handle<EffectAsset>,
    /// Blue glow effect (used for mana effects, magic cast).
    pub glow_blue: Handle<EffectAsset>,
    /// Dash / dodge trail effect.
    pub dash_trail: Handle<EffectAsset>,
    /// Pulsing telegraph ring for enemy windup attacks.
    pub telegraph_pulse: Handle<EffectAsset>,
    /// Slow ambient particles behind the main menu.
    pub menu_background: Handle<EffectAsset>,
}

/// Builds all effect assets and inserts them as a resource.
pub fn build_effects_library(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let impact_burst = build_impact_burst(&mut effects, 16, Vec3::splat(0.3));
    let impact_critical = build_impact_burst(&mut effects, 48, Vec3::splat(0.6));
    let death_explosion = build_death_explosion(&mut effects);
    let projectile_trail_spark = build_trail_spark(&mut effects);
    let glow_green = build_glow(&mut effects, Vec4::new(0.0, 1.0, 0.3, 1.0));
    let glow_blue = build_glow(&mut effects, Vec4::new(0.3, 0.6, 1.0, 1.0));
    let dash_trail = build_dash_trail(&mut effects);
    let telegraph_pulse = build_telegraph_pulse(&mut effects);
    let menu_background = build_menu_background_particles(&mut effects);
    commands.insert_resource(EffectsLibrary {
        impact_burst,
        impact_critical,
        death_explosion,
        projectile_trail_spark,
        glow_green,
        glow_blue,
        dash_trail,
        telegraph_pulse,
        menu_background,
    });
}

fn build_impact_burst(
    effects: &mut Assets<EffectAsset>,
    particle_count: u32,
    size: Vec3,
) -> Handle<EffectAsset> {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1.0, 0.8, 0.3, 1.0));
    gradient.add_key(0.5, Vec4::new(1.0, 0.4, 0.0, 0.5));
    gradient.add_key(1.0, Vec4::new(0.5, 0.0, 0.0, 0.0));

    effects.add(
        EffectAsset::new(
            64,
            SpawnerSettings::once((particle_count as f32).into()),
            Module::default(),
        )
        .with_name("impact_burst")
        .render(ColorOverLifetimeModifier {
            gradient,
            blend: ColorBlendMode::Overwrite,
            mask: ColorBlendMask::all(),
        })
        .render(SetSizeModifier {
            size: CpuValue::Single(size),
        }),
    )
}

/// Death explosion — large red/orange burst with many particles that spread outward.
fn build_death_explosion(effects: &mut Assets<EffectAsset>) -> Handle<EffectAsset> {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1.0, 0.6, 0.1, 1.0));
    gradient.add_key(0.3, Vec4::new(1.0, 0.2, 0.0, 0.8));
    gradient.add_key(0.7, Vec4::new(0.8, 0.0, 0.0, 0.3));
    gradient.add_key(1.0, Vec4::new(0.2, 0.0, 0.0, 0.0));

    effects.add(
        EffectAsset::new(
            128,
            SpawnerSettings::once(48.0.into()),
            Module::default(),
        )
        .with_name("death_explosion")
        .render(ColorOverLifetimeModifier {
            gradient,
            blend: ColorBlendMode::Overwrite,
            mask: ColorBlendMask::all(),
        })
        .render(SetSizeModifier {
            size: CpuValue::Single(Vec3::splat(0.5)),
        }),
    )
}

/// Small spark for projectile trails.
fn build_trail_spark(effects: &mut Assets<EffectAsset>) -> Handle<EffectAsset> {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.3, 0.8, 1.0, 0.6));
    gradient.add_key(0.5, Vec4::new(0.1, 0.4, 0.8, 0.2));
    gradient.add_key(1.0, Vec4::new(0.0, 0.1, 0.3, 0.0));

    effects.add(
        EffectAsset::new(
            16,
            SpawnerSettings::once(4.0.into()),
            Module::default(),
        )
        .with_name("projectile_trail_spark")
        .render(ColorOverLifetimeModifier {
            gradient,
            blend: ColorBlendMode::Overwrite,
            mask: ColorBlendMask::all(),
        })
        .render(SetSizeModifier {
            size: CpuValue::Single(Vec3::splat(0.12)),
        }),
    )
}

fn build_glow(
    effects: &mut Assets<EffectAsset>,
    color: Vec4,
) -> Handle<EffectAsset> {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, color);
    gradient.add_key(0.5, Vec4::new(
        color.x * 0.3,
        color.y * 0.3,
        color.z * 0.3,
        0.2,
    ));
    gradient.add_key(1.0, color);

    effects.add(
        EffectAsset::new(
            32,
            SpawnerSettings::rate(20.0.into()),
            Module::default(),
        )
        .with_name("glow")
        .render(ColorOverLifetimeModifier {
            gradient,
            blend: ColorBlendMode::Overwrite,
            mask: ColorBlendMask::all(),
        })
        .render(SetSizeModifier {
            size: CpuValue::Single(Vec3::splat(0.4)),
        }),
    )
}

fn build_dash_trail(effects: &mut Assets<EffectAsset>) -> Handle<EffectAsset> {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.5, 0.8, 1.0, 0.6));
    gradient.add_key(1.0, Vec4::new(0.3, 0.5, 0.8, 0.0));

    effects.add(
        EffectAsset::new(
            32,
            SpawnerSettings::rate(30.0.into()),
            Module::default(),
        )
        .with_name("dash_trail")
        .render(ColorOverLifetimeModifier {
            gradient,
            blend: ColorBlendMode::Overwrite,
            mask: ColorBlendMask::all(),
        })
        .render(SetSizeModifier {
            size: CpuValue::Single(Vec3::splat(0.25)),
        }),
    )
}

/// Pulsing red/orange telegraph ring for enemy windup attacks.
fn build_telegraph_pulse(effects: &mut Assets<EffectAsset>) -> Handle<EffectAsset> {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1.0, 0.2, 0.05, 0.8));
    gradient.add_key(0.5, Vec4::new(1.0, 0.6, 0.1, 0.3));
    gradient.add_key(1.0, Vec4::new(0.8, 0.0, 0.0, 0.0));

    effects.add(
        EffectAsset::new(48, SpawnerSettings::once(20.0.into()), Module::default())
            .with_name("telegraph_pulse")
            .render(ColorOverLifetimeModifier {
                gradient,
                blend: ColorBlendMode::Overwrite,
                mask: ColorBlendMask::all(),
            })
            .render(SetSizeModifier {
                size: CpuValue::Single(Vec3::splat(0.6)),
            }),
    )
}

// ============================================================================
// Inline particle spawners (use EffectLibrary handles)
// ============================================================================

/// Spawns a default impact burst at a position.
pub fn spawn_impact(commands: &mut Commands, library: &EffectsLibrary, position: Vec3) {
    commands.spawn((
        ParticleEffect::new(library.impact_burst.clone()),
        Transform::from_translation(position),
        GlobalTransform::default(),
        ir_core::Lifetime { remaining: 0.5 },
    ));
}

/// Spawns a larger critical impact burst at a position.
pub fn spawn_impact_critical(commands: &mut Commands, library: &EffectsLibrary, position: Vec3) {
    commands.spawn((
        ParticleEffect::new(library.impact_critical.clone()),
        Transform::from_translation(position),
        GlobalTransform::default(),
        ir_core::Lifetime { remaining: 0.8 },
    ));
}

/// Spawns a death explosion effect at a position.
pub fn spawn_death_explosion(commands: &mut Commands, library: &EffectsLibrary, position: Vec3) {
    commands.spawn((
        ParticleEffect::new(library.death_explosion.clone()),
        Transform::from_translation(position),
        GlobalTransform::default(),
        ir_core::Lifetime { remaining: 1.0 },
    ));
}

/// Spawns a small trail spark at a position.
pub fn spawn_trail_spark(commands: &mut Commands, library: &EffectsLibrary, position: Vec3) {
    commands.spawn((
        ParticleEffect::new(library.projectile_trail_spark.clone()),
        Transform::from_translation(position),
        GlobalTransform::default(),
        ir_core::Lifetime { remaining: 0.3 },
    ));
}

/// Spawns a glow effect at a position.
pub fn spawn_glow(
    commands: &mut Commands,
    library: &EffectsLibrary,
    position: Vec3,
) -> Entity {
    commands
        .spawn((
            ParticleEffect::new(library.glow_green.clone()),
            Transform::from_translation(position),
            GlobalTransform::default(),
        ))
        .id()
}

/// Spawns a dash trail at a position.
pub fn spawn_dash_trail(commands: &mut Commands, library: &EffectsLibrary, position: Vec3) {
    commands.spawn((
        ParticleEffect::new(library.dash_trail.clone()),
        Transform::from_translation(position),
        GlobalTransform::default(),
        ir_core::Lifetime { remaining: 0.3 },
    ));
}

/// Spawns a telegraph pulse effect (enemy windup indicator).
pub fn spawn_telegraph(commands: &mut Commands, library: &EffectsLibrary, position: Vec3) {
    commands.spawn((
        ParticleEffect::new(library.telegraph_pulse.clone()),
        Transform::from_translation(position),
        GlobalTransform::default(),
        ir_core::Lifetime { remaining: 1.2 },
    ));
}

// ============================================================================
// Main Menu Background Particles
// ============================================================================

/// Builds a slow ambient particle effect for the main menu background.
///
/// Creates particles with:
/// - Random position within a sphere (volume)
/// - Slow upward drift velocity (~0.3 units/sec)
/// - Purple/blue color gradient matching the VOIDFORGED title
/// - Low spawn rate (~3 particles/sec)
/// - Small particles (~0.1 units)
/// - Lifetime of ~7.5 seconds with fade out
fn build_menu_background_particles(effects: &mut Assets<EffectAsset>) -> Handle<EffectAsset> {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.7, 0.5, 1.0, 0.6));  // purple at birth
    gradient.add_key(0.5, Vec4::new(0.4, 0.6, 1.0, 0.3));  // blue mid-life
    gradient.add_key(1.0, Vec4::new(0.2, 0.3, 0.8, 0.0));  // fade out at death

    let mut module = Module::default();

    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(6.0),
        dimension: ShapeDimension::Volume,
    };

    let init_vel = SetAttributeModifier::new(
        Attribute::VELOCITY,
        module.lit(Vec3::new(0.0, 0.3, 0.0)),
    );

    let init_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME,
        module.lit(7.5),
    );

    let init_age = SetAttributeModifier::new(
        Attribute::AGE,
        module.lit(0.0),
    );

    let accel = module.lit(Vec3::new(0.0, 0.04, 0.0));
    let update_accel = AccelModifier::new(accel);

    effects.add(
        EffectAsset::new(
            200,
            SpawnerSettings::rate(3.0.into()),
            module,
        )
        .with_name("menu_background_particles")
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .init(init_age)
        .update(update_accel)
        .render(ColorOverLifetimeModifier {
            gradient,
            blend: ColorBlendMode::Overwrite,
            mask: ColorBlendMask::all(),
        })
        .render(SetSizeModifier {
            size: CpuValue::Single(Vec3::splat(0.1)),
        }),
    )
}

/// System: spawns the main menu background particle effect entity.
pub fn spawn_menu_bg_particles_system(
    mut commands: Commands,
    library: Res<EffectsLibrary>,
) {
    commands.spawn((
        ParticleEffect::new(library.menu_background.clone()),
        Transform::from_translation(Vec3::new(0.0, 2.0, 0.0)),
        GlobalTransform::default(),
        MenuBackgroundParticles,
    ));
}

/// System: despawns all main menu background particle entities.
pub fn despawn_menu_bg_particles_system(
    mut commands: Commands,
    query: Query<Entity, With<MenuBackgroundParticles>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// ============================================================================
// Custom Shader Materials
// ============================================================================

/// A glowing material with time-based pulsing for telegraph/emissive effects.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GlowMaterial {
    #[uniform(0)]
    /// Base (diffuse) color when not pulsing.
    pub base_color: Vec4,
    #[uniform(1)]
    /// Emissive glow color during the pulse peak.
    pub glow_color: Vec4,
    #[uniform(2)]
    /// Intensity multiplier for the glow effect.
    pub glow_intensity: f32,
    #[uniform(3)]
    /// Speed of the pulsation (cycles per second).
    pub pulse_speed: f32,
}

impl Material for GlowMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/glow.wgsl".into()
    }
}

/// Creates a telegraph material (pulsing red for enemy windup).
pub fn telegraph_material() -> GlowMaterial {
    GlowMaterial {
        base_color: Vec4::new(0.3, 0.0, 0.0, 1.0),
        glow_color: Vec4::new(1.0, 0.0, 0.0, 1.0),
        glow_intensity: 2.5,
        pulse_speed: 5.0,
    }
}

/// Creates a green glow material for XP gems.
pub fn gem_glow_material() -> GlowMaterial {
    GlowMaterial {
        base_color: Vec4::new(0.0, 0.3, 0.0, 1.0),
        glow_color: Vec4::new(0.0, 1.0, 0.3, 1.0),
        glow_intensity: 1.5,
        pulse_speed: 2.0,
    }
}
