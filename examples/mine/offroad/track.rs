use bevy::prelude::{debug, info, warn};

use bevy::math::NormedVectorSpace;
use bevy::math::{Vec2, Vec3};

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
}

#[derive(Debug)]
pub struct CornerData {
    left: f32,
    right: f32,
    radius: f32,
    angle: f32,
    num_quads: u32,
}

impl CornerData {
    const fn right_turn() -> Self {
        Self {
            left: -1.0,
            right: 1.0,
            radius: 2.0,
            angle: PI / 2.0,
            num_quads: 8,
        }
    }
    const fn left_turn() -> Self {
        Self {
            left: -1.0,
            right: 1.0,
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
}

pub fn make_track_mesh(track_data: &TrackData) -> bevy::render::mesh::Mesh {
    use bevy::prelude::Quat;

    use bevy::render::mesh::Indices;
    use bevy::render::mesh::Mesh;
    use bevy::render::render_asset::RenderAssetUsages;
    use bevy::render::render_resource::PrimitiveTopology;

    let up = track_data.initial_up;

    let mut mesh_positions: Vec<Vec3> = vec![];
    let mut mesh_normals: Vec<Vec3> = vec![];
    let mut mesh_triangles: Vec<u32> = vec![];
    let mut mesh_uvs: Vec<Vec2> = vec![];

    let mut push_section =
        |position: &Vec3, forward: &Vec3, left: f32, right: f32, length: f32| -> u32 {
            let left_pos = position + forward.cross(up) * left;
            let right_pos = position + forward.cross(up) * right;
            let next_vertex = mesh_positions.len() as u32;
            assert!(next_vertex % 2 == 0);
            mesh_positions.push(left_pos);
            mesh_positions.push(right_pos);
            mesh_normals.push(up);
            mesh_normals.push(up);
            mesh_uvs.push(Vec2::new(left, length));
            mesh_uvs.push(Vec2::new(right, length));
            if next_vertex >= 2 {
                let mut tri_aa = vec![next_vertex - 2, next_vertex - 1, next_vertex];
                let mut tri_bb = vec![next_vertex - 1, next_vertex + 1, next_vertex];
                mesh_triangles.append(&mut tri_aa);
                mesh_triangles.append(&mut tri_bb);
            }
            next_vertex
        };

    let mut current_position = track_data.initial_position.clone();
    let mut current_forward = track_data.initial_forward.clone();
    let mut current_length: f32 = 0.0;

    for piece in track_data.pieces {
        match piece {
            TrackPiece::Start => {
                debug!("Start {:?}", current_position.clone());
                assert!(current_length == 0.0);
                let foo = push_section(
                    &current_position,
                    &current_forward,
                    -1.0,
                    1.0,
                    current_length,
                );
                assert!(foo == 0);
            }
            TrackPiece::Straight(data) => {
                debug!("Straight {:?} {:?}", current_position.clone(), data);
                current_position += current_forward * data.length;
                current_length += data.length;
                assert!(current_length != 0.0);
                let foo = push_section(
                    &current_position,
                    &current_forward,
                    data.left,
                    data.right,
                    current_length,
                );
                assert!(foo > 0);
            }
            TrackPiece::Corner(data) => {
                debug!("Corner {:?} {:?}", current_position.clone(), data);
                assert!(data.num_quads > 0);
                let current_right = current_forward.cross(up);
                let center = current_position + current_right * data.radius;
                let sign: f32 = if data.radius < 0.0 { 1.0 } else { -1.0 };
                for kk in 0..data.num_quads {
                    let ang = (kk + 1) as f32 / data.num_quads as f32 * data.angle;
                    let pos = center - current_right * data.radius * f32::cos(ang)
                        + current_forward * f32::abs(data.radius) * f32::sin(ang);
                    let fwd = Quat::from_axis_angle(up, sign * ang) * current_forward;
                    let len = f32::abs(data.radius) * ang + current_length;
                    let foo = push_section(&pos, &fwd, data.left, data.right, len);
                    assert!(foo > 0);
                }
                current_position += current_forward * f32::abs(data.radius);
                current_forward = Quat::from_axis_angle(up, sign * data.angle) * current_forward;
                current_position += current_forward * data.radius;
                current_length += f32::abs(data.radius) * data.angle;
            }
            TrackPiece::Finish => {
                let pos_error = (current_position - track_data.initial_position).norm();
                let dir_error = (current_forward - track_data.initial_forward).norm();
                let is_looping: bool = pos_error < 1e-3 && dir_error < 1e-3;
                debug!(
                    "Finish {:?} pos_err {:0.3e} dir_err {:0.3e} total_length {} loop {}",
                    current_position.clone(),
                    pos_error,
                    dir_error,
                    current_length,
                    is_looping,
                );
                if !is_looping {
                    warn!("!!! road is not looping !!!");
                }
            }
        }
        //     push_road(piece);
    }

    assert!(mesh_triangles.len() % 3 == 0);
    info!("num_vertices {}", mesh_positions.len());
    info!("num_triangles {}", mesh_triangles.len() / 3);
    info!("total_length {}", current_length);

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    mesh = mesh.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_positions);
    mesh = mesh.with_inserted_indices(Indices::U32(mesh_triangles));
    mesh = mesh.with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_normals);
    mesh = mesh.with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_uvs);

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
    TrackPiece::Straight(StraightData::from_length(10.0)),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Straight(StraightData::from_length(12.0)),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Straight(StraightData::default()),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Straight(StraightData::from_length(4.0)),
    TrackPiece::Finish,
];

pub static TRACK0_DATA: TrackData = TrackData {
    pieces: &TRACK0_PIECES,
    initial_position: Vec3::new(-10.0, 0.25, 0.0),
    initial_forward: Vec3::Z,
    initial_up: Vec3::Y,
};
