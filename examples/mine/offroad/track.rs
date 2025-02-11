use bevy::prelude::Vec3Swizzles;
use bevy::prelude::{debug, info, warn};

use bevy::math::NormedVectorSpace;
use bevy::math::{Mat3, Quat, Vec2, Vec3};

use std::f32::consts::PI;

pub enum TrackPiece {
    Start,
    Straight(StraightData),
    Corner(CornerData),
    Finish,
}

#[derive(Debug)]
pub struct StraightData {
    left: f32,
    right: f32,
    length: f32,
}

impl StraightData {
    const fn default() -> Self {
        Self {
            left: -1.0,
            right: 1.0,
            length: 2.0,
        }
    }
    const fn from_length(length: f32) -> Self {
        Self {
            length,
            ..StraightData::default()
        }
    }
    const fn from_left_right(left: f32, right: f32) -> Self {
        Self {
            left,
            right,
            ..StraightData::default()
        }
    }
    const fn from_left_right_length(left: f32, right: f32, length: f32) -> Self {
        Self {
            left,
            right,
            length,
        }
    }
}

#[derive(Debug)]
pub struct CornerData {
    radius: f32,
    angle: f32,
    num_quads: u32,
}

impl CornerData {
    const fn right_turn() -> Self {
        Self {
            radius: 2.0,
            angle: PI / 2.0,
            num_quads: 8,
        }
    }
    const fn left_turn() -> Self {
        Self {
            radius: -2.0,
            angle: PI / 2.0,
            num_quads: 8,
        }
    }
}

pub struct TrackData {
    pieces: &'static [TrackPiece],
    initial_position: Vec3,
    initial_forward: Vec3,
    initial_up: Vec3,
    initial_left: f32,
    initial_right: f32,
    num_segments: u32,
}

pub fn make_track_mesh(track_data: &TrackData) -> bevy::render::mesh::Mesh {
    assert!(f32::abs(track_data.initial_forward.norm() - 1.0) < 1e-5);
    assert!(f32::abs(track_data.initial_up.norm() - 1.0) < 1e-5);
    assert!(track_data.initial_left < track_data.initial_right);
    assert!(track_data.num_segments > 0);
    assert!(track_data.pieces.len() >= 2);
    match &track_data.pieces[0] {
        TrackPiece::Start => {}
        _ => assert!(false, "!!! first piece should be a start !!!"),
    }
    match &track_data.pieces[track_data.pieces.len() - 1] {
        TrackPiece::Finish => {}
        _ => assert!(false, "!!! last piece should be a finish !!!"),
    }

    let initial_righthand = track_data.initial_forward.cross(track_data.initial_up);

    let mut mesh_positions: Vec<Vec3> = vec![];
    let mut mesh_normals: Vec<Vec3> = vec![];
    let mut mesh_triangles: Vec<u32> = vec![];
    let mut mesh_uvs: Vec<Vec2> = vec![];
    let mut mesh_pqs: Vec<Vec2> = vec![];
    let mut push_section =
        |position: &Vec3, forward: &Vec3, left: f32, right: f32, length: f32| -> u32 {
            let left_pos = position + forward.cross(track_data.initial_up) * left;
            let right_pos = position + forward.cross(track_data.initial_up) * right;
            let next_vertex = mesh_positions.len() as u32;
            let num_segments = track_data.num_segments;
            assert!(next_vertex % (num_segments + 1) == 0);
            for kk in 0..=num_segments {
                let aa = kk as f32 / num_segments as f32;
                assert!(aa >= 0.0);
                assert!(aa <= 1.0);
                let pos = aa * right_pos + (1.0 - aa) * left_pos;
                let uv = Vec2::new(aa * right + (1.0 - aa) * left, length);
                let proj =
                    Mat3::from_cols(initial_righthand, track_data.initial_forward, Vec3::ZERO);
                let pq = proj * (pos - track_data.initial_position);
                assert!(f32::abs(pq.z) < 1e-5);
                mesh_positions.push(pos);
                mesh_normals.push(track_data.initial_up);
                mesh_uvs.push(uv);
                mesh_pqs.push(pq.xy());
            }
            if next_vertex != 0 {
                assert!(next_vertex >= (num_segments + 1));
                for kk in 0..num_segments {
                    let mut tri_aa = vec![
                        next_vertex + kk - num_segments - 1,
                        next_vertex + kk - num_segments,
                        next_vertex + kk,
                    ];
                    // let mut tri_bb = vec![next_vertex - 1, next_vertex + 1, next_vertex];
                    let mut tri_bb = vec![
                        next_vertex + kk - num_segments,
                        next_vertex + kk + 1,
                        next_vertex + kk,
                    ];
                    mesh_triangles.append(&mut tri_aa);
                    mesh_triangles.append(&mut tri_bb);
                }
            }
            next_vertex
        };

    let mut current_position = track_data.initial_position.clone();
    let mut current_forward = track_data.initial_forward.clone();
    let mut current_length: f32 = 0.0;
    let mut is_looping: bool = false;
    let mut current_left: f32 = track_data.initial_left;
    let mut current_right: f32 = track_data.initial_right;
    for piece in track_data.pieces {
        match piece {
            TrackPiece::Start => {
                debug!("Start {:?}", current_position.clone());
                assert!(current_length == 0.0);
                assert!(current_left == track_data.initial_left);
                assert!(current_right == track_data.initial_right);
                assert!(current_left < current_right);
                let foo = push_section(
                    &current_position,
                    &current_forward,
                    current_left,
                    current_right,
                    current_length,
                );
                assert!(foo == 0);
            }
            TrackPiece::Straight(data) => {
                debug!("Straight {:?} {:?}", current_position.clone(), data);
                current_position += current_forward * data.length;
                current_length += data.length;
                assert!(current_length != 0.0);
                assert!(current_left < current_right);
                current_left = data.left;
                current_right = data.right;
                assert!(current_left < current_right);
                let foo = push_section(
                    &current_position,
                    &current_forward,
                    current_left,
                    current_right,
                    current_length,
                );
                assert!(foo > 0);
            }
            TrackPiece::Corner(data) => {
                debug!("Corner {:?} {:?}", current_position.clone(), data);
                assert!(current_left < current_right);
                assert!(data.num_quads > 0);
                let current_righthand = current_forward.cross(track_data.initial_up);
                let center = current_position + current_righthand * data.radius;
                let sign: f32 = if data.radius < 0.0 { 1.0 } else { -1.0 };
                for kk in 0..data.num_quads {
                    let angle = (kk + 1) as f32 / data.num_quads as f32 * data.angle;
                    let pos = center + current_forward * f32::abs(data.radius) * f32::sin(angle)
                        - current_righthand * data.radius * f32::cos(angle);
                    let quat = Quat::from_axis_angle(track_data.initial_up, sign * angle);
                    let fwd = quat * current_forward;
                    let len = f32::abs(data.radius) * angle + current_length;
                    let foo = push_section(&pos, &fwd, current_left, current_right, len);
                    assert!(foo > 0);
                }
                current_position = center
                    + current_forward * f32::abs(data.radius) * f32::sin(data.angle)
                    - current_righthand * data.radius * f32::cos(data.angle);
                let quat = Quat::from_axis_angle(track_data.initial_up, sign * data.angle);
                current_forward = quat * current_forward;
                current_length += f32::abs(data.radius) * data.angle;
                assert!(current_length != 0.0);
            }
            TrackPiece::Finish => {
                let pos_error = (current_position - track_data.initial_position).norm();
                let dir_error = (current_forward - track_data.initial_forward).norm();
                let left_error = f32::abs(current_left - track_data.initial_left);
                let right_error = f32::abs(current_left - track_data.initial_left);
                let eps: f32 = 1e-3;
                is_looping = pos_error < eps
                    && dir_error < eps
                    && left_error < eps
                    && right_error < eps
                    && current_length > 0.0;
                debug!(
                    "Finish {:?} pos_err {:0.3e} dir_err {:0.3e} total_length {} loop {}",
                    current_position.clone(),
                    pos_error,
                    dir_error,
                    current_length,
                    is_looping,
                );
            }
        }
    }

    assert!(mesh_triangles.len() % 3 == 0);
    info!("num_vertices {}", mesh_positions.len());
    info!("num_triangles {}", mesh_triangles.len() / 3);
    info!("total_length {}", current_length);
    if !is_looping {
        warn!("!!! road is not looping !!!");
    }

    use bevy::render::mesh::Indices;
    use bevy::render::mesh::Mesh;
    use bevy::render::render_asset::RenderAssetUsages;
    use bevy::render::render_resource::PrimitiveTopology;

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    mesh = mesh.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_positions);
    mesh = mesh.with_inserted_indices(Indices::U32(mesh_triangles));
    mesh = mesh.with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_normals);
    mesh = mesh.with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_uvs);
    mesh = mesh.with_inserted_attribute(Mesh::ATTRIBUTE_UV_1, mesh_pqs);

    mesh
}

