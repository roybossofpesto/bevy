//! offroad ftw

use bevy::prelude::*;

// use bevy::color::palettes::basic::RED;
use bevy::color::palettes::basic::BLUE;
use bevy::color::palettes::basic::SILVER;
use bevy::math::Affine2;
use bevy::pbr::DirectionalLightShadowMap;
use bevy::pbr::UvChannel;
use bevy::render::camera::ScalingMode;

use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};

use std::f32::consts::PI;

mod scene;
mod track;

use scene::{make_cube_mesh, make_uv_debug_texture};
use track::{make_track_mesh, TRACK0_DATA, TRACK1_DATA};

fn main() {
    let mut app = App::new();

    app.insert_resource(DirectionalLightShadowMap { size: 2048 });
    app.add_plugins(DefaultPlugins);

    #[cfg(feature = "bevy_dev_tools")]
    {
        // fps overlay
        use bevy::dev_tools::fps_overlay::FpsOverlayConfig;
        use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;
        app.add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig::default(),
        });
    }

    #[cfg(feature = "bevy_dev_tools")]
    {
        // wireframe toggle
        use bevy::color::palettes::basic::WHITE;
        use bevy::pbr::wireframe::WireframeConfig;
        use bevy::pbr::wireframe::WireframePlugin;
        app.insert_resource(WireframeConfig {
            global: false,
            default_color: WHITE.into(),
        });
        app.add_plugins(WireframePlugin);
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
    }

    app.add_systems(Startup, setup);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    info!("** setup **");

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
        Transform::from_xyz(5.0, 3.0, 5.0),
    ));

    // cube
    commands.spawn((
        Mesh3d(meshes.add(make_cube_mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("textures/array_texture.png")),
            ..default()
        })),
        Transform::from_xyz(8.0, 3.0, 5.0),
    ));

    // parallal
    let parallal_material = materials.add(StandardMaterial {
        perceptual_roughness: 0.2,
        normal_map_channel: UvChannel::Uv1,
        normal_map_texture: Some(asset_server.load_with_settings(
            "textures/parallax_example/cube_normal.png",
            // The normal map texture is in linear color space. Lighting won't look correct
            // if `is_srgb` is `true`, which is the default.
            |settings: &mut ImageLoaderSettings| {
                *settings = ImageLoaderSettings {
                    is_srgb: false,
                    sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        ..default()
                    }),
                    ..default()
                }
            },
        )),
        base_color_channel: UvChannel::Uv1,
        base_color_texture: Some(asset_server.load_with_settings(
            "textures/parallax_example/cube_color.png",
            |settings: &mut ImageLoaderSettings| {
                *settings = ImageLoaderSettings {
                    sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        ..default()
                    }),
                    ..default()
                }
            },
        )),
        depth_map: Some(asset_server.load_with_settings(
            "textures/parallax_example/cube_depth.png",
            |settings: &mut ImageLoaderSettings| {
                *settings = ImageLoaderSettings {
                    sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        ..default()
                    }),
                    ..default()
                }
            },
        )),
        parallax_depth_scale: 0.1,
        max_parallax_layer_count: 32.0,
        parallax_mapping_method: ParallaxMappingMethod::Relief { max_steps: 16 },
        ..default()
    });
    commands.spawn((
        Mesh3d(
            meshes.add(
                Mesh::from(Cuboid::default())
                    .with_generated_tangents()
                    .unwrap(),
            ),
        ),
        MeshMaterial3d(parallal_material),
        Transform::from_xyz(3.0, 2.0, 18.0).with_scale(Vec3::ONE * 4.0),
    ));

    // track 0 showcases flow parametrization
    let track0_material = materials.add(StandardMaterial {
        base_color_channel: UvChannel::Uv0,
        base_color_texture: Some(asset_server.load_with_settings(
            "textures/fantasy_ui_borders/panel-border-010-repeated.png",
            |s: &mut _| {
                *s = ImageLoaderSettings {
                    sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                        // rewriting mode to repeat image,
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        ..default()
                    }),
                    ..default()
                }
            },
        )),
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(make_track_mesh(&TRACK0_DATA))),
        MeshMaterial3d(track0_material.clone()),
    ));

    // track 1 showcases projected parametrization
    let track1_material = materials.add(StandardMaterial {
        base_color_channel: UvChannel::Uv1,
        base_color_texture: Some(asset_server.load_with_settings(
            "textures/uv_checker_bw.png",
            |s: &mut _| {
                *s = ImageLoaderSettings {
                    sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        ..default()
                    }),
                    ..default()
                }
            },
        )),
        uv_transform: Affine2::from_scale(Vec2::new(1.0 / 8.0, 1.0 / 8.0)),
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(make_track_mesh(&TRACK1_DATA))),
        MeshMaterial3d(track1_material),
        Transform::from_xyz(-1.0, 0.0, -2.0)
    ));

    // track2 showcases water effect
    commands.spawn((
        Mesh3d(meshes.add(make_track_mesh(&TRACK1_DATA))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::from(BLUE),
            ..default()
        })),
        Transform::from_xyz(11.0, 0.0, 10.0)
            .with_rotation(Quat::from_axis_angle(Vec3::X, -PI / 3.0)),
    ));

    // "textures/BlueNoise-Normal.png",

    // lights
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 1.0e6,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(-4.0, 16.0, 8.0),
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
