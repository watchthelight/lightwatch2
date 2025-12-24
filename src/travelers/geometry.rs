//! Traveler geometry - procedural Platonic solid mesh generation

#![allow(dead_code)]

use std::collections::HashMap;

use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;

use super::TravelerGeometry;

/// Generate mesh for traveler geometry type
pub fn generate_mesh(geometry: TravelerGeometry, noise_amount: f32) -> Mesh {
    match geometry {
        TravelerGeometry::Icosahedron => generate_icosahedron(noise_amount),
        TravelerGeometry::Tetrahedron => generate_tetrahedron(noise_amount),
        TravelerGeometry::Cube => generate_cube(noise_amount),
        TravelerGeometry::Octahedron => generate_octahedron(noise_amount),
        TravelerGeometry::Dodecahedron => generate_dodecahedron(noise_amount),
    }
}

/// Icosahedron - 20 triangular faces (Archivist)
pub fn generate_icosahedron(noise: f32) -> Mesh {
    let phi = (1.0 + 5.0_f32.sqrt()) / 2.0; // Golden ratio

    let mut vertices = vec![
        Vec3::new(-1.0, phi, 0.0),
        Vec3::new(1.0, phi, 0.0),
        Vec3::new(-1.0, -phi, 0.0),
        Vec3::new(1.0, -phi, 0.0),
        Vec3::new(0.0, -1.0, phi),
        Vec3::new(0.0, 1.0, phi),
        Vec3::new(0.0, -1.0, -phi),
        Vec3::new(0.0, 1.0, -phi),
        Vec3::new(phi, 0.0, -1.0),
        Vec3::new(phi, 0.0, 1.0),
        Vec3::new(-phi, 0.0, -1.0),
        Vec3::new(-phi, 0.0, 1.0),
    ];

    // Normalize and apply noise
    for v in vertices.iter_mut() {
        *v = v.normalize();
        apply_noise(v, noise);
    }

    let indices: Vec<u32> = vec![
        0, 11, 5, 0, 5, 1, 0, 1, 7, 0, 7, 10, 0, 10, 11, 1, 5, 9, 5, 11, 4, 11, 10, 2, 10, 7, 6,
        7, 1, 8, 3, 9, 4, 3, 4, 2, 3, 2, 6, 3, 6, 8, 3, 8, 9, 4, 9, 5, 2, 4, 11, 6, 2, 10, 8, 6,
        7, 9, 8, 1,
    ];

    build_mesh(vertices, indices)
}

/// Tetrahedron - 4 triangular faces (Wanderer)
pub fn generate_tetrahedron(noise: f32) -> Mesh {
    let a = 1.0 / 3.0_f32.sqrt();

    let mut vertices = vec![
        Vec3::new(a, a, a),
        Vec3::new(-a, -a, a),
        Vec3::new(-a, a, -a),
        Vec3::new(a, -a, -a),
    ];

    for v in vertices.iter_mut() {
        *v = v.normalize();
        apply_noise(v, noise);
    }

    let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3, 0, 3, 1, 1, 3, 2];

    build_mesh(vertices, indices)
}

/// Cube - 6 square faces (Keeper)
pub fn generate_cube(noise: f32) -> Mesh {
    let s = 1.0 / 3.0_f32.sqrt();

    let mut vertices = vec![
        Vec3::new(-s, -s, -s),
        Vec3::new(s, -s, -s),
        Vec3::new(s, s, -s),
        Vec3::new(-s, s, -s),
        Vec3::new(-s, -s, s),
        Vec3::new(s, -s, s),
        Vec3::new(s, s, s),
        Vec3::new(-s, s, s),
    ];

    for v in vertices.iter_mut() {
        *v = v.normalize();
        apply_noise(v, noise);
    }

    // Each face as two triangles
    let indices: Vec<u32> = vec![
        0, 2, 1, 0, 3, 2, // Front
        4, 5, 6, 4, 6, 7, // Back
        0, 1, 5, 0, 5, 4, // Bottom
        2, 3, 7, 2, 7, 6, // Top
        0, 4, 7, 0, 7, 3, // Left
        1, 2, 6, 1, 6, 5, // Right
    ];

    build_mesh(vertices, indices)
}

/// Octahedron - 8 triangular faces (Child)
pub fn generate_octahedron(noise: f32) -> Mesh {
    let mut vertices = vec![
        Vec3::new(0.0, 1.0, 0.0),  // Top
        Vec3::new(0.0, -1.0, 0.0), // Bottom
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
    ];

    for v in vertices.iter_mut() {
        apply_noise(v, noise);
    }

    let indices: Vec<u32> = vec![
        0, 4, 2, 0, 2, 5, 0, 5, 3, 0, 3, 4, // Top
        1, 2, 4, 1, 5, 2, 1, 3, 5, 1, 4, 3, // Bottom
    ];

    build_mesh(vertices, indices)
}

