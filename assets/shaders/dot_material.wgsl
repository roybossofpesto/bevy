#import bevy_pbr::forward_io::VertexOutput
// we can import items from shader modules in the assets folder with a quoted path
// #import "shaders/custom_material_import.wgsl"::COLOR_MULTIPLIER

@group(2) @binding(0) var material_color_texture: texture_2d<f32>;
@group(2) @binding(1) var material_color_sampler: sampler;
@group(2) @binding(2) var<uniform> material_color: vec4<f32>;
@group(2) @binding(3) var<uniform> track_length: f32;
@group(2) @binding(4) var<uniform> track_threshold: f32;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    // let uv = mesh.uv;
    // uv /= 2.0;
    // return material_color * ;
    let aa = textureSample(material_color_texture, material_color_sampler, mesh.uv);
    var color = material_color * aa;
    if mesh.uv.x < 0 { color = vec4(0.0); }
    if mesh.uv.x < -0.8 { 
        color = vec4(1.0);
        if mesh.uv.y < track_length / 2.0 { color = vec4(0.0, 1.0, 0.0, 1.0); }
        if mesh.uv.y < track_threshold { color = vec4(1.0, 0.0, 0.0, 1.0); }
    }
    return color;
}
