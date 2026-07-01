//! Procedural UI texture generation.
//!
//! Generates panel backgrounds, 9-slice borders, gradient fills, and
//! other UI textures at startup. All textures are created as raw
//! `Image` assets and stored in `UiTextureAssets` for use by HUD panels.
//!
//! Texture generation happens once during `OnEnter(AppState::Loading)`,
//! right after `generate_placeholder_assets`.

use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::sprite::{BorderRect, SliceScaleMode, TextureSlicer};
use bevy::ui::widget::{ImageNode, NodeImageMode};

// ============================================================================
// Resource — all generated UI texture handles
// ============================================================================

/// Holds handles to every procedural UI texture used by the HUD.
#[derive(Resource, Debug, Clone)]
pub struct UiTextureAssets {
    // Panel backgrounds (rounded rects)
    pub panel_dark: Handle<Image>,
    pub panel_light: Handle<Image>,
    pub panel_input: Handle<Image>,

    // 9-slice border textures (small frame images)
    pub border_default: Handle<Image>,
    pub border_gold: Handle<Image>,
    pub border_red: Handle<Image>,

    // Gradient bar fills
    pub bar_health: Handle<Image>,
    pub bar_resource: Handle<Image>,
    pub bar_xp: Handle<Image>,
    pub bar_stamina: Handle<Image>,

    // Action bar
    pub slot_bg: Handle<Image>,
    pub slot_border: Handle<Image>,
    pub cooldown_overlay: Handle<Image>,

    // Buttons
    pub btn_primary: Handle<Image>,
    pub btn_danger: Handle<Image>,
    pub btn_hover: Handle<Image>,

    // Minimap
    pub minimap_border: Handle<Image>,
}

/// Component for buttons that support hover-state texture swapping.
#[derive(Component)]
pub struct HoverableButton {
    /// Texture handle for the normal (un-hovered) state.
    pub normal: Handle<Image>,
    /// Texture handle for the hovered / pressed state.
    pub hover: Handle<Image>,
}

// ============================================================================
// Startup system — generates every texture and inserts the resource
// ============================================================================

/// Startup system: generates all UI textures and inserts `UiTextureAssets`.
///
/// Should run in `OnEnter(AppState::Loading)` after `generate_placeholder_assets`.
pub fn generate_ui_textures(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.insert_resource(UiTextureAssets {
        // Panel backgrounds
        panel_dark: images.add(make_panel_dark()),
        panel_light: images.add(make_panel_light()),
        panel_input: images.add(make_panel_input()),

        // Borders
        border_default: images.add(make_border_texture(8, 2, [80, 80, 90, 255])),
        border_gold: images.add(make_border_texture(8, 2, [200, 170, 50, 255])),
        border_red: images.add(make_border_texture(8, 2, [180, 40, 40, 255])),

        // Gradient bars
        bar_health: images.add(make_health_gradient()),
        bar_resource: images.add(make_resource_gradient()),
        bar_xp: images.add(make_xp_gradient()),
        bar_stamina: images.add(make_stamina_gradient()),

        // Action bar
        slot_bg: images.add(make_slot_background()),
        slot_border: images.add(make_border_texture(8, 2, [70, 70, 80, 255])),
        cooldown_overlay: images.add(make_cooldown_overlay()),

        // Buttons
        btn_primary: images.add(make_rounded_rect(64, 32, 4, 1, [40, 80, 180, 255], [60, 100, 200, 255])),
        btn_danger: images.add(make_rounded_rect(64, 32, 4, 1, [150, 30, 30, 255], [180, 50, 50, 255])),
        btn_hover: images.add(make_rounded_rect(64, 32, 4, 1, [60, 100, 200, 255], [80, 120, 220, 255])),

        // Minimap
        minimap_border: images.add(make_minimap_border()),
    });
}

/// System that swaps button textures on hover/unhover.
pub fn button_hover_system(
    mut query: Query<(&Interaction, &mut ImageNode, &HoverableButton), Changed<Interaction>>,
) {
    for (interaction, mut image, btn) in query.iter_mut() {
        match interaction {
            Interaction::Hovered | Interaction::Pressed => {
                image.image = btn.hover.clone();
            }
            Interaction::None => {
                image.image = btn.normal.clone();
            }
        }
    }
}

