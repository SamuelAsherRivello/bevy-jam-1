use bevy::prelude::*;
use bevy_hanabi::prelude::*;

use crate::{
    particles_advanced_component::ParticlesAdvancedComponent,
    particles_advanced_resource::ParticlesAdvancedResource, plane_component::PlaneComponent,
};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum ParticlesAdvancedType {
    SmokeTrail,
}

struct ParticlesAdvancedTypeConfig {
    capacity: u32,
    particles_per_second: f32,
    local_offset: Vec3,
    emit_radius: f32,
    velocity: Vec3,
    drag: f32,
    min_lifetime_seconds: f32,
    max_lifetime_seconds: f32,
    start_size: f32,
    end_size: f32,
    roundness: f32,
    name: &'static str,
}

impl ParticlesAdvancedType {
    fn config(self) -> ParticlesAdvancedTypeConfig {
        match self {
            ParticlesAdvancedType::SmokeTrail => ParticlesAdvancedTypeConfig {
                capacity: 512,
                particles_per_second: 36.0,
                local_offset: Vec3::new(0.0, 0.42, -0.96),
                emit_radius: 0.1,
                velocity: Vec3::new(0.0, 0.3, -2.8),
                drag: 1.6,
                min_lifetime_seconds: 0.8,
                max_lifetime_seconds: 1.25,
                start_size: 0.08,
                end_size: 0.38,
                roundness: 1.0,
                name: "SmokeTrail",
            },
        }
    }

    fn display_name(self) -> &'static str {
        self.config().name
    }
}

const PARTICLES_ADVANCED_TYPES: [ParticlesAdvancedType; 1] = [ParticlesAdvancedType::SmokeTrail];

// System creates shared Hanabi effect assets for reusable particle types.
pub fn particles_advanced_startup_system(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let effects = PARTICLES_ADVANCED_TYPES
        .into_iter()
        .map(|particle_type| (particle_type, effects.add(particle_type.effect())))
        .collect();

    commands.insert_resource(ParticlesAdvancedResource::new(effects));
}

// System attaches configured particle emitters to live game entities.
pub fn particles_advanced_attach_system(
    mut commands: Commands,
    particles: Option<Res<ParticlesAdvancedResource>>,
    plane_query: Query<
        (Entity, Option<&Name>, Option<&ParticlesAdvancedComponent>),
        With<PlaneComponent>,
    >,
) {
    let Some(particles) = particles else {
        return;
    };

    for (plane_entity, plane_name, particle_component) in &plane_query {
        if particle_component
            .is_some_and(|particle| particle.particle_type == ParticlesAdvancedType::SmokeTrail)
        {
            continue;
        }

        particle_attach_to_entity(
            &mut commands,
            &particles,
            plane_entity,
            plane_name,
            ParticlesAdvancedType::SmokeTrail,
        );
    }
}

fn particle_attach_to_entity(
    commands: &mut Commands,
    particles: &ParticlesAdvancedResource,
    entity: Entity,
    entity_name: Option<&Name>,
    particle_type: ParticlesAdvancedType,
) {
    let Some(effect) = particles.effect(particle_type) else {
        return;
    };

    let config = particle_type.config();
    let particle_name = match entity_name {
        Some(name) => format!("{name} {}", particle_type.display_name()),
        None => format!("Entity {}", particle_type.display_name()),
    };

    let particle_entity = commands
        .spawn((
            Name::new(particle_name),
            ParticleEffect::new(effect.clone()),
            Transform::from_translation(config.local_offset),
        ))
        .id();

    commands
        .entity(entity)
        .add_child(particle_entity)
        .insert(ParticlesAdvancedComponent { particle_type });
}

impl ParticlesAdvancedType {
    fn effect(self) -> EffectAsset {
        let config = self.config();
        let writer = ExprWriter::new();

        let init_pos = SetPositionSphereModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            radius: writer.lit(config.emit_radius).expr(),
            dimension: ShapeDimension::Volume,
        };

        let velocity = writer.lit(config.velocity).expr();
        let init_velocity = SetAttributeModifier::new(Attribute::VELOCITY, velocity);

        let age = writer.lit(0.0).expr();
        let init_age = SetAttributeModifier::new(Attribute::AGE, age);

        let lifetime = writer
            .lit(config.min_lifetime_seconds)
            .uniform(writer.lit(config.max_lifetime_seconds))
            .expr();
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

        let drag = writer.lit(config.drag).expr();
        let update_drag = LinearDragModifier::new(drag);

        let mut color_gradient = bevy_hanabi::Gradient::new();
        color_gradient.add_key(0.0, Vec4::new(1.0, 1.0, 1.0, 0.0));
        color_gradient.add_key(0.15, Vec4::new(1.0, 1.0, 1.0, 0.6));
        color_gradient.add_key(0.75, Vec4::new(1.0, 1.0, 1.0, 0.38));
        color_gradient.add_key(1.0, Vec4::new(1.0, 1.0, 1.0, 0.0));

        let mut size_gradient = bevy_hanabi::Gradient::new();
        size_gradient.add_key(0.0, Vec3::splat(config.start_size));
        size_gradient.add_key(1.0, Vec3::splat(config.end_size));

        let mut module = writer.finish();
        let round = RoundModifier::constant(&mut module, config.roundness);

        EffectAsset::new(
            config.capacity,
            SpawnerSettings::rate(config.particles_per_second.into()),
            module,
        )
        .with_name(config.name)
        .init(init_pos)
        .init(init_velocity)
        .init(init_age)
        .init(init_lifetime)
        .update(update_drag)
        .render(ColorOverLifetimeModifier {
            gradient: color_gradient,
            blend: ColorBlendMode::Overwrite,
            mask: ColorBlendMask::RGBA,
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient,
            screen_space_size: false,
        })
        .render(round)
    }
}
