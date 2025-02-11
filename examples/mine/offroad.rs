//! offroad ftw

use bevy::prelude::*;

// use bevy::color::palettes::basic::RED;
use bevy::color::palettes::basic::BLUE;
use bevy::color::palettes::basic::SILVER;
use bevy::color::palettes::basic::WHITE;
use bevy::pbr::wireframe::WireframeConfig;
use bevy::pbr::wireframe::WireframePlugin;
use bevy::pbr::DirectionalLightShadowMap;
use bevy::render::camera::ScalingMode;

use bevy::render::mesh::Indices;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::Extent3d;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::render::render_resource::TextureDimension;
use bevy::render::render_resource::TextureFormat;
// use bevy::render::mesh;

use std::f32::consts::PI;

fn main() {
    let mut app = App::new();

    app.insert_resource(DirectionalLightShadowMap { size: 2048 });
    app.insert_resource(WireframeConfig {
        // The global wireframe config enables drawing of wireframes on every mesh,
        // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
        // regardless of the global configuration.
        global: true,
        // Controls the default color of all wireframes. Used as the default color for global wireframes.
        // Can be changed per mesh using the `WireframeColor` component.
        default_color: WHITE.into(),
    });

    app.add_plugins((DefaultPlugins, WireframePlugin));

    app.add_systems(Startup, setup);

    app.add_systems(
        Update,
        |mut wireframe_config: ResMut<WireframeConfig>,
         keyboard: Res<ButtonInput<KeyCode>>|
         -> () {
            if keyboard.just_pressed(KeyCode::Space) {
                wireframe_config.global = !wireframe_config.global;
            }
        },
    );

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    info!("coucou");

    // ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
    ));

    // tower
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(make_uv_debug_texture())),
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 5.0, 1.0))),
        MeshMaterial3d(debug_material),
        Transform::from_xyz(0.0, 3.0, 0.0),
    ));

    // cube
    commands.spawn((
        Mesh3d(meshes.add(make_cube_mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("textures/array_texture.png")),
            ..default()
        })),
        Transform::from_xyz(3.0, 3.0, 0.0),
    ));

    // track
    let pieces = vec![
        RoadPiece::Start,
        RoadPiece::Straight(StraightData::default()),
        RoadPiece::Finish,
    ];
    commands.spawn((
        Mesh3d(meshes.add(make_road(&pieces))),
        MeshMaterial3d(materials.add(Color::from(BLUE))),
    ));

    // lights
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 1.0e6,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));
    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            shadows_enabled: true,
            illuminance: light_consts::lux::CLEAR_SUNRISE,
            ..default()
        },
        Transform::from_translation(Vec3::Y).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            // 20 world units per pixel of window height.
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 20.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(-10.0, 10.0, 14.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
    ));
}

/// Creates a colorful test pattern
fn make_uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}

#[derive(Debug)]
enum RoadPiece {
    Start,
    Straight(StraightData),
    Corner(CornerData),
    Finish,
}

#[derive(Debug)]
struct StraightData {
    left: f32,
    right: f32,
    length: f32,
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
struct CornerData {
    radius: f32,
    angle: f32,
    num_quads: u32,
}

impl Default for CornerData {
    fn default() -> Self {
        Self {
            radius: 2.0,
            angle: -PI / 2.0,
            num_quads: 8,
        }
    }
}

impl CornerData {
    const fn right_turn() -> Self {
        Self {
            radius: 2.0,
            angle: PI / 2.0,
            num_quads: 8,
        }
    }
}

// struct RoadPiece {
//     left: f32,
//     right: f32,
//     length: f32,
//     // angle: f32,
// }

// impl RoadPiece {
//     const fn default() -> Self {
//         Self {
//             left: -1.0,
//             right: 1.0,
//             length: 4.0,
//             // angle: 0.0,
//             num_quads: 8,
//         }
//     }
//     const fn straight(length: f32) -> Self {
//         Self {
//             left: -1.0,
//             right: 1.0,
//             length,
//             // angle: 0.0,
//             num_quads: 1,
//         }
//     }
// }

fn make_road(pieces: &Vec<RoadPiece>) -> Mesh {
    let mut mesh_positions: Vec<Vec3> = vec![];
    let mut mesh_normals: Vec<Vec3> = vec![];
    let mut mesh_triangles: Vec<u32> = vec![];

    let initial_position = Vec3::new(-10.0, 0.25, 0.0);
    let initial_forward = -Vec3::Z;
    let initial_up = Vec3::Y;

    let mut push_section =
        |position: &Vec3, forward: &Vec3, up: &Vec3, left: f32, right: f32| -> u32 {
            let left_pos = position + forward.cross(initial_up) * left;
            let right_pos = position + forward.cross(initial_up) * right;
            let next_vertex = mesh_positions.len() as u32;
            assert!(next_vertex % 2 == 0);
            mesh_positions.push(left_pos);
            mesh_positions.push(right_pos);
            mesh_normals.push(initial_up);
            mesh_normals.push(initial_up);
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

    // assert!(piece.num_quads > 0);
    // for _ in 0..piece.num_quads {
    // }

    for piece in pieces {
        match piece {
            RoadPiece::Start => {
                info!("Start {:?}", current_position.clone());
                let foo = push_section(&current_position, &current_forward, &initial_up, -1.0, 1.0);
                assert!(foo == 0);
            }
            RoadPiece::Straight(data) => {
                info!("Straight {:?} {:?}", current_position.clone(), data);
                current_position += current_forward * data.length;
                let foo = push_section(
                    &current_position,
                    &current_forward,
                    &initial_up,
                    data.left,
                    data.right,
                );
                assert!(foo > 0);
            }
            RoadPiece::Corner(data) => {
                info!("Corner {:?} {:?}", current_position.clone(), data);
                // assert!(mesh_positions.len() > 0);
            }
            RoadPiece::Finish => {
                info!("Finish {:?}", current_position.clone());
                // push_section(-1.0, 1.0);
            }
        }
        //     push_road(piece);
    }

    assert!(mesh_triangles.len() % 3 == 0);
    info!("num_vertices {}", mesh_positions.len());
    info!("{:?}", mesh_positions);
    info!("num_triangles {}", mesh_triangles.len() / 3);

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    mesh = mesh.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_positions);
    mesh = mesh.with_inserted_indices(Indices::U32(mesh_triangles));
    mesh = mesh.with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_normals);