/// Convenience: build an `ImageNode` with a proper 9-slice configuration.
/// The border texture should be an N×N image with `border_thickness` pixels
/// of border colour on each edge and transparent centre.
pub fn make_9slice_node(handle: Handle<Image>, border_thickness: f32) -> ImageNode {
    ImageNode {
        image: handle,
        image_mode: NodeImageMode::Sliced(TextureSlicer {
            border: BorderRect::square(border_thickness),
            center_scale_mode: SliceScaleMode::Stretch,
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: 1.0,
        }),
        ..Default::default()
    }
}

// ============================================================================
// Image utility helpers
// ============================================================================

fn make_image(width: u32, height: u32, data: Vec<u8>) -> Image {
    Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
}

fn set_pixel(data: &mut [u8], x: u32, y: u32, width: u32, color: [u8; 4]) {
    let idx = ((y * width + x) * 4) as usize;
    if idx + 3 < data.len() {
        data[idx] = color[0];
        data[idx + 1] = color[1];
        data[idx + 2] = color[2];
        data[idx + 3] = color[3];
    }
}

fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t).round().clamp(0.0, 255.0) as u8
}

// ============================================================================
// Border textures (small images for 9-slice)
// ============================================================================

/// Creates a small square border frame image for 9-slice scaling.
///
/// The outer `thickness` pixels on each edge get the border `color`;
/// the interior is transparent. When sliced with `BorderRect::square(thickness)`,
/// the corners stay fixed, the edges stretch, and the centre stays transparent.
fn make_border_texture(size: u32, thickness: u32, color: [u8; 4]) -> Image {
    let mut data = vec![0u8; (size * size * 4) as usize];
    for y in 0..size {
        for x in 0..size {
            if x < thickness || y < thickness || x >= size - thickness || y >= size - thickness {
                set_pixel(&mut data, x, y, size, color);
            }
        }
    }
    make_image(size, size, data)
}

// ============================================================================
// Panel backgrounds (rounded rectangles)
// ============================================================================

fn make_panel_dark() -> Image {
    make_rounded_rect(256, 256, 8, 2, [20, 20, 30, 242], [50, 50, 60, 255])
}

fn make_panel_light() -> Image {
    make_rounded_rect(256, 256, 8, 2, [35, 35, 45, 242], [70, 70, 80, 255])
}

fn make_panel_input() -> Image {
    make_rounded_rect(256, 32, 4, 1, [15, 15, 20, 242], [40, 40, 50, 255])
}

// ============================================================================
// Gradient bars
// ============================================================================

fn make_health_gradient() -> Image {
    make_gradient_horizontal(256, 18, [50, 200, 30, 255], [20, 100, 10, 255])
}

fn make_resource_gradient() -> Image {
    make_gradient_horizontal(256, 12, [60, 100, 220, 255], [20, 40, 140, 255])
}

fn make_xp_gradient() -> Image {
    make_gradient_horizontal(256, 16, [140, 60, 220, 255], [60, 20, 120, 255])
}

fn make_stamina_gradient() -> Image {
    make_gradient_horizontal(256, 10, [220, 180, 40, 255], [160, 120, 10, 255])
}

// ============================================================================
// Slot backgrounds
// ============================================================================

fn make_slot_background() -> Image {
    make_rounded_rect(72, 66, 3, 1, [25, 25, 33, 255], [45, 45, 55, 255])
}

// ============================================================================
// Cooldown overlay
// ============================================================================

fn make_cooldown_overlay() -> Image {
    let w = 72u32;
    let h = 66u32;
    let mut data = vec![0u8; (w * h * 4) as usize];
    for y in 0..h {
        for x in 0..w {
            set_pixel(&mut data, x, y, w, [0, 0, 0, 140]);
        }
    }
    make_image(w, h, data)
}

// ============================================================================
// Minimap border (circular ring)
// ============================================================================

