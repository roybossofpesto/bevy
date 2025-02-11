use bevy::prelude::{debug, info, warn};

use bevy::math::NormedVectorSpace;

use std::f32::consts::PI;

#[derive(Debug)]
pub enum RoadPiece {
    Start,
    Straight(StraightData),
    Corner(CornerData),
    Finish,
}

#[derive(Debug)]
pub struct StraightData {
    pub left: f32,
    pub right: f32,
    pub length: f32,
}

impl Default for StraightData {
    fn default() -> Self {
        Self {
            left: -1.0,
            right: 1.0,
            length: 2.0,
        }
    }
}

#[derive(Debug)]
pub struct CornerData {
    pub left: f32,
    pub right: f32,
    pub radius: f32,
    pub angle: f32,
    pub num_quads: u32,
}

impl CornerData {
    pub fn right_turn() -> Self {
        Self {
            left: -1.0,
            right: 1.0,
            radius: 2.0,
            angle: PI / 2.0,
            num_quads: 8,
        }
    }
    pub fn left_turn() -> Self {
        Self {
            left: -1.0,
            right: 1.0,
            radius: -2.0,
            angle: PI / 2.0,
            num_quads: 8,
        }
    }
}

pub fn make_road_mesh(pieces: &Vec<RoadPiece>) -> bevy::render::mesh::Mesh {
    use bevy::prelude::Quat;
    use bevy::prelude::Vec2;
    use bevy::prelude::Vec3;

    use bevy::render::mesh::Indices;
    use bevy::render::mesh::Mesh;
    use bevy::render::render_asset::RenderAssetUsages;
    use bevy::render::render_resource::PrimitiveTopology;

    let initial_position = Vec3::new(-10.0, 0.25, 0.0);
    let initial_forward = Vec3::Z;
    let initial_up = Vec3::Y;

    let mut mesh_positions: Vec<Vec3> = vec![];
    let mut mesh_normals: Vec<Vec3> = vec![];
    let mut mesh_triangles: Vec<u32> = vec![];
    let mut mesh_uvs: Vec<Vec2> = vec![];

    let mut push_section =
        |position: &Vec3, forward: &Vec3, left: f32, right: f32, length: f32| -> u32 {
            let left_pos = position + forward.cross(initial_up) * left;
            let right_pos = position + forward.cross(initial_up) * right;
            let next_vertex = mesh_positions.len() as u32;
            assert!(next_vertex % 2 == 0);
            mesh_positions.push(left_pos);
            mesh_positions.push(right_pos);
            mesh_normals.push(initial_up);
            mesh_normals.push(initial_up);
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

    let mut current_position = initial_position.clone();
    let mut current_forward = initial_forward.clone();
    // let current_up = initial_up.clone();
    let mut current_length: f32 = 0.0;

    for piece in pieces {
        match piece {
            RoadPiece::Start => {
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
            RoadPiece::Straight(data) => {
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
            RoadPiece::Corner(data) => {
                debug!("Corner {:?} {:?}", current_position.clone(), data);
                assert!(data.num_quads > 0);
                let current_right = current_forward.cross(initial_up);
                let center = current_position + current_right * data.radius;
                let sign: f32 = if data.radius < 0.0 { 1.0 } else { -1.0 };
                for kk in 0..data.num_quads {
                    let ang = (kk + 1) as f32 / data.num_quads as f32 * data.angle;
                    let pos = center - current_right * data.radius * f32::cos(ang)
                        + current_forward * f32::abs(data.radius) * f32::sin(ang);
                    let fwd = Quat::from_axis_angle(initial_up, sign * ang) * current_forward;
                    let len = f32::abs(data.radius) * ang + current_length;
                    let foo = push_section(&pos, &fwd, data.left, data.right, len);
                    assert!(foo > 0);
                }
                current_position += current_forward * f32::abs(data.radius);
                current_forward =
                    Quat::from_axis_angle(initial_up, sign * data.angle) * current_forward;
                current_position += current_forward * data.radius;
                current_length += f32::abs(data.radius) * data.angle;
            }
            RoadPiece::Finish => {
                let pos_error = (current_position - initial_position).norm();
                let dir_error = (current_forward - initial_forward).norm();
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
