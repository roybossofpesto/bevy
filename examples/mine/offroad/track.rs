use bevy::asset::{Asset, AssetServer, Assets};
use bevy::math::NormedVectorSpace;
use bevy::math::{Mat3, Quat, Vec2, Vec3};
use bevy::pbr::StandardMaterial;
use bevy::reflect::TypePath;
use bevy::render::mesh::Mesh;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

use bevy::prelude::Vec3Swizzles;
use bevy::prelude::{debug, info, warn};
use bevy::prelude::{ButtonInput, KeyCode};
use bevy::prelude::{Commands, Component, Handle, Query, Res, ResMut, Time, With};
use bevy::prelude::{Mesh3d, MeshMaterial3d};

use bevy::color::palettes::basic::BLUE;
use bevy::color::palettes::basic::WHITE;

use std::f32::consts::PI;

//////////////////////////////////////////////////////////////////////

pub struct TrackPlugin;

impl bevy::prelude::Plugin for TrackPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        use bevy::prelude::{MaterialPlugin, Startup, Update};
        app.add_plugins(MaterialPlugin::<RacingLineMaterial>::default());
        app.add_systems(Startup, populate_tracks);
        app.add_systems(Startup, populate_racing_lines);
        app.add_systems(Update, animate_wavy_materials);
        app.add_systems(Update, animate_racing_line_materials);
    }
}

//////////////////////////////////////////////////////////////////////

fn populate_tracks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    use bevy::color::Color;
    use bevy::image::ImageAddressMode;
    use bevy::image::ImageLoaderSettings;
    use bevy::image::ImageSampler;
    use bevy::image::ImageSamplerDescriptor;
    use bevy::math::Affine2;
    use bevy::pbr::UvChannel;
    use bevy::prelude::Transform;

    info!("** populate_tracks **");

    // let checkpoint_material = materials.add(StandardMaterial {
    //     perceptual_roughness: 0.2,
    //     base_color: Color::from(RED),
    //     ..StandardMaterial::default()
    // });
    let checkpoint_material = materials.add(StandardMaterial {
        base_color: Color::hsva(0.0, 0.8, 1.0, 0.8),
        alpha_mode: AlphaMode::Blend,
        ..StandardMaterial::default()
    });

    // ColorMaterial

    // track 0 showcases flow parametrization
    let track0_material = materials.add(StandardMaterial {
        base_color_channel: UvChannel::Uv0,
        base_color_texture: Some(asset_server.load_with_settings(
            "textures/fantasy_ui_borders/panel-border-010.png",
            |s: &mut _| {
                *s = ImageLoaderSettings {
                    sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                        // rewriting mode to repeat image,
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        ..ImageSamplerDescriptor::default()
                    }),
                    ..ImageLoaderSettings::default()
                }
            },
        )),
        ..StandardMaterial::default()
    });
    commands.spawn((
        Mesh3d(meshes.add(make_track_mesh(&TRACK0_DATA).0)),
        MeshMaterial3d(track0_material.clone()),
    ));
    commands.spawn((
        Mesh3d(meshes.add(make_track_mesh(&TRACK0_DATA).1)),
        MeshMaterial3d(checkpoint_material),
    ));

    // track 1 showcases projected parametrization
    let track1_material = materials.add(StandardMaterial {
        base_color_channel: UvChannel::Uv1,
        base_color_texture: Some(asset_server.load_with_settings(
            // "textures/parallax_example/cube_color.png",
            // "textures/slice_square.png",
            // "textures/fantasy_ui_borders/panel-border-015.png",
            "textures/uv_checker_bw.png",
            |s: &mut _| {
                *s = ImageLoaderSettings {
                    sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        ..ImageSamplerDescriptor::default()
                    }),
                    ..ImageLoaderSettings::default()
                }
            },
        )),
        uv_transform: Affine2::from_scale(Vec2::new(1.0 / 8.0, 1.0 / 8.0)),
        ..StandardMaterial::default()
    });
    commands.spawn((
        Mesh3d(meshes.add(make_track_mesh(&TRACK1_DATA).0)),
        MeshMaterial3d(track1_material),
        Transform::from_xyz(-1.0, 0.0, -2.0),
    ));

    // track 2 showcases water effect
    let track2_material = materials.add(make_wavy_material(&asset_server, 0.5));
    commands.spawn((
        WavyMarker,
        Mesh3d(meshes.add(make_track_mesh(&TRACK1_DATA).0)),
        MeshMaterial3d(track2_material),
        Transform::from_xyz(12.0, 0.0, 9.0)
            .with_rotation(Quat::from_axis_angle(Vec3::X, -PI / 2.0)),
    ));
}