/// Dodecahedron - 12 pentagonal faces (Other)
pub fn generate_dodecahedron(noise: f32) -> Mesh {
    let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;
    let inv_phi = 1.0 / phi;

    let mut vertices = vec![
        // Cube vertices
        Vec3::new(1.0, 1.0, 1.0),    // 0
        Vec3::new(1.0, 1.0, -1.0),   // 1
        Vec3::new(1.0, -1.0, 1.0),   // 2
        Vec3::new(1.0, -1.0, -1.0),  // 3
        Vec3::new(-1.0, 1.0, 1.0),   // 4
        Vec3::new(-1.0, 1.0, -1.0),  // 5
        Vec3::new(-1.0, -1.0, 1.0),  // 6
        Vec3::new(-1.0, -1.0, -1.0), // 7
        // Rectangle vertices (3 orientations)
        Vec3::new(0.0, inv_phi, phi),  // 8
        Vec3::new(0.0, inv_phi, -phi), // 9
        Vec3::new(0.0, -inv_phi, phi), // 10
        Vec3::new(0.0, -inv_phi, -phi), // 11
        Vec3::new(inv_phi, phi, 0.0),  // 12
        Vec3::new(inv_phi, -phi, 0.0), // 13
        Vec3::new(-inv_phi, phi, 0.0), // 14
        Vec3::new(-inv_phi, -phi, 0.0), // 15
        Vec3::new(phi, 0.0, inv_phi),  // 16
        Vec3::new(phi, 0.0, -inv_phi), // 17
        Vec3::new(-phi, 0.0, inv_phi), // 18
        Vec3::new(-phi, 0.0, -inv_phi), // 19
    ];

    for v in vertices.iter_mut() {
        *v = v.normalize();
        apply_noise(v, noise);
    }

    // Pentagon faces triangulated (each pentagon = 3 triangles from center)
    // 12 pentagons, each becomes 5 triangles for a fan triangulation
    let indices: Vec<u32> = vec![
        // Face 1: 0, 8, 10, 2, 16
        0, 8, 10, 0, 10, 2, 0, 2, 16,
        // Face 2: 0, 16, 17, 1, 12
        0, 16, 17, 0, 17, 1, 0, 1, 12,
        // Face 3: 0, 12, 14, 4, 8
        0, 12, 14, 0, 14, 4, 0, 4, 8,
        // Face 4: 1, 17, 3, 11, 9
        1, 17, 3, 1, 3, 11, 1, 11, 9,
        // Face 5: 1, 9, 5, 14, 12
        1, 9, 5, 1, 5, 14, 1, 14, 12,
        // Face 6: 2, 10, 6, 15, 13
        2, 10, 6, 2, 6, 15, 2, 15, 13,
        // Face 7: 2, 13, 3, 17, 16
        2, 13, 3, 2, 3, 17, 2, 17, 16,
        // Face 8: 3, 13, 15, 7, 11
        3, 13, 15, 3, 15, 7, 3, 7, 11,
        // Face 9: 4, 14, 5, 19, 18
        4, 14, 5, 4, 5, 19, 4, 19, 18,
        // Face 10: 4, 18, 6, 10, 8
        4, 18, 6, 4, 6, 10, 4, 10, 8,
        // Face 11: 5, 9, 11, 7, 19
        5, 9, 11, 5, 11, 7, 5, 7, 19,
        // Face 12: 6, 18, 19, 7, 15
        6, 18, 19, 6, 19, 7, 6, 7, 15,
    ];

    build_mesh(vertices, indices)
}

/// Apply subtle random displacement for organic feel
fn apply_noise(v: &mut Vec3, amount: f32) {
    if amount > 0.0 {
        let noise_x = (hash_f32(v.x * 1000.0) - 0.5) * amount;
        let noise_y = (hash_f32(v.y * 1000.0 + 100.0) - 0.5) * amount;
        let noise_z = (hash_f32(v.z * 1000.0 + 200.0) - 0.5) * amount;
        *v += Vec3::new(noise_x, noise_y, noise_z);
    }
}

fn hash_f32(x: f32) -> f32 {
    let x = (x * 12.9898).sin() * 43758.5453;
    x.fract()
}