fn make_minimap_border() -> Image {
    let size = 160u32;
    let cx = size as f32 / 2.0;
    let cy = size as f32 / 2.0;
    let outer_r = size as f32 / 2.0 - 1.0;
    let inner_r = outer_r - 3.0;
    let mut data = vec![0u8; (size * size * 4) as usize];

    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist >= inner_r && dist <= outer_r {
                set_pixel(&mut data, x, y, size, [100, 100, 120, 255]);
            }
        }
    }
    make_image(size, size, data)
}

// ============================================================================
// Core shape generators
// ============================================================================

/// Checks whether a point `(x, y)` lies inside a rounded rectangle anchored at
/// `(0, 0)` with the given width, height and corner radius.
fn inside_rounded_rect(x: u32, y: u32, w: u32, h: u32, r: u32) -> bool {
    if x >= w || y >= h {
        return false;
    }
    if r == 0 {
        return true;
    }
    let r = r.min(w / 2).min(h / 2);

    // Top-left corner
    if x < r && y < r {
        let dx = r as i32 - x as i32;
        let dy = r as i32 - y as i32;
        return dx * dx + dy * dy <= (r * r) as i32;
    }
    // Top-right corner
    if x >= w - r && y < r {
        let dx = x as i32 - (w - r) as i32;
        let dy = r as i32 - y as i32;
        return dx * dx + dy * dy <= (r * r) as i32;
    }
    // Bottom-left corner
    if x < r && y >= h - r {
        let dx = r as i32 - x as i32;
        let dy = y as i32 - (h - r) as i32;
        return dx * dx + dy * dy <= (r * r) as i32;
    }
    // Bottom-right corner
    if x >= w - r && y >= h - r {
        let dx = x as i32 - (w - r) as i32;
        let dy = y as i32 - (h - r) as i32;
        return dx * dx + dy * dy <= (r * r) as i32;
    }
    true // central region
}

/// Draws a rounded rectangle with a solid fill and optional border stroke.
///
/// Pixels inside the outer rounded rect get the `fill` colour. Pixels that are
/// inside the outer shape but *outside* the inner shape (inset by `border`)
/// get the `stroke` colour. Everything outside is transparent.
fn make_rounded_rect(
    width: u32,
    height: u32,
    radius: u32,
    border: u32,
    fill: [u8; 4],
    stroke: [u8; 4],
) -> Image {
    let mut data = vec![0u8; (width * height * 4) as usize];
    let r = radius.min(width / 2).min(height / 2);

    for y in 0..height {
        for x in 0..width {
            if !inside_rounded_rect(x, y, width, height, r) {
                continue; // transparent
            }

            if border > 0 {
                let in_stroke = x < border
                    || y < border
                    || x >= width - border
                    || y >= height - border
                    || !inside_rounded_rect(
                        x.saturating_sub(border),
                        y.saturating_sub(border),
                        width.saturating_sub(border * 2),
                        height.saturating_sub(border * 2),
                        r.saturating_sub(border),
                    );

                if in_stroke {
                    set_pixel(&mut data, x, y, width, stroke);
                } else {
                    set_pixel(&mut data, x, y, width, fill);
                }
            } else {
                set_pixel(&mut data, x, y, width, fill);
            }
        }
    }
    make_image(width, height, data)
}

/// Creates a horizontal linear gradient from `left` to `right`.
fn make_gradient_horizontal(width: u32, height: u32, left: [u8; 4], right: [u8; 4]) -> Image {
    let mut data = vec![0u8; (width * height * 4) as usize];
    let denom = (width.max(1) - 1) as f32;

    for y in 0..height {
        for x in 0..width {
            let t = x as f32 / denom;
            let idx = ((y * width + x) * 4) as usize;
            data[idx] = lerp_u8(left[0], right[0], t);
            data[idx + 1] = lerp_u8(left[1], right[1], t);
            data[idx + 2] = lerp_u8(left[2], right[2], t);
            data[idx + 3] = lerp_u8(left[3], right[3], t);
        }
    }
    make_image(width, height, data)
}