fn populate_racing_lines(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<RacingLineMaterial>>,
    asset_server: Res<AssetServer>,
) {
    info!("** populate_track_dots **");

    use bevy::prelude::Transform;

    // track 3 showcases racing lines on track 0 data
    let track3_mesh = make_track_mesh(&TRACK0_DATA);
    let track3_material = make_racing_line_material(&asset_server, track3_mesh.2);
    commands.spawn((
        Mesh3d(meshes.add(track3_mesh.0)),
        MeshMaterial3d(materials.add(track3_material)),
        Transform::from_xyz(0.0, 1e-3, 0.0),
    ));

    // track 4 showcases racing lines on track 1 data
    let track4_mesh = make_track_mesh(&TRACK1_DATA);
    let track4_material = make_racing_line_material(&asset_server, track4_mesh.2);
    commands.spawn((
        Mesh3d(meshes.add(track4_mesh.0)),
        MeshMaterial3d(materials.add(track4_material)),
        Transform::from_xyz(-1.0, 0.0, -2.0 + 1e-3),
    ));

    // track 5 showcases racing lines on track 1 data
    let track5_mesh = make_track_mesh(&TRACK1_DATA);
    let mut track5_material = make_racing_line_material(&asset_server, track5_mesh.2);
    track5_material.line_width = 0.5;
    track5_material.lateral_range = Vec2::new(-1.8, 0.8);
    commands.spawn((
        Mesh3d(meshes.add(track5_mesh.0)),
        MeshMaterial3d(materials.add(track5_material)),
        Transform::from_xyz(12.0, 1e-3, 9.0)
            .with_rotation(Quat::from_axis_angle(Vec3::X, -PI / 2.0)),
    ));

    // "textures/BlueNoise-Normal.png",
}

//////////////////////////////////////////////////////////////////////

use bevy::prelude::AlphaMode;
use bevy::prelude::LinearRgba;

const SHADER_ASSET_PATH: &str = "shaders/offroad/racing_line_material.wgsl";

// This struct defines the data that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct RacingLineMaterial {
    #[texture(0)]
    #[sampler(1)]
    color_texture: Option<Handle<bevy::image::Image>>,
    #[uniform(2)]
    color: LinearRgba,
    #[uniform(3)]
    track_length: f32,
    #[uniform(4)]
    line_width: f32,
    #[uniform(5)]
    time: f32,
    #[uniform(6)]
    cursor_position: Vec2,
    #[uniform(7)]
    cursor_radius: f32,
    #[uniform(8)]
    lateral_range: Vec2,
    alpha_mode: AlphaMode,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl bevy::prelude::Material for RacingLineMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

fn make_racing_line_material(
    asset_server: &Res<AssetServer>,
    track_length: f32,
) -> RacingLineMaterial {
    use bevy::image::ImageAddressMode;
    use bevy::image::ImageLoaderSettings;
    use bevy::image::ImageSampler;
    use bevy::image::ImageSamplerDescriptor;
    RacingLineMaterial {
        track_length,
        line_width: 0.2,
        lateral_range: Vec2::new(-0.8, 0.8),
        time: 0.0,
        cursor_position: Vec2::ZERO,
        cursor_radius: 0.5,
        color: LinearRgba::from(WHITE),
        color_texture: Some(asset_server.load_with_settings(
            // "branding/icon.png",
            // "textures/parallax_example/cube_color.png",
            "textures/slice_square.png",
            |settings: &mut ImageLoaderSettings| {
                *settings = ImageLoaderSettings {
                    sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        ..ImageSamplerDescriptor::default()
                    }),
                    ..ImageLoaderSettings::default()
                }
            },
        )),
        alpha_mode: AlphaMode::Blend,
    }
}