    mesh
}

#[rustfmt::skip]
fn make_cube_mesh() -> Mesh {
    // Keep the mesh data accessible in future frames to be able to mutate it in toggle_texture.
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        // Each array is an [x, y, z] coordinate in local space.
        // The camera coordinate space is right-handed x-right, y-up, z-back. This means "forward" is -Z.
        // Meshes always rotate around their local [0, 0, 0] when a rotation is applied to their Transform.
        // By centering our mesh around the origin, rotating the mesh preserves its center of mass.
        vec![
            // top (facing towards +y)
            [-0.5, 0.5, -0.5], // vertex with index 0
            [0.5, 0.5, -0.5], // vertex with index 1
            [0.5, 0.5, 0.5], // etc. until 23
            [-0.5, 0.5, 0.5],
            // bottom   (-y)
            [-0.5, -0.5, -0.5],
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [-0.5, -0.5, 0.5],
            // right    (+x)
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5], // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
            [0.5, 0.5, -0.5],
            // left     (-x)
            [-0.5, -0.5, -0.5],
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [-0.5, 0.5, -0.5],
            // back     (+z)
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, -0.5, 0.5],
            // forward  (-z)
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [0.5, 0.5, -0.5],
            [0.5, -0.5, -0.5],
        ],
    )
    // Set-up UV coordinates to point to the upper (V < 0.5), "dirt+grass" part of the texture.
    // Take a look at the custom image (assets/textures/array_texture.png)
    // so the UV coords will make more sense
    // Note: (0.0, 0.0) = Top-Left in UV mapping, (1.0, 1.0) = Bottom-Right in UV mapping
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            // Assigning the UV coords for the top side.
            [0.0, 0.2], [0.0, 0.0], [1.0, 0.0], [1.0, 0.2],
            // Assigning the UV coords for the bottom side.
            [0.0, 0.45], [0.0, 0.25], [1.0, 0.25], [1.0, 0.45],
            // Assigning the UV coords for the right side.
            [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
            // Assigning the UV coords for the left side.
            [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
            // Assigning the UV coords for the back side.
            [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
            // Assigning the UV coords for the forward side.
            [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
        ],
    )
    // For meshes with flat shading, normals are orthogonal (pointing out) from the direction of
    // the surface.
    // Normals are required for correct lighting calculations.
    // Each array represents a normalized vector, which length should be equal to 1.0.
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            // Normals for the top side (towards +y)
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            // Normals for the bottom side (towards -y)
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            // Normals for the right side (towards +x)
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            // Normals for the left side (towards -x)
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            // Normals for the back side (towards +z)
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            // Normals for the forward side (towards -z)
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
        ],
    )
    // Create the triangles out of the 24 vertices we created.
    // To construct a square, we need 2 triangles, therefore 12 triangles in total.
    // To construct a triangle, we need the indices of its 3 defined vertices, adding them one
    // by one, in a counter-clockwise order (relative to the position of the viewer, the order
    // should appear counter-clockwise from the front of the triangle, in this case from outside the cube).
    // Read more about how to correctly build a mesh manually in the Bevy documentation of a Mesh,
    // further examples and the implementation of the built-in shapes.
    //
    // The first two defined triangles look like this (marked with the vertex indices,
    // and the axis), when looking down at the top (+y) of the cube:
    //   -Z
    //   ^
    // 0---1
    // |  /|
    // | / | -> +X
    // |/  |
    // 3---2
    //
    // The right face's (+x) triangles look like this, seen from the outside of the cube.
    //   +Y
    //   ^
    // 10--11
    // |  /|
    // | / | -> -Z
    // |/  |
    // 9---8
    //
    // The back face's (+z) triangles look like this, seen from the outside of the cube.
    //   +Y
    //   ^
    // 17--18
    // |\  |
    // | \ | -> +X
    // |  \|
    // 16--19
    .with_inserted_indices(Indices::U32(vec![
        0,3,1 , 1,3,2, // triangles making up the top (+y) facing side.
        4,5,7 , 5,6,7, // bottom (-y)
        8,11,9 , 9,11,10, // right (+x)
        12,13,15 , 13,14,15, // left (-x)
        16,19,17 , 17,19,18, // back (+z)
        20,21,23 , 21,22,23, // forward (-z)
    ]))
}
