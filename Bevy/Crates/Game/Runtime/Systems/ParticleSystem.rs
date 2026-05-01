use bevy::prelude::*;
use bevy_hanabi::prelude::*;

use crate::{
    enemy_component::EnemyComponent, particle_component::ParticleComponent,
    particle_resource::ParticleResource, player_component::PlayerComponent,
};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum ParticleType {
    SmokeTrail,
}

struct ParticleTypeConfig {
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

impl ParticleType {
    fn config(self) -> ParticleTypeConfig {
        match self {
            ParticleType::SmokeTrail => ParticleTypeConfig {
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

const PARTICLE_TYPES: [ParticleType; 1] = [ParticleType::SmokeTrail];

// System creates shared Hanabi effect assets for reusable particle types.
pub fn particle_startup_system(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let effects = PARTICLE_TYPES
        .into_iter()
        .map(|particle_type| (particle_type, effects.add(particle_type.effect())))
        .collect();

    commands.insert_resource(ParticleResource::new(effects));
}

// System attaches configured particle emitters to live game entities.
pub fn particle_attach_system(
    mut commands: Commands,
    particles: Option<Res<ParticleResource>>,
    plane_query: Query<
        (Entity, Option<&Name>, Option<&ParticleComponent>),
        Or<(With<PlayerComponent>, With<EnemyComponent>)>,
    >,
) {
    let Some(particles) = particles else {
        return;
    };

    for (plane_entity, plane_name, particle_component) in &plane_query {
        if particle_component
            .is_some_and(|particle| particle.particle_type == ParticleType::SmokeTrail)
        {
            continue;
        }

        particle_attach_to_entity(
            &mut commands,
            &particles,
            plane_entity,
            plane_name,
            ParticleType::SmokeTrail,
        );
    }
}

fn particle_attach_to_entity(
    commands: &mut Commands,
    particles: &ParticleResource,
    entity: Entity,
    entity_name: Option<&Name>,
    particle_type: ParticleType,
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
        .insert(ParticleComponent { particle_type });
}

impl ParticleType {
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
