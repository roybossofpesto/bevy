// The shader reads the previous frame's state from the `input` texture, and writes the new state of
// each pixel to the `output` texture. The textures are flipped each step to progress the
// simulation.
// Two textures are needed for the game of life as each pixel of step N depends on the state of its
// neighbors at step N-1.


struct Settings {
    rng_seed: u32,
}

@group(0) @binding(0) var input: texture_storage_2d<rgba32float, read>;
@group(0) @binding(1) var output: texture_storage_2d<rgba32float, write>;
@group(0) @binding(2) var<uniform> settings: Settings;

fn hash(value: u32) -> u32 {
    var state = value;
    // state += settings.rng_seed;
    state = state ^ 2747636419u;
    state = state * 2654435769u;
    state = state ^ state >> 16u;
    state = state * 2654435769u;
    state = state ^ state >> 16u;
    state = state * 2654435769u;
    return state;
}

fn random_float_aa(value: u32) -> f32 {
    return f32(hash(value)) / 4294967295.0;
}

fn random_float_bb(value: u32) -> f32 {
    return f32(hash(hash(value))) / 4294967295.0;
}

@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));

    let aa = random_float_aa(invocation_id.y << 16u | invocation_id.x);
    let bb = random_float_bb(invocation_id.y << 16u | invocation_id.x);
    let aa_alive = aa > 0.9;
    let bb_alive = bb > 0.5;

    let color = vec4<f32>(f32(aa_alive), f32(aa_alive), f32(bb_alive), 1.0);

    textureStore(output, location, color);
}

fn is_alive_aa(location: vec2<i32>, offset_x: i32, offset_y: i32) -> u32 {
    let value: vec4<f32> = textureLoad(input, location + vec2<i32>(offset_x, offset_y));
    return u32(value.x);
}

fn is_alive_bb(location: vec2<i32>, offset_x: i32, offset_y: i32) -> u32 {
    let value: vec4<f32> = textureLoad(input, location + vec2<i32>(offset_x, offset_y));
    return u32(value.z);
}

fn count_alive_neighbors_aa(location: vec2<i32>) -> u32 {
    return
        is_alive_aa(location, -1, -1) +
        is_alive_aa(location, -1,  0) +
        is_alive_aa(location, -1,  1) +
        is_alive_aa(location,  0, -1) +
        is_alive_aa(location,  0,  1) +
        is_alive_aa(location,  1, -1) +
        is_alive_aa(location,  1,  0) +
        is_alive_aa(location,  1,  1);
}

fn count_alive_neighbors_bb(location: vec2<i32>) -> u32 {
    return
        is_alive_bb(location, -1, -1) +
        is_alive_bb(location, -1,  0) +
        is_alive_bb(location, -1,  1) +
        is_alive_bb(location,  0, -1) +
        is_alive_bb(location,  0,  1) +
        is_alive_bb(location,  1, -1) +
        is_alive_bb(location,  1,  0) +
        is_alive_bb(location,  1,  1);
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));

    var color : vec4<f32> = textureLoad(input, location);

    {
        let num_alive_neighbors = count_alive_neighbors_aa(location);
    
        var next_alive: bool = false;
        if (num_alive_neighbors == 3) {
            next_alive = true;
        } else if (num_alive_neighbors == 2) {
            let current_alive = bool(is_alive_aa(location, 0, 0));
            next_alive = current_alive;
        } else {
            next_alive = false;
        }
    
        color.x = f32(next_alive);
    }

    {
        let num_alive_neighbors = count_alive_neighbors_bb(location);
    
        var next_alive: bool = false;
        if (num_alive_neighbors == 3) {
            next_alive = true;
        } else if (num_alive_neighbors == 2) {
            let current_alive = bool(is_alive_bb(location, 0, 0));
            next_alive = current_alive;
        } else {
            next_alive = false;
        }
    
        color.z = f32(next_alive);
    }

    textureStore(output, location, color);
}
