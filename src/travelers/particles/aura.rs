//! Aura particles - orbiting motes of light around travelers

#![allow(dead_code)]

use bevy::prelude::*;
use rand::Rng;

use crate::core::TravelerId;
use crate::travelers::{Traveler, TravelerPulse, TravelerState, TravelerVisibility};

/// Aura particle system configuration
#[derive(Component, Debug)]
pub struct TravelerAura {
    /// Which traveler this belongs to
    pub traveler_id: TravelerId,
    /// Number of particles
    pub particle_count: usize,
    /// Orbital radius (min, max)
    pub radius: (f32, f32),
    /// Orbital speed
    pub speed: f32,
    /// Particle size
    pub size: f32,
    /// Base color
    pub color: Color,
    /// Density multiplier (affected by pulse)
    pub density: f32,
}

impl TravelerAura {
    pub fn for_traveler(id: TravelerId) -> Self {
        match id {
            TravelerId::Archivist => Self {
                traveler_id: id,
                particle_count: 60,
                radius: (1.2, 2.0),
                speed: 0.3,
                size: 0.04,
                color: Color::srgba(0.91, 0.64, 0.27, 0.6),
                density: 1.0,
            },
            TravelerId::Wanderer => Self {
                traveler_id: id,
                particle_count: 40,
                radius: (1.0, 2.5),
                speed: 0.5, // Faster, more erratic
                size: 0.05,
                color: Color::srgba(0.31, 0.80, 0.77, 0.5),
                density: 1.0,
            },
            TravelerId::Keeper => Self {
                traveler_id: id,
                particle_count: 30,
                radius: (1.5, 1.8),
                speed: 0.15, // Slow, steady
                size: 0.06,
                color: Color::srgba(0.83, 0.46, 0.18, 0.7),
                density: 1.0,
            },
            TravelerId::Child => Self {
                traveler_id: id,
                particle_count: 80, // Many particles
                radius: (0.8, 2.2),
                speed: 0.7, // Fast, playful
                size: 0.03,
                color: Color::srgba(0.96, 0.94, 0.91, 0.4),
                density: 1.0,
            },
            TravelerId::Other => Self {
                traveler_id: id,
                particle_count: 20, // Sparse
                radius: (2.0, 3.5),
                speed: 0.08, // Very slow
                size: 0.08,
                color: Color::srgba(0.42, 0.36, 0.58, 0.3),
                density: 0.5,
            },
        }
    }
}

/// Individual particle state
#[derive(Component, Debug)]
pub struct AuraParticle {
    /// Orbital angle
    pub angle: f32,
    /// Orbital radius
    pub radius: f32,
    /// Height offset
    pub height: f32,
    /// Speed multiplier
    pub speed_mult: f32,
    /// Orbital plane tilt
    pub tilt: Quat,
    /// Phase offset for animation
    pub phase: f32,
}

/// Marker for aura particle entities
#[derive(Component)]
pub struct AuraParticleMarker;

/// Calculate particle position in orbit
fn calculate_particle_position(angle: f32, radius: f32, height: f32, tilt: Quat) -> Vec3 {
    let base_pos = Vec3::new(angle.cos() * radius, height, angle.sin() * radius);
    tilt * base_pos
}

