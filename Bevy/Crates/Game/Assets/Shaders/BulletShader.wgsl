#import bevy_pbr::forward_io::VertexOutput

struct BulletShader {
    age_seconds: f32,
};

@group(2) @binding(0)
var<storage, read> material: BulletShader;

const FADE_DURATION_SECONDS: f32 = 0.25;

@fragment
fn fragment(_in: VertexOutput) -> @location(0) vec4<f32> {
    let fade = clamp(material.age_seconds / FADE_DURATION_SECONDS, 0.0, 1.0);
    let value = 1.0 - fade;
    return vec4<f32>(vec3<f32>(value), 1.0);
}
