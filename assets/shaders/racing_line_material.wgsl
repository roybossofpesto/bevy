#import bevy_pbr::forward_io::VertexOutput
// we can import items from shader modules in the assets folder with a quoted path
// #import "shaders/custom_material_import.wgsl"::COLOR_MULTIPLIER

@group(2) @binding(0) var material_color_texture: texture_2d<f32>;
@group(2) @binding(1) var material_color_sampler: sampler;
@group(2) @binding(2) var<uniform> material_color: vec4<f32>;
@group(2) @binding(3) var<uniform> track_length: f32;
@group(2) @binding(4) var<uniform> line_width: f32;
@group(2) @binding(5) var<uniform> time: f32;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    var color = vec4(0.0);
    if abs(mesh.uv.x) > 0.8 {
        // let foo = mesh.position.xy / 16.0;
        let foo = mesh.uv_b;
        let aa = textureSample(material_color_texture, material_color_sampler, foo);
        color = material_color * aa;
    }
    if abs(mesh.uv.x) < line_width / 2.0 { 
        color = vec4(1.0);
        if fract(mesh.uv.y / track_length * 10.0 - time * 3.0) < 0.5 { color = vec4(0.0, 1.0, 0.0, 1.0); }
    }
    return color;
}
