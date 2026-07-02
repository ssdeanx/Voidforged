//! Procedural mesh generation for environment objects.
//!
//! Each function returns a [`Mesh`] built by hand (positions, normals, UVs, indices)
//! or composed from Bevy primitives. All meshes use [`PrimitiveTopology::TriangleList`].
//!
//! Shapes provided:
//! - `make_bush` — sphere cluster (3 overlapping spheres)
//! - `make_tree` — cylinder trunk + cone canopy
//! - `make_rock` — icosphere with vertex displacement
//! - `make_grass_blade` — thin tall quad
//! - `make_flower` — small sphere on a thin stalk
//! - `make_cactus` — cylinder with arms
//! - `make_mushroom` — cone cap on cylinder stalk
//! - `make_crystal` — elongated octahedron
//! - `make_pillar` — fluted cylinder

use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
use bevy::render::render_asset::RenderAssetUsages;
use std::f32::consts::{PI, TAU};

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Merges several meshes into one by concatenating vertex buffers.
/// Each mesh's positions are offset by the corresponding vector in `offsets`.
/// Panics if `offsets` is shorter than `meshes`.
fn merge_shifted(meshes: Vec<Mesh>, offsets: Vec<Vec3>) -> Mesh {
    let mut out_positions: Vec<[f32; 3]> = Vec::new();
    let mut out_normals: Vec<[f32; 3]> = Vec::new();
    let mut out_uvs: Vec<[f32; 2]> = Vec::new();
    let mut out_indices: Vec<u32> = Vec::new();
    let mut base: u32 = 0;

    for (i, mesh) in meshes.into_iter().enumerate() {
        let off = offsets[i];

        let positions: Vec<[f32; 3]> = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .and_then(|v| {
                if let VertexAttributeValues::Float32x3(p) = v {
                    Some(
                        p.iter()
                            .map(|&[x, y, z]| [x + off.x, y + off.y, z + off.z])
                            .collect(),
                    )
                } else {
                    None
                }
            })
            .unwrap_or_default();

        let normals: Vec<[f32; 3]> = mesh
            .attribute(Mesh::ATTRIBUTE_NORMAL)
            .and_then(|v| {
                if let VertexAttributeValues::Float32x3(n) = v {
                    Some(n.clone())
                } else {
                    None
                }
            })
            .unwrap_or_default();

        let uvs: Vec<[f32; 2]> = mesh
            .attribute(Mesh::ATTRIBUTE_UV_0)
            .and_then(|v| {
                if let VertexAttributeValues::Float32x2(u) = v {
                    Some(u.clone())
                } else {
                    None
                }
            })
            .unwrap_or_default();

        let indices: Vec<u32> = mesh
            .indices()
            .map(|idx| match idx {
                Indices::U32(v) => v.iter().map(|x| *x + base).collect(),
                Indices::U16(v) => v.iter().map(|x| *x as u32 + base).collect(),
            })
            .unwrap_or_default();

        let count = positions.len() as u32;
        out_positions.extend(positions);
        out_normals.extend(normals);
        out_uvs.extend(uvs);
        out_indices.extend(indices);
        base += count;
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, out_positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, out_normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, out_uvs);
    mesh.insert_indices(Indices::U32(out_indices));
    mesh
}

