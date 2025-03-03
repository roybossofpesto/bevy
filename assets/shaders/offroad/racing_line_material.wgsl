#import bevy_pbr::forward_io::VertexOutput
// we can import items from shader modules in the assets folder with a quoted path
// #import "shaders/custom_material_import.wgsl"::COLOR_MULTIPLIER

@group(2) @binding(0) var material_color_texture: texture_2d<f32>;
@group(2) @binding(1) var material_color_sampler: sampler;
@group(2) @binding(2) var<uniform> material_color: vec4<f32>;
@group(2) @binding(3) var<uniform> track_length: f32;
@group(2) @binding(4) var<uniform> middle_line_width: f32;
@group(2) @binding(5) var<uniform> start_line_width: f32;
@group(2) @binding(6) var<uniform> time: f32;
@group(2) @binding(7) var<uniform> cursor_position: vec2<f32>;
@group(2) @binding(8) var<uniform> cursor_radius: f32;
@group(2) @binding(9) var<uniform> lateral_range: vec2<f32>;

const pi = radians(180.0);

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    var color = vec4(0.0);
    if mesh.uv.x < lateral_range.x || mesh.uv.x > lateral_range.y {
        let aa = textureSample(material_color_texture, material_color_sampler, mesh.uv_b);
        color = material_color * aa;
    }
    if abs(mesh.uv.x) < middle_line_width / 2.0 { 
        color = vec4(1.0);
        if fract(mesh.uv.y / track_length * 10.0 - time * 3.0) < 0.5 { color = vec4(0.0, 1.0, 0.0, 1.0); }
    }
    let radius = (3.0 + cos(2 * pi * time)) / 4.0 * cursor_radius;
    if length(mesh.uv_b - cursor_position) < radius {
        color = vec4(1.0, 1.0, 0.0, 1.0);
    }
    if mesh.uv.y < start_line_width / 2.0 || mesh.uv.y > track_length - start_line_width / 2.0 {
        color = vec4(1.0, 1.0, 1.0, 1.0);
    }
    return color;
}
