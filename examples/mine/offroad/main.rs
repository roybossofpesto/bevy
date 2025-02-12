//! offroad ftw

// use bevy::color::palettes::basic::RED;
// use bevy::color::palettes::basic::BLUE;

use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};

use std::f32::consts::PI;

mod scene;
mod track;

use scene::{populate_background, populate_camera_and_lights};
use track::{make_track_material, make_track_mesh, TRACK0_DATA, TRACK1_DATA};

use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.insert_resource(bevy::pbr::DirectionalLightShadowMap { size: 2048 });
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
    app.add_systems(Startup, populate_background);
    app.add_systems(Startup, populate_camera_and_lights);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    use bevy::math::Affine2;
    use bevy::pbr::UvChannel;

    info!("** setup **");

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
        Transform::from_xyz(-1.0, 0.0, -2.0),
    ));

    // track2 showcases parallax effect
    let track2_material = materials.add(make_track_material(asset_server, 0.5));
    commands.spawn((
        Mesh3d(meshes.add(make_track_mesh(&TRACK1_DATA))),
        MeshMaterial3d(track2_material),
        Transform::from_xyz(12.0, 0.0, 9.0)
            .with_rotation(Quat::from_axis_angle(Vec3::X, -PI / 2.0)),
    ));

    // "textures/BlueNoise-Normal.png",
}