/// Build a UV sphere with the given radius and level of detail.
fn uv_sphere(radius: f32, rings: u32, sectors: u32) -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for r in 0..=rings {
        let theta = r as f32 * PI / rings as f32;
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        for s in 0..=sectors {
            let phi = s as f32 * TAU / sectors as f32;
            let sin_phi = phi.sin();
            let cos_phi = phi.cos();

            let nx = sin_theta * cos_phi;
            let ny = cos_theta;
            let nz = sin_theta * sin_phi;
            positions.push([nx * radius, ny * radius, nz * radius]);
            normals.push([nx, ny, nz]);
            uvs.push([s as f32 / sectors as f32, r as f32 / rings as f32]);
        }
    }

    for r in 0..rings {
        for s in 0..sectors {
            let i0 = r * (sectors + 1) + s;
            let i1 = i0 + sectors + 1;
            indices.push(i0);
            indices.push(i1);
            indices.push(i0 + 1);
            indices.push(i1);
            indices.push(i1 + 1);
            indices.push(i0 + 1);
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

/// Build a cone with the given bottom radius, height, and number of sides.
fn cone(radius: f32, height: f32, sides: u32) -> Mesh {
    let apex = [0.0, height, 0.0];
    let mut positions: Vec<[f32; 3]> = vec![apex];
    let mut normals: Vec<[f32; 3]> = vec![[0.0, 1.0, 0.0]];
    let mut uvs: Vec<[f32; 2]> = vec![[0.5, 1.0]];
    let mut indices: Vec<u32> = Vec::new();

    // Base ring vertices
    for s in 0..=sides {
        let phi = s as f32 * TAU / sides as f32;
        let x = radius * phi.cos();
        let z = radius * phi.sin();
        positions.push([x, 0.0, z]);

        // Cone side normal (pointing outward and slightly upward)
        let nx = phi.cos();
        let nz = phi.sin();
        let ny = radius / height; // slope factor
        let len = (nx * nx + ny * ny + nz * nz).sqrt();
        normals.push([nx / len, ny / len, nz / len]);
        uvs.push([s as f32 / sides as f32, 0.0]);
    }

    // Cone side triangles
    for s in 0..sides {
        indices.push(0); // apex
        indices.push(s + 2);
        indices.push(s + 1);
    }

    // Base (flat bottom, facing down)
    let base_center_idx = positions.len() as u32;
    positions.push([0.0, 0.0, 0.0]);
    normals.push([0.0, -1.0, 0.0]);
    uvs.push([0.5, 0.5]);

    for s in 0..=sides {
        let phi = s as f32 * TAU / sides as f32;
        positions.push([radius * phi.cos(), 0.0, radius * phi.sin()]);
        normals.push([0.0, -1.0, 0.0]);
        uvs.push([0.5 + 0.5 * phi.cos(), 0.5 + 0.5 * phi.sin()]);
    }

    for s in 0..sides {
        indices.push(base_center_idx);
        indices.push(base_center_idx + s as u32 + 1);
        indices.push(base_center_idx + s as u32 + 2);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

/// Build a simple cylinder.
fn cylinder(radius: f32, height: f32, sides: u32) -> Mesh {
    let half = height * 0.5;
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    // Side vertices: top ring + bottom ring
    let top_start = 0u32;
    let bot_start = sides + 1;

    for s in 0..=sides {
        let phi = s as f32 * TAU / sides as f32;
        let x = radius * phi.cos();
        let z = radius * phi.sin();
        let nx = phi.cos();
        let nz = phi.sin();
        let u = s as f32 / sides as f32;

        positions.push([x, half, z]);
        normals.push([nx, 0.0, nz]);
        uvs.push([u, 1.0]);
    }
    for s in 0..=sides {
        let phi = s as f32 * TAU / sides as f32;
        let x = radius * phi.cos();
        let z = radius * phi.sin();
        let nx = phi.cos();
        let nz = phi.sin();
        let u = s as f32 / sides as f32;

        positions.push([x, -half, z]);
        normals.push([nx, 0.0, nz]);
        uvs.push([u, 0.0]);
    }

    // Side triangles (quad strip)
    for s in 0..sides {
        let a = top_start + s;
        let b = a + 1;
        let c = bot_start + s;
        let d = c + 1;
        indices.extend_from_slice(&[a, c, b, c, d, b]);
    }

    // Top cap
    let top_center = positions.len() as u32;
    positions.push([0.0, half, 0.0]);
    normals.push([0.0, 1.0, 0.0]);
    uvs.push([0.5, 0.5]);
    for s in 0..=sides {
        let phi = s as f32 * TAU / sides as f32;
        let x = radius * phi.cos();
        let z = radius * phi.sin();
        positions.push([x, half, z]);
        normals.push([0.0, 1.0, 0.0]);
        uvs.push([0.5 + 0.5 * phi.cos(), 0.5 + 0.5 * phi.sin()]);
    }
    for s in 0..sides {
        indices.push(top_center);
        indices.push(top_center + s as u32 + 1);
        indices.push(top_center + s as u32 + 2);
    }

    // Bottom cap
    let bot_center = positions.len() as u32;
    positions.push([0.0, -half, 0.0]);
    normals.push([0.0, -1.0, 0.0]);
    uvs.push([0.5, 0.5]);
    for s in 0..=sides {
        let phi = s as f32 * TAU / sides as f32;
        let x = radius * phi.cos();
        let z = radius * phi.sin();
        positions.push([x, -half, z]);
        normals.push([0.0, -1.0, 0.0]);
        uvs.push([0.5 + 0.5 * phi.cos(), 0.5 + 0.5 * phi.sin()]);
    }
    for s in 0..sides {
        indices.push(bot_center);
        indices.push(bot_center + s as u32 + 2);
        indices.push(bot_center + s as u32 + 1);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

/// Displace vertices of a mesh by a small random offset for organic look.
fn displace_vertices(mesh: &mut Mesh, amount: f32) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // Read positions, displace, write back
    let displaced: Vec<[f32; 3]> = {
        let positions = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .and_then(|v| {
                if let VertexAttributeValues::Float32x3(p) = v {
                    Some(p.clone())
                } else {
                    None
                }
            })
            .unwrap_or_default();
        positions
            .iter()
            .map(|&[x, y, z]| {
                let dx = rng.gen_range(-amount..amount);
                let dy = rng.gen_range(-amount..amount);
                let dz = rng.gen_range(-amount..amount);
                [x + dx, y + dy, z + dz]
            })
            .collect()
    };
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, displaced.clone());

    // Recompute normals from displaced positions
    let new_normals: Vec<[f32; 3]> = displaced
        .iter()
        .map(|p| {
            let len = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
            if len > 0.0 {
                [p[0] / len, p[1] / len, p[2] / len]
            } else {
                [0.0, 1.0, 0.0]
            }
        })
        .collect();
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, new_normals);
}

// ─── Public API ──────────────────────────────────────────────────────────────

/// A round bush made from 3 overlapping sphere-like clusters.
pub fn make_bush(size: f32) -> Mesh {
    let r = size * 0.3;
    let spheres = vec![
        uv_sphere(r, 8, 8),
        uv_sphere(r * 0.75, 6, 6),
        uv_sphere(r * 0.6, 6, 6),
    ];
    let offsets = vec![
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(r * 0.5, r * 0.3, r * 0.3),
        Vec3::new(-r * 0.4, r * 0.2, -r * 0.4),
    ];
    let mut mesh = merge_shifted(spheres, offsets);
    displace_vertices(&mut mesh, size * 0.05);
    mesh
}

/// A tree: cylinder trunk topped with a cone canopy.
pub fn make_tree(trunk_height: f32, canopy_radius: f32) -> Mesh {
    let trunk_r = canopy_radius * 0.12;
    let canopy_height = canopy_radius * 1.4;
    let trunk = cylinder(trunk_r, trunk_height, 8);
    let canopy = cone(canopy_radius, canopy_height, 8);
    merge_shifted(
        vec![trunk, canopy],
        vec![Vec3::ZERO, Vec3::new(0.0, trunk_height, 0.0)],
    )
}

/// A rock: sphere with vertex displacement for an organic look.
pub fn make_rock(size: f32, _seed: u32) -> Mesh {
    let mut mesh = uv_sphere(size * 0.5, 6, 6);
    displace_vertices(&mut mesh, size * 0.15);
    mesh
}

/// A thin tall grass blade (2-triangle quad).
pub fn make_grass_blade(height: f32) -> Mesh {
    let half_w = 0.02;
    let half_h = height * 0.5;
    let positions: Vec<[f32; 3]> = vec![
        [-half_w, -half_h, 0.0],
        [half_w, -half_h, 0.0],
        [-half_w, half_h, 0.0],
        [half_w, half_h, 0.0],
    ];
    let normals: Vec<[f32; 3]> = vec![[0.0, 0.0, 1.0]; 4];
    let uvs: Vec<[f32; 2]> = vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    let indices: Vec<u32> = vec![0, 1, 2, 2, 1, 3];

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

/// A flower: small sphere on a thin cylindrical stalk.
pub fn make_flower(head_radius: f32) -> Mesh {
    let stalk_h = head_radius * 4.0;
    let stalk_r = head_radius * 0.1;
    let stalk = cylinder(stalk_r, stalk_h, 6);
    let head = uv_sphere(head_radius, 6, 6);
    merge_shifted(
        vec![stalk, head],
        vec![
            Vec3::new(0.0, -stalk_h * 0.5, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
        ],
    )
}

/// A cactus: main cylinder with two arm cylinders.
pub fn make_cactus(height: f32) -> Mesh {
    let main_r = height * 0.06;
    let main_cyl = cylinder(main_r, height, 8);

    let arm_r = main_r * 0.6;
    let arm_len = height * 0.3;
    let arm = cylinder(arm_r, arm_len, 6);
    let arm2 = cylinder(arm_r, arm_len, 6); // clone to avoid move

    merge_shifted(
        vec![main_cyl, arm, arm2],
        vec![
            Vec3::ZERO,
            Vec3::new(main_r + arm_len * 0.5, height * 0.4, 0.0),
            Vec3::new(-(main_r + arm_len * 0.5), height * 0.6, 0.0),
        ],
    )
}

/// A mushroom: cone cap on a cylinder stalk.
pub fn make_mushroom(cap_radius: f32) -> Mesh {
    let stalk_h = cap_radius * 0.8;
    let stalk_r = cap_radius * 0.15;
    let cap_h = cap_radius * 0.8;
    let stalk = cylinder(stalk_r, stalk_h, 8);
    let cap = cone(cap_radius, cap_h, 10);
    merge_shifted(
        vec![stalk, cap],
        vec![
            Vec3::new(0.0, -stalk_h * 0.5, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
        ],
    )
}

/// An elongated octahedron (diamond/crystal shape).
pub fn make_crystal(height: f32) -> Mesh {
    let half_h = height * 0.5;
    let r = height * 0.12;

    let positions: Vec<[f32; 3]> = vec![
        [0.0, half_h, 0.0],  // 0: top
        [r, 0.0, 0.0],       // 1: +X
        [0.0, 0.0, r],       // 2: +Z
        [-r, 0.0, 0.0],      // 3: -X
        [0.0, 0.0, -r],      // 4: -Z
        [0.0, -half_h, 0.0], // 5: bottom
    ];

    // Compute per-vertex normals by averaging face normals
    let indices: Vec<u32> = vec![
        0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 1, 5, 2, 1, 5, 3, 2, 5, 4, 3, 5, 1, 4,
    ];

    let compute_normal = |a: [f32; 3], b: [f32; 3], c: [f32; 3]| -> [f32; 3] {
        let ux = b[0] - a[0];
        let uy = b[1] - a[1];
        let uz = b[2] - a[2];
        let vx = c[0] - a[0];
        let vy = c[1] - a[1];
        let vz = c[2] - a[2];
        let nx = uy * vz - uz * vy;
        let ny = uz * vx - ux * vz;
        let nz = ux * vy - uy * vx;
        let len = (nx * nx + ny * ny + nz * nz).sqrt();
        if len > 0.0 {
            [nx / len, ny / len, nz / len]
        } else {
            [0.0, 1.0, 0.0]
        }
    };

    let mut vertex_normals = vec![[0.0, 0.0, 0.0]; 6];
    for tri in indices.chunks(3) {
        let a = positions[tri[0] as usize];
        let b = positions[tri[1] as usize];
        let c = positions[tri[2] as usize];
        let n = compute_normal(a, b, c);
        for &idx in tri {
            let idx = idx as usize;
            vertex_normals[idx][0] += n[0];
            vertex_normals[idx][1] += n[1];
            vertex_normals[idx][2] += n[2];
        }
    }
    for n in vertex_normals.iter_mut() {
        let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        if len > 0.0 {
            n[0] /= len;
            n[1] /= len;
            n[2] /= len;
        }
    }

    let uvs = vec![[0.0, 0.0]; 6];

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vertex_normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

/// A fluted cylinder (pillar with vertical ridges).
pub fn make_pillar(height: f32, width: f32) -> Mesh {
    let r = width * 0.5;
    let flutes: u32 = 12;
    let half = height * 0.5;
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let flute_depth = r * 0.12;

    // Top and bottom rings with alternating fluted radius
    for s in 0..=flutes * 2 {
        let phi = s as f32 * TAU / (flutes * 2) as f32;
        let is_flute = s % 2 == 0;
        let rad = if is_flute { r - flute_depth } else { r };
        let x = rad * phi.cos();
        let z = rad * phi.sin();
        let nx = phi.cos();
        let nz = phi.sin();
        let u = s as f32 / (flutes * 2) as f32;

        positions.push([x, half, z]);
        normals.push([nx, 0.0, nz]);
        uvs.push([u, 1.0]);
    }
    for s in 0..=flutes * 2 {
        let phi = s as f32 * TAU / (flutes * 2) as f32;
        let is_flute = s % 2 == 0;
        let rad = if is_flute { r - flute_depth } else { r };
        let x = rad * phi.cos();
        let z = rad * phi.sin();
        let nx = phi.cos();
        let nz = phi.sin();
        let u = s as f32 / (flutes * 2) as f32;

        positions.push([x, -half, z]);
        normals.push([nx, 0.0, nz]);
        uvs.push([u, 0.0]);
    }

    let top_start = 0;
    let bot_start = (flutes * 2 + 1) as u32;

    // Side triangles
    for s in 0..(flutes * 2) {
        let a = top_start + s;
        let b = a + 1;
        let c = bot_start + s as u32;
        let d = c + 1;
        indices.extend_from_slice(&[a, c, b, c, d, b]);
    }

    // Top cap
    let top_center = positions.len() as u32;
    positions.push([0.0, half, 0.0]);
    normals.push([0.0, 1.0, 0.0]);
    uvs.push([0.5, 0.5]);
    for s in 0..=flutes * 2 {
        let phi = s as f32 * TAU / (flutes * 2) as f32;
        let is_flute = s % 2 == 0;
        let rad = if is_flute { r - flute_depth } else { r };
        positions.push([rad * phi.cos(), half, rad * phi.sin()]);
        normals.push([0.0, 1.0, 0.0]);
        uvs.push([0.5 + 0.5 * phi.cos(), 0.5 + 0.5 * phi.sin()]);
    }
    for s in 0..(flutes * 2) {
        indices.push(top_center);
        indices.push(top_center + s as u32 + 1);
        indices.push(top_center + s as u32 + 2);
    }

    // Bottom cap
    let bot_center = positions.len() as u32;
    positions.push([0.0, -half, 0.0]);
    normals.push([0.0, -1.0, 0.0]);
    uvs.push([0.5, 0.5]);
    for s in 0..=flutes * 2 {
        let phi = s as f32 * TAU / (flutes * 2) as f32;
        let is_flute = s % 2 == 0;
        let rad = if is_flute { r - flute_depth } else { r };
        positions.push([rad * phi.cos(), -half, rad * phi.sin()]);
        normals.push([0.0, -1.0, 0.0]);
        uvs.push([0.5 + 0.5 * phi.cos(), 0.5 + 0.5 * phi.sin()]);
    }
    for s in 0..(flutes * 2) {
        indices.push(bot_center);
        indices.push(bot_center + s as u32 + 2);
        indices.push(bot_center + s as u32 + 1);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}