fn animate_racing_line_materials(
    material_handles: Query<&MeshMaterial3d<RacingLineMaterial>>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut materials: ResMut<Assets<RacingLineMaterial>>,
) {
    let mut delta = Vec2::ZERO;
    if keyboard.just_pressed(KeyCode::KeyJ) {
        delta.x -= 1.0;
    }
    if keyboard.just_pressed(KeyCode::KeyL) {
        delta.x += 1.0;
    }
    if keyboard.just_pressed(KeyCode::KeyI) {
        delta.y += 1.0;
    }
    if keyboard.just_pressed(KeyCode::KeyK) {
        delta.y -= 1.0;
    }
    for material_handle in material_handles.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            material.time += time.delta_secs();
            material.cursor_position += delta;
        }
    }
}

//////////////////////////////////////////////////////////////////////

#[derive(Component)]
struct WavyMarker;

fn make_wavy_material(asset_server: &Res<AssetServer>, scale: f32) -> StandardMaterial {
    use bevy::color::Color;
    use bevy::image::ImageAddressMode;
    use bevy::image::ImageLoaderSettings;
    use bevy::image::ImageSampler;
    use bevy::image::ImageSamplerDescriptor;
    use bevy::math::Affine2;
    use bevy::math::Vec2;
    use bevy::pbr::UvChannel;
    StandardMaterial {
        perceptual_roughness: 0.2,
        base_color: Color::from(BLUE),
        // base_color_channel: UvChannel::Uv1,
        // base_color_texture: Some(asset_server.load_with_settings(
        //     "textures/parallax_example/cube_color.png",
        //     |settings: &mut ImageLoaderSettings| {
        //         *settings = ImageLoaderSettings {
        //             sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
        //                 address_mode_u: ImageAddressMode::Repeat,
        //                 address_mode_v: ImageAddressMode::Repeat,
        //                 ..ImageSamplerDescriptor::default()
        //             }),
        //             ..ImageLoaderSettings::default()
        //         }
        //     },
        // )),
        normal_map_channel: UvChannel::Uv1,
        normal_map_texture: Some(asset_server.load_with_settings(
            "textures/wavy_normal.png",
            // The normal map texture is in linear color space. Lighting won't look correct
            // if `is_srgb` is `true`, which is the default.
            |settings: &mut ImageLoaderSettings| {
                *settings = ImageLoaderSettings {
                    is_srgb: false,
                    sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        ..ImageSamplerDescriptor::default()
                    }),
                    ..ImageLoaderSettings::default()
                }
            },
        )),
        depth_map: Some(asset_server.load_with_settings(
            "textures/wavy_depth.png",
            |settings: &mut ImageLoaderSettings| {
                *settings = ImageLoaderSettings {
                    sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        ..ImageSamplerDescriptor::default()
                    }),
                    ..ImageLoaderSettings::default()
                }
            },
        )),
        parallax_depth_scale: 0.1,
        uv_transform: Affine2::from_scale(Vec2::ONE * scale),
        ..StandardMaterial::default()
    }
}

fn animate_wavy_materials(
    material_handles: Query<&MeshMaterial3d<StandardMaterial>, With<WavyMarker>>,
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for material_handle in material_handles.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            material.uv_transform.translation.y += -0.8 * time.delta_secs();
        }
    }
}

//////////////////////////////////////////////////////////////////////

enum TrackPiece {
    Start,
    Straight(StraightData),
    Corner(CornerData),
    Checkpoint(u8),
    Finish,
}

