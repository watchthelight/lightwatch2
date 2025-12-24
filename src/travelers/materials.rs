//! Traveler PBR materials - core, shell, edge with dynamic properties

#![allow(dead_code)]

use std::collections::HashMap;

use bevy::pbr::StandardMaterial;
use bevy::prelude::*;

use super::{TravelerDef, TravelerGrief, TravelerPulse, Traveler};
use crate::core::{ExperienceClock, Phase, TravelerId};

/// Layer type for material lookup
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TravelerLayer {
    Core,
    Shell,
    Edge,
}

/// Create the core (inner solid) material for a traveler
pub fn create_core_material(id: TravelerId) -> StandardMaterial {
    let def = TravelerDef::get(id);
    let base_srgba = def.color.base.to_srgba();

    StandardMaterial {
        base_color: def.color.base,
        emissive: LinearRgba::new(base_srgba.red, base_srgba.green, base_srgba.blue, 1.0),
        emissive_exposure_weight: 0.5,
        metallic: 0.8,
        perceptual_roughness: 0.3,
        reflectance: 0.7,
        alpha_mode: AlphaMode::Opaque,
        ..default()
    }
}

/// Create the shell (outer translucent) material
pub fn create_shell_material(id: TravelerId) -> StandardMaterial {
    let def = TravelerDef::get(id);
    let base_srgba = def.color.base.to_srgba();

    // Lighter, more transparent version
    let shell_color = Color::srgba(base_srgba.red, base_srgba.green, base_srgba.blue, 0.3);

    StandardMaterial {
        base_color: shell_color,
        emissive: LinearRgba::new(
            base_srgba.red * 0.5,
            base_srgba.green * 0.5,
            base_srgba.blue * 0.5,
            1.0,
        ),
        emissive_exposure_weight: 0.3,
        metallic: 0.2,
        perceptual_roughness: 0.1, // Smooth/glassy
        reflectance: 0.9,
        alpha_mode: AlphaMode::Blend,
        // Transmission for glass-like effect
        specular_transmission: 0.6,
        thickness: 0.5,
        ior: 1.5, // Glass-like
        ..default()
    }
}

/// Create edge (wireframe) material
pub fn create_edge_material(id: TravelerId) -> StandardMaterial {
    let def = TravelerDef::get(id);
    let base_srgba = def.color.base.to_srgba();

    // Bright, emissive edge color
    StandardMaterial {
        base_color: def.color.base,
        emissive: LinearRgba::new(
            base_srgba.red * 2.0,
            base_srgba.green * 2.0,
            base_srgba.blue * 2.0,
            1.0,
        ),
        emissive_exposure_weight: 1.0,
        unlit: true, // Pure emissive, no shading
        alpha_mode: AlphaMode::Blend,
        ..default()
    }
}

/// Cache for traveler materials
#[derive(Resource, Default)]
pub struct TravelerMaterialCache {
    materials: HashMap<(TravelerId, TravelerLayer), Handle<StandardMaterial>>,
}

impl TravelerMaterialCache {
    pub fn get_or_create(
        &mut self,
        id: TravelerId,
        layer: TravelerLayer,
        materials: &mut Assets<StandardMaterial>,
    ) -> Handle<StandardMaterial> {
        self.materials
            .entry((id, layer))
            .or_insert_with(|| {
                let mat = match layer {
                    TravelerLayer::Core => create_core_material(id),
                    TravelerLayer::Shell => create_shell_material(id),
                    TravelerLayer::Edge => create_edge_material(id),
                };
                materials.add(mat)
            })
            .clone()
    }

    /// Get all three layer handles for a traveler
    pub fn get_all_layers(
        &mut self,
        id: TravelerId,
        materials: &mut Assets<StandardMaterial>,
    ) -> (
        Handle<StandardMaterial>,
        Handle<StandardMaterial>,
        Handle<StandardMaterial>,
    ) {
        let core = self.get_or_create(id, TravelerLayer::Core, materials);
        let shell = self.get_or_create(id, TravelerLayer::Shell, materials);
        let edge = self.get_or_create(id, TravelerLayer::Edge, materials);
        (core, shell, edge)
    }
}