/// Spawn aura particles when traveler is created
pub fn spawn_aura_particles(
    mut commands: Commands,
    travelers: Query<(Entity, &Traveler), Added<Traveler>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, traveler) in travelers.iter() {
        let aura = TravelerAura::for_traveler(traveler.id);
        let mut rng = rand::thread_rng();

        // Create particle mesh (small sphere)
        let particle_mesh = meshes.add(Sphere::new(aura.size).mesh().ico(1).unwrap());

        // Create particle material
        let base_srgba = aura.color.to_srgba();
        let particle_material = materials.add(StandardMaterial {
            base_color: aura.color,
            emissive: LinearRgba::new(base_srgba.red, base_srgba.green, base_srgba.blue, 1.0),
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            ..default()
        });

        // Spawn particles as children
        commands.entity(entity).with_children(|parent| {
            for i in 0..aura.particle_count {
                let angle = rng.gen::<f32>() * std::f32::consts::TAU;
                let radius =
                    aura.radius.0 + rng.gen::<f32>() * (aura.radius.1 - aura.radius.0);
                let height = (rng.gen::<f32>() - 0.5) * 1.5;
                let speed_mult = 0.7 + rng.gen::<f32>() * 0.6;

                // Random orbital plane tilt
                let tilt = Quat::from_euler(
                    EulerRot::XYZ,
                    (rng.gen::<f32>() - 0.5) * 0.5,
                    (rng.gen::<f32>() - 0.5) * 0.5,
                    0.0,
                );

                let position = calculate_particle_position(angle, radius, height, tilt);

                parent.spawn((
                    PbrBundle {
                        mesh: particle_mesh.clone(),
                        material: particle_material.clone(),
                        transform: Transform::from_translation(position),
                        ..default()
                    },
                    AuraParticle {
                        angle,
                        radius,
                        height,
                        speed_mult,
                        tilt,
                        phase: i as f32 / aura.particle_count as f32 * std::f32::consts::TAU,
                    },
                    AuraParticleMarker,
                ));
            }
        });

        // Add aura component to traveler
        commands.entity(entity).insert(aura);
    }
}

/// Animate aura particles
pub fn animate_aura_particles(
    time: Res<Time>,
    travelers: Query<(&TravelerAura, &TravelerPulse, &TravelerVisibility)>,
    mut particles: Query<(&Parent, &mut Transform, &mut AuraParticle), With<AuraParticleMarker>>,
) {
    let t = time.elapsed_seconds();

    for (parent, mut transform, mut particle) in particles.iter_mut() {
        let Ok((aura, pulse, vis)) = travelers.get(parent.get()) else {
            continue;
        };

        // Update angle based on speed
        particle.angle += aura.speed * particle.speed_mult * time.delta_seconds();

        // Radius oscillation based on pulse
        let pulse_offset = (t * 2.0 + particle.phase).sin() * 0.1 * pulse.intensity;
        let current_radius = particle.radius + pulse_offset;

        // Height oscillation
        let height_offset = (t * 1.5 + particle.phase * 0.7).sin() * 0.2;
        let current_height = particle.height + height_offset;

        // Calculate new position
        let position =
            calculate_particle_position(particle.angle, current_radius, current_height, particle.tilt);

        transform.translation = position;

        // Scale based on pulse and visibility
        let scale = aura.size * (0.8 + pulse.intensity * 0.4) * vis.opacity;
        transform.scale = Vec3::splat(scale.max(0.001)); // Prevent zero scale
    }
}

/// Control particle visibility based on density
pub fn control_particle_density(
    travelers: Query<(&TravelerAura, &TravelerPulse)>,
    mut particles: Query<(&Parent, &mut Visibility, &AuraParticle), With<AuraParticleMarker>>,
) {
    for (parent, mut visibility, particle) in particles.iter_mut() {
        let Ok((aura, pulse)) = travelers.get(parent.get()) else {
            continue;
        };

        // Density increases with pulse
        let effective_density = aura.density * (0.5 + pulse.intensity * 0.5);

        // Use phase to determine visibility threshold
        let threshold = particle.phase / std::f32::consts::TAU;
        let should_show = threshold < effective_density;

        *visibility = if should_show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

/// Fade particles when traveler is fading
pub fn fade_aura_with_traveler(
    travelers: Query<(&TravelerAura, &TravelerVisibility, &TravelerState)>,
    particles: Query<(&Parent, &Handle<StandardMaterial>), With<AuraParticleMarker>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (parent, material_handle) in particles.iter() {
        let Ok((aura, vis, state)) = travelers.get(parent.get()) else {
            continue;
        };

        if *state == TravelerState::Fading || *state == TravelerState::Gone {
            if let Some(material) = materials.get_mut(material_handle) {
                let base_srgba = aura.color.to_srgba();
                let alpha = base_srgba.alpha * vis.opacity;
                material.base_color = Color::srgba(
                    base_srgba.red,
                    base_srgba.green,
                    base_srgba.blue,
                    alpha,
                );
                material.emissive = LinearRgba::new(
                    base_srgba.red * vis.opacity,
                    base_srgba.green * vis.opacity,
                    base_srgba.blue * vis.opacity,
                    1.0,
                );
            }
        }
    }
}
