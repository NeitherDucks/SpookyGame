// var is mutable variable.
// let is constant variable.
// const is compile-constant variable.

#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var color_texture: texture_2d<f32>;
@group(2) @binding(1) var texture_sampler: sampler;
// @group(2) @binding(2) var lights_texture: texture_2d<f32>;
// @group(2) @binding(3) var texture_sampler: sampler;
// @group(2) @binding(4) var height_texture: texture_2d<f32>;
// @group(2) @binding(5) var texture_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv = mesh.uv;

    var color = textureSample(color_texture, texture_sampler, uv);
    // var height = textureSample(height_texture, texture_sampler, uv);
    // var light = textureSample(lights_texture, texture_sampler, uv) * height;

    // let lum = light.r * 0.8;
    // let alpha = vec4<f32>(light.r, light.r, light.r, 1.0);
    let night = vec4<f32>(0.01201 * 0.5, 0.01918 * 0.5, 0.13108 * 0.5, 1.0);

    // Hard light blend, to make it "night time"
    let mult = night * color;
    let screen = night + color - mult;

    var hard_light = screen;

    if length(night) < 0.5 * 2 {
        hard_light = mult;
    }
    
    // // Soft light blend, to light up where the enemies are looking.
    // let mult2 = light * hard_light;
    // var soft_light = hard_light;

    // if length(mult2) < 1.0 {
    //     soft_light = 2.0 * mult2;
    // } else {
    //     soft_light = hard_light * (2.0 * light + (hard_light * (1.0 - mult2)));
    // }

    return hard_light;
}