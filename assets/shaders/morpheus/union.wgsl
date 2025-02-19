#import bevy_pbr::{
    mesh_functions,
    view_transformations,
}

@group(2) @binding(0) var matcap_texture: texture_2d<f32>;
@group(2) @binding(1) var matcap_sampler: sampler;

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

    if dist > 1e-3 {
        return vec4(0.0);
    }

    let hh = 1e-3;
    let world_grad = normalize(vec3(
        signed_disance_function(pos + vec3(hh, 0.0, 0.0)) - signed_disance_function(pos - vec3(hh, 0.0, 0.0)), 
        signed_disance_function(pos + vec3(0.0, hh, 0.0)) - signed_disance_function(pos - vec3(0.0, hh, 0.0)), 
        signed_disance_function(pos + vec3(0.0, 0.0, hh)) - signed_disance_function(pos - vec3(0.0, 0.0, hh)), 
    ));
    let view_grad = normalize(view_transformations::direction_world_to_view(world_grad));
    
    var color = textureSample(matcap_texture, matcap_sampler, (view_grad.xy + 1.0) / 2.0);

    /*
    let light_pos = vec3(-4.0, 16.0, 8.0);
    var light_dir = light_pos - pos;
    light_dir /= length(light_dir);
    let shadow_factor = clamp(dot(world_grad, light_dir), 0.2, 1.0);
    color = vec4(color.xyz * shadow_factor, color.w);
    */
    
    return color;
}

fn signed_disance_function(pos: vec3<f32>) -> f32 {
    let aa = length(pos - vec3(.2, 0, 0)) - 0.6;
    let bb = length(pos + vec3(.2, 0, 0)) - 0.6;
    return min(aa, bb);
}
