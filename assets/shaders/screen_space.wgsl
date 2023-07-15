#import bevy_sprite::mesh2d_view_bindings globals

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;

@fragment
fn fragment(
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
) -> @location(0) vec4<f32> {
    return textureSample(texture, texture_sampler, vec2<f32>(fract(world_position.x), fract(world_position.y)));
}