#[derive(Debug)]
struct StraightData {
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
struct CornerData {
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

struct TrackData {
    pieces: &'static [TrackPiece],
    initial_position: Vec3,
    initial_forward: Vec3,
    initial_up: Vec3,
    initial_left: f32,
    initial_right: f32,
    num_segments: u32,
}

fn make_track_mesh(track_data: &TrackData) -> (Mesh, Mesh, f32, bool) {
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

    let mut checkpoint_positions: Vec<Vec3> = vec![];
    let mut checkpoint_normals: Vec<Vec3> = vec![];
    let mut checkpoint_triangles: Vec<u32> = vec![];
    let mut push_checkpoint = |position: &Vec3, forward: &Vec3, left: f32, right: f32| -> u32 {
        let righthand = forward.cross(track_data.initial_up);
        let aa = position + righthand * left;
        let bb = position + righthand * right;
        let cc = aa + track_data.initial_up;
        let dd = bb + track_data.initial_up;
        let next_vertex = checkpoint_positions.len() as u32;
        checkpoint_positions.push(aa);
        checkpoint_positions.push(bb);
        checkpoint_positions.push(cc);
        checkpoint_positions.push(dd);
        checkpoint_normals.push(-forward.clone());
        checkpoint_normals.push(-forward.clone());
        checkpoint_normals.push(-forward.clone());
        checkpoint_normals.push(-forward.clone());
        let mut tri_aa = vec![next_vertex, next_vertex + 1, next_vertex + 2];
        let mut tri_bb = vec![next_vertex + 2, next_vertex + 1, next_vertex + 3];
        checkpoint_triangles.append(&mut tri_aa);
        checkpoint_triangles.append(&mut tri_bb);
        next_vertex
    };

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
                    Mat3::from_cols(initial_righthand, track_data.initial_forward, Vec3::ZERO)
                        .transpose();
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
                let right_error = f32::abs(current_right - track_data.initial_right);
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
            TrackPiece::Checkpoint(index) => {
                debug!("Checkpoint {}", index);
                push_checkpoint(
                    &current_position,
                    &current_forward,
                    current_left,
                    current_right,
                );
                push_checkpoint(
                    &current_position,
                    &-current_forward,
                    -current_right,
                    -current_left,
                );
            }
        }
    }

    assert!(checkpoint_triangles.len() % 3 == 0);
    assert!(mesh_triangles.len() % 3 == 0);
    debug!("num_vertices {}", mesh_positions.len());
    debug!("num_triangles {}", mesh_triangles.len() / 3);
    debug!("total_length {}", current_length);
    if !is_looping {
        warn!("!!! road is not looping !!!");
    }

    use bevy::render::mesh::Indices;
    use bevy::render::mesh::Mesh;
    use bevy::render::render_asset::RenderAssetUsages;
    use bevy::render::render_resource::PrimitiveTopology;

    let mut checkpoint = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    checkpoint = checkpoint.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, checkpoint_positions);
    checkpoint = checkpoint.with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, checkpoint_normals);
    checkpoint = checkpoint.with_inserted_indices(Indices::U32(checkpoint_triangles));

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    mesh = mesh.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_positions);
    mesh = mesh.with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_normals);
    mesh = mesh.with_inserted_indices(Indices::U32(mesh_triangles));

    // let mut channel_uvs = Mesh::ATTRIBUTE_UV_0;
    // let mut channel_pqs = Mesh::ATTRIBUTE_UV_1;
    // if swap_uvs {
    //     std::mem::swap(&mut channel_uvs, &mut channel_pqs);
    // }
    mesh = mesh.with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_uvs);
    mesh = mesh.with_inserted_attribute(Mesh::ATTRIBUTE_UV_1, mesh_pqs);

    mesh = mesh.with_generated_tangents().unwrap();

    (mesh, checkpoint, current_length, is_looping)
}

//////////////////////////////////////////////////////////////////////

static TRACK0_PIECES: [TrackPiece; 21] = [
    TrackPiece::Start,
    TrackPiece::Straight(StraightData::default()),
    TrackPiece::Corner(CornerData::left_turn()),
    TrackPiece::Checkpoint(0),
    TrackPiece::Straight(StraightData::from_length(8.0)),
    TrackPiece::Checkpoint(1),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Checkpoint(2),
    TrackPiece::Straight(StraightData::default()),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Checkpoint(3),
    TrackPiece::Straight(StraightData::from_length(14.0)),
    TrackPiece::Checkpoint(4),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Straight(StraightData::from_length(11.0)),
    TrackPiece::Checkpoint(5),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Straight(StraightData::default()),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Straight(StraightData::from_length(3.0)),
    TrackPiece::Finish,
];

static TRACK0_DATA: TrackData = TrackData {
    pieces: &TRACK0_PIECES,
    initial_position: Vec3::new(-12.0, 0.0, 0.0),
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

static TRACK1_DATA: TrackData = TrackData {
    pieces: &TRACK1_PIECES,
    initial_position: Vec3::new(1.0, 2.0, 0.0),
    initial_forward: Vec3::new(-1.0, 0.0, 0.0),
    initial_up: Vec3::Z,
    initial_left: -2.0,
    initial_right: 1.0,
    num_segments: 4,
};
