//! offroad ftw

use bevy::prelude::*;

// use bevy::color::palettes::basic::RED;
// use bevy::color::palettes::basic::BLUE;
use bevy::color::palettes::basic::SILVER;
use bevy::math::Affine2;
use bevy::pbr::DirectionalLightShadowMap;
use bevy::render::camera::ScalingMode;

use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};

mod scene;
mod track;

use scene::{make_cube_mesh, make_uv_debug_texture};
use track::{make_track_mesh, TRACK0_PIECES};

fn main() {
    let mut app = App::new();

    app.insert_resource(DirectionalLightShadowMap { size: 2048 });
    app.add_plugins(DefaultPlugins);

    #[cfg(feature = "bevy_dev_tools")]
    {
        use bevy::color::palettes::basic::WHITE;
        use bevy::pbr::wireframe::WireframeConfig;
        use bevy::pbr::wireframe::WireframePlugin;

        // wireframe
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
    // let track_material = materials.add(Color::from(BLUE));
    let track_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load_with_settings(
            // "textures/uv_checker_bw.png",
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

        // uv_transform used here for proportions only, but it is full Affine2
        // that's why you can use rotation and shift also
        // uv_transform: Affine2::from_scale(Vec2::new(1.0 / 16.0, 1.0 / 16.0)),
        uv_transform: Affine2::default(),
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(make_track_mesh(&TRACK0_PIECES))),
        MeshMaterial3d(track_material),
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