/// Build mesh from vertices and indices
fn build_mesh(vertices: Vec<Vec3>, indices: Vec<u32>) -> Mesh {
    let positions: Vec<[f32; 3]> = vertices.iter().map(|v| v.to_array()).collect();

    // Calculate normals
    let mut normals = vec![[0.0, 0.0, 0.0]; vertices.len()];
    for chunk in indices.chunks(3) {
        let i0 = chunk[0] as usize;
        let i1 = chunk[1] as usize;
        let i2 = chunk[2] as usize;

        let v0 = vertices[i0];
        let v1 = vertices[i1];
        let v2 = vertices[i2];

        let normal = (v1 - v0).cross(v2 - v0).normalize();

        for &i in &[i0, i1, i2] {
            normals[i][0] += normal.x;
            normals[i][1] += normal.y;
            normals[i][2] += normal.z;
        }
    }

    // Normalize accumulated normals
    for n in normals.iter_mut() {
        let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        if len > 0.0 {
            n[0] /= len;
            n[1] /= len;
            n[2] /= len;
        }
    }

    // UVs (simple spherical projection)
    let uvs: Vec<[f32; 2]> = vertices
        .iter()
        .map(|v| {
            let n = v.normalize();
            let u = 0.5 + n.x.atan2(n.z) / std::f32::consts::TAU;
            let v = 0.5 - n.y.asin() / std::f32::consts::PI;
            [u, v]
        })
        .collect();

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}

/// Three-layer structure for traveler visuals
pub struct TravelerLayers {
    pub core: Mesh,  // Solid inner
    pub shell: Mesh, // Translucent outer
    pub edges: Mesh, // Wireframe
}

/// Create the three-layer structure for a traveler
pub fn create_traveler_layers(geometry: TravelerGeometry) -> TravelerLayers {
    let noise = 0.02; // Subtle imperfection

    TravelerLayers {
        core: generate_mesh(geometry, noise),
        shell: generate_shell_mesh(geometry, noise * 0.5),
        edges: generate_edges_mesh(geometry, noise),
    }
}

/// Generate shell mesh (slightly larger than core)
fn generate_shell_mesh(geometry: TravelerGeometry, noise: f32) -> Mesh {
    let mut mesh = generate_mesh(geometry, noise);

    // Scale up positions slightly for shell effect
    if let Some(positions) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
        if let bevy::render::mesh::VertexAttributeValues::Float32x3(ref mut pos) = positions {
            for p in pos.iter_mut() {
                p[0] *= 1.1;
                p[1] *= 1.1;
                p[2] *= 1.1;
            }
        }
    }

    mesh
}

/// Generate edge mesh (wireframe)
fn generate_edges_mesh(geometry: TravelerGeometry, noise: f32) -> Mesh {
    let base = generate_mesh(geometry, noise);

    // Extract positions
    let positions: Vec<[f32; 3]> = if let Some(attr) = base.attribute(Mesh::ATTRIBUTE_POSITION) {
        if let bevy::render::mesh::VertexAttributeValues::Float32x3(pos) = attr {
            pos.clone()
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    // Extract indices
    let indices: Vec<u32> = if let Some(ind) = base.indices() {
        match ind {
            Indices::U32(v) => v.clone(),
            Indices::U16(v) => v.iter().map(|&i| i as u32).collect(),
        }
    } else {
        vec![]
    };

    // Convert triangles to lines (unique edges)
    let mut edge_set = std::collections::HashSet::new();
    let mut edge_indices = Vec::new();

    for chunk in indices.chunks(3) {
        let edges = [(chunk[0], chunk[1]), (chunk[1], chunk[2]), (chunk[2], chunk[0])];

        for (a, b) in edges {
            let edge = if a < b { (a, b) } else { (b, a) };
            if edge_set.insert(edge) {
                edge_indices.push(a);
                edge_indices.push(b);
            }
        }
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::LineList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(Indices::U32(edge_indices));

    mesh
}

/// Cache for generated meshes to prevent duplicate generation
#[derive(Resource, Default)]
pub struct TravelerMeshCache {
    core_meshes: HashMap<TravelerGeometry, Handle<Mesh>>,
    shell_meshes: HashMap<TravelerGeometry, Handle<Mesh>>,
    edge_meshes: HashMap<TravelerGeometry, Handle<Mesh>>,
}

impl TravelerMeshCache {
    /// Get or create mesh handles for a geometry type
    pub fn get_or_create(
        &mut self,
        geometry: TravelerGeometry,
        meshes: &mut Assets<Mesh>,
    ) -> (Handle<Mesh>, Handle<Mesh>, Handle<Mesh>) {
        let core = self
            .core_meshes
            .entry(geometry)
            .or_insert_with(|| meshes.add(generate_mesh(geometry, 0.02)))
            .clone();

        let shell = self
            .shell_meshes
            .entry(geometry)
            .or_insert_with(|| meshes.add(generate_shell_mesh(geometry, 0.01)))
            .clone();

        let edges = self
            .edge_meshes
            .entry(geometry)
            .or_insert_with(|| meshes.add(generate_edges_mesh(geometry, 0.02)))
            .clone();

        (core, shell, edges)
    }
}
