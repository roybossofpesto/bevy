// #import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::{
    mesh_functions,
    view_transformations,
}
#import bevy_pbr::mesh_view_bindings as view_bindings


@group(2) @binding(0) var color_texture: texture_2d<f32>;
@group(2) @binding(1) var color_sampler: sampler;
@group(2) @binding(2) var<uniform> cursor_position: vec2<f32>;
@group(2) @binding(3) var<uniform> cursor_radius: f32;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    var world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4(vertex.position, 1.0)).xyz;
    out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal, vertex.instance_index);
    out.clip_position = view_transformations::position_world_to_clip(out.world_position);
    out.uv = vertex.uv;
    return out;
}

@fragment
fn fragment(
    out: VertexOutput,
) -> @location(0) vec4<f32> {
    let eye_position = view_transformations::position_ndc_to_world(vec3(0.0, 0.0, -1.0));
    let world_direction = normalize(out.world_position - eye_position);

    var pos = out.world_position;
    var dist = signed_disance_function(pos);
    for (var kk=0; kk<64; kk++) {
        if (dist <= 0.0) { break; }
        if (length(pos) > sqrt(3.0)) { break; }
        pos += world_direction * dist;
        dist = signed_disance_function(pos);
    }

    var color = textureSample(color_texture, color_sampler, out.uv);

    if color.w == 0 && dist <= 1e-3 {
        let hh = 1e-3;
        let world_grad = normalize(vec3(
            signed_disance_function(pos + vec3(hh, 0.0, 0.0)) - signed_disance_function(pos - vec3(hh, 0.0, 0.0)), 
            signed_disance_function(pos + vec3(0.0, hh, 0.0)) - signed_disance_function(pos - vec3(0.0, hh, 0.0)), 
            signed_disance_function(pos + vec3(0.0, 0.0, hh)) - signed_disance_function(pos - vec3(0.0, 0.0, hh)), 
        ));
        let view_grad = view_transformations::direction_world_to_view(world_grad);
        let light_pos = vec3(-4.0, 16.0, 8.0);
        var light_dir = light_pos - pos;
        light_dir /= length(light_dir);
        let shadow_factor = clamp(dot(world_grad, light_dir), 0.2, 1.0);
        color = vec4((normalize(view_grad) + 1.0) / 2.0 * shadow_factor, 1.0);
    }

    if length(out.uv - cursor_position) < cursor_radius {
        color = vec4(1.0, 1.0, 0.0, 1.0);
    }

    return color;
}

fn signed_disance_function(pos: vec3<f32>) -> f32 {
    return length(pos) - 0.8;
}