/// Component for materials that pulse with traveler
#[derive(Component)]
pub struct PulsingMaterial {
    pub base_emissive: LinearRgba,
    pub max_emissive: LinearRgba,
    pub layer: TravelerLayer,
}

/// Update materials based on pulse state
pub fn update_pulsing_materials(
    travelers: Query<(&TravelerPulse, &Children)>,
    children_query: Query<&PulsingMaterial>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    material_handles: Query<&Handle<StandardMaterial>>,
) {
    for (pulse, children) in travelers.iter() {
        for &child in children.iter() {
            if let Ok(pulsing) = children_query.get(child) {
                if let Ok(handle) = material_handles.get(child) {
                    if let Some(material) = materials.get_mut(handle) {
                        // Interpolate emissive based on pulse intensity
                        let t = pulse.intensity;
                        material.emissive = LinearRgba::new(
                            pulsing.base_emissive.red
                                + (pulsing.max_emissive.red - pulsing.base_emissive.red) * t,
                            pulsing.base_emissive.green
                                + (pulsing.max_emissive.green - pulsing.base_emissive.green) * t,
                            pulsing.base_emissive.blue
                                + (pulsing.max_emissive.blue - pulsing.base_emissive.blue) * t,
                            1.0,
                        );
                    }
                }
            }
        }
    }
}

/// Evolve traveler colors based on phase
pub fn evolve_materials_for_phase(
    clock: Res<ExperienceClock>,
    travelers: Query<(&Traveler, &Children)>,
    pulsing_query: Query<&PulsingMaterial>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    material_handles: Query<&Handle<StandardMaterial>>,
) {
    let phase = clock.phase();
    let phase_progress = clock.phase_progress();

    // Only evolve during certain phases
    let evolution = match phase {
        Phase::Connection => phase_progress * 0.5, // Gradual color evolution
        Phase::Acceptance => 0.5 + phase_progress * 0.5, // Continue evolution
        _ => 0.0,
    };

    if evolution < 0.01 {
        return;
    }

    for (traveler, children) in travelers.iter() {
        let def = TravelerDef::get(traveler.id);

        for &child in children.iter() {
            if let Ok(pulsing) = pulsing_query.get(child) {
                if pulsing.layer != TravelerLayer::Core {
                    continue;
                }

                if let Ok(handle) = material_handles.get(child) {
                    if let Some(material) = materials.get_mut(handle) {
                        // Lerp between base and evolved color
                        let base = def.color.base.to_srgba();
                        let evolved = def.color.evolved.to_srgba();

                        material.base_color = Color::srgba(
                            base.red + (evolved.red - base.red) * evolution,
                            base.green + (evolved.green - base.green) * evolution,
                            base.blue + (evolved.blue - base.blue) * evolution,
                            1.0,
                        );
                    }
                }
            }
        }
    }
}

/// Apply grief visual effect to materials
pub fn apply_grief_to_materials(
    travelers: Query<(&TravelerGrief, &Children), Changed<TravelerGrief>>,
    pulsing_query: Query<&PulsingMaterial>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    material_handles: Query<&Handle<StandardMaterial>>,
) {
    for (grief, children) in travelers.iter() {
        if !grief.active {
            continue;
        }

        for &child in children.iter() {
            if pulsing_query.get(child).is_ok() {
                if let Ok(handle) = material_handles.get(child) {
                    if let Some(material) = materials.get_mut(handle) {
                        // Desaturate and dim during grief
                        let current = material.base_color.to_srgba();
                        let gray = (current.red + current.green + current.blue) / 3.0;

                        let grief_amount = grief.intensity * 0.5;
                        material.base_color = Color::srgba(
                            current.red + (gray - current.red) * grief_amount,
                            current.green + (gray - current.green) * grief_amount,
                            current.blue + (gray - current.blue) * grief_amount,
                            1.0,
                        );

                        // Reduce emissive
                        let emissive_reduction = 1.0 - grief_amount * 0.5;
                        material.emissive = LinearRgba::new(
                            material.emissive.red * emissive_reduction,
                            material.emissive.green * emissive_reduction,
                            material.emissive.blue * emissive_reduction,
                            material.emissive.alpha,
                        );
                    }
                }
            }
        }
    }
}