static TRACK0_PIECES: [TrackPiece; 15] = [
    TrackPiece::Start,
    TrackPiece::Straight(StraightData::default()),
    TrackPiece::Corner(CornerData::left_turn()),
    TrackPiece::Straight(StraightData::from_length(8.0)),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Straight(StraightData::default()),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Straight(StraightData::from_length(14.0)),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Straight(StraightData::from_length(11.0)),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Straight(StraightData::default()),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Straight(StraightData::from_length(3.0)),
    TrackPiece::Finish,
];

pub static TRACK0_DATA: TrackData = TrackData {
    pieces: &TRACK0_PIECES,
    initial_position: Vec3::new(-12.0, 0.25, 0.0),
    initial_forward: Vec3::Z,
    initial_up: Vec3::Y,
    initial_left: -1.0,
    initial_right: 1.0,
    num_segments: 4,
};

static TRACK1_PIECES: [TrackPiece; 13] = [
    TrackPiece::Start,
    TrackPiece::Straight(StraightData::from_length(6.0)),
    TrackPiece::Straight(StraightData::from_left_right(-1.0, 0.5)),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Straight(StraightData::default()),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Straight(StraightData::from_length(2.0)),
    TrackPiece::Straight(StraightData::from_left_right(-2.0, 1.0)),
    TrackPiece::Straight(StraightData::from_left_right_length(-2.0, 1.0, 4.0)),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Straight(StraightData::from_left_right(-2.0, 1.0)),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Finish,
];

pub static TRACK1_DATA: TrackData = TrackData {
    pieces: &TRACK1_PIECES,
    initial_position: Vec3::new(1.0, 2.25, 0.0),
    initial_forward: Vec3::new(-1.0, 0.0, 0.0),
    initial_up: Vec3::Z,
    initial_left: -2.0,
    initial_right: 1.0,
    num_segments: 4,
};
