use bevy::{
    pbr::Material, prelude::*, reflect::TypePath, render::render_resource::AsBindGroup,
    shader::ShaderRef,
};

// This custom material is kept as a scaffold for future bullet visuals.
// It is intentionally not wired into the active bullet pipeline yet.
#[allow(dead_code)]
#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct BulletShader {
    #[uniform(0)]
    pub age_seconds: f32,
}

impl Material for BulletShader {
    fn fragment_shader() -> ShaderRef {
        "Shaders/BulletShader.wgsl".into()
    }
}
