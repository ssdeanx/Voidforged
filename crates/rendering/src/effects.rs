//! GPU particle effects and custom shader materials for professional VFX.

use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy_hanabi::*;

// ============================================================================
// Effects Library — pre-built effect handles stored as a resource
// ============================================================================

/// Resource holding handles to all pre-built particle effects.
#[derive(Resource)]
pub struct EffectsLibrary {
    pub impact_burst: Handle<EffectAsset>,
    pub glow_green: Handle<EffectAsset>,
    pub glow_blue: Handle<EffectAsset>,
    pub dash_trail: Handle<EffectAsset>,
}

/// Builds all effect assets and inserts them as a resource.
pub fn build_effects_library(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let impact_burst = build_impact_burst(&mut effects);
    let glow_green = build_glow(&mut effects, Vec4::new(0.0, 1.0, 0.3, 1.0));
    let glow_blue = build_glow(&mut effects, Vec4::new(0.3, 0.6, 1.0, 1.0));
    let dash_trail = build_dash_trail(&mut effects);
    commands.insert_resource(EffectsLibrary { impact_burst, glow_green, glow_blue, dash_trail });
}

fn build_impact_burst(effects: &mut Assets<EffectAsset>) -> Handle<EffectAsset> {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1.0, 0.8, 0.3, 1.0));
    gradient.add_key(0.5, Vec4::new(1.0, 0.4, 0.0, 0.5));
    gradient.add_key(1.0, Vec4::new(0.5, 0.0, 0.0, 0.0));

    effects.add(
        EffectAsset::new(64, SpawnerSettings::once(16.0.into()), Module::default())
            .with_name("impact_burst")
            .render(ColorOverLifetimeModifier {
                gradient,
                blend: ColorBlendMode::Overwrite,
                mask: ColorBlendMask::all(),
            })
            .render(SetSizeModifier { size: CpuValue::Single(Vec3::splat(0.3)) }),
    )
}

fn build_glow(effects: &mut Assets<EffectAsset>, color: Vec4) -> Handle<EffectAsset> {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, color);
    gradient.add_key(0.5, Vec4::new(color.x * 0.3, color.y * 0.3, color.z * 0.3, 0.2));
    gradient.add_key(1.0, color);

    effects.add(
        EffectAsset::new(32, SpawnerSettings::rate(20.0.into()), Module::default())
            .with_name("glow")
            .render(ColorOverLifetimeModifier {
                gradient,
                blend: ColorBlendMode::Overwrite,
                mask: ColorBlendMask::all(),
            })
            .render(SetSizeModifier { size: CpuValue::Single(Vec3::splat(0.4)) }),
    )
}

fn build_dash_trail(effects: &mut Assets<EffectAsset>) -> Handle<EffectAsset> {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.5, 0.8, 1.0, 0.6));
    gradient.add_key(1.0, Vec4::new(0.3, 0.5, 0.8, 0.0));

    effects.add(
        EffectAsset::new(32, SpawnerSettings::rate(30.0.into()), Module::default())
            .with_name("dash_trail")
            .render(ColorOverLifetimeModifier {
                gradient,
                blend: ColorBlendMode::Overwrite,
                mask: ColorBlendMask::all(),
            })
            .render(SetSizeModifier { size: CpuValue::Single(Vec3::splat(0.25)) }),
    )
}

// ============================================================================
// Inline particle spawners (use EffectLibrary handles)
// ============================================================================

/// Spawns an impact burst at a position.
pub fn spawn_impact(commands: &mut Commands, library: &EffectsLibrary, position: Vec3) {
    commands.spawn((
        ParticleEffect::new(library.impact_burst.clone()),
        Transform::from_translation(position),
        GlobalTransform::default(),
        ir_core::Lifetime { remaining: 0.5 },
    ));
}

/// Spawns a glow effect at a position.
pub fn spawn_glow(commands: &mut Commands, library: &EffectsLibrary, position: Vec3) -> Entity {
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

// ============================================================================
// Custom Shader Materials
// ============================================================================

/// A glowing material with time-based pulsing for telegraph/emissive effects.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GlowMaterial {
    #[uniform(0)]
    pub base_color: Vec4,
    #[uniform(1)]
    pub glow_color: Vec4,
    #[uniform(2)]
    pub glow_intensity: f32,
    #[uniform(3)]
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
