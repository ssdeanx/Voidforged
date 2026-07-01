//! Glow/telegraph shader — additive emission with time-based pulsing.
//!
//! Uses standard PBR lighting but adds an emissive glow that pulses
//! over time. The glow blends between base_color and glow_color.

#import "bevy_pbr::mesh_bindings"
#import "bevy_pbr::fragment_output"
#import "bevy_pbr::mesh_functions"
#import "bevy_render::view::View"

@group(1) @binding(0)
var<uniform> material_base_color: vec4<f32>;
@group(1) @binding(1)
var<uniform> material_glow_color: vec4<f32>;
@group(1) @binding(2)
var<uniform> material_glow_intensity: f32;
@group(1) @binding(3)
var<uniform> material_pulse_speed: f32;

@group(0) @binding(0)
var<uniform> view: View;

@fragment
fn fragment(in: bevy_pbr::mesh_functions::MeshVertexOutput) -> bevy_pbr::fragment_output::FragmentOutput {
    // Standard lighting would go here — for simplicity, output the glow-blended color
    let time = view.time_jump / 1000.0;
    let pulse = 0.5 + 0.5 * sin(time * material_pulse_speed);
    let glow_factor = material_glow_intensity * pulse;
    let emissive = material_glow_color.rgb * glow_factor;
    let base = material_base_color.rgb;

    var out: bevy_pbr::fragment_output::FragmentOutput;
    out.color = vec4(base + emissive, material_base_color.a);
    return out;
}
