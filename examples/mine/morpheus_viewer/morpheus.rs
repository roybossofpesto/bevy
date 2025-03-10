use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

use bevy::prelude::*;

use bevy::color::palettes::basic::BLUE;
use bevy::color::palettes::basic::GREEN;
use bevy::color::palettes::basic::RED;
use std::f32::consts::PI;

//////////////////////////////////////////////////////////////////////

pub struct MorpheusPlugin;

impl Plugin for MorpheusPlugin {
    fn build(&self, app: &mut App) {
        info!("** build_morpheus_plugin **");

        app.add_plugins(MaterialPlugin::<MorpheusSphereMaterial>::default());
        app.add_plugins(MaterialPlugin::<MorpheusUnionMaterial>::default());
        app.add_plugins(MaterialPlugin::<MorpheusAlienMaterial>::default());

        app.add_systems(Startup, populate_camera_and_lights);
        app.add_systems(Startup, populate_models);
        app.add_systems(Update, animate_camera);
    }
}

//////////////////////////////////////////////////////////////////////

fn populate_models(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut morpheus_sphere_materials: ResMut<Assets<MorpheusSphereMaterial>>,
    mut morpheus_union_materials: ResMut<Assets<MorpheusUnionMaterial>>,
    mut morpheus_alien_materials: ResMut<Assets<MorpheusAlienMaterial>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let tube = meshes.add(Mesh::from(Cylinder::new(0.1, 2.0)));
    commands.spawn((
        Mesh3d(tube.clone()),
        MeshMaterial3d(standard_materials.add(StandardMaterial {
            base_color: Color::from(RED),
            ..default()
        })),
        Transform::from_xyz(0.0, -1.2, -1.2)
            .with_rotation(Quat::from_axis_angle(Vec3::Z, PI / 2.0)),
    ));
    commands.spawn((
        Mesh3d(tube.clone()),
        MeshMaterial3d(standard_materials.add(StandardMaterial {
            base_color: Color::from(GREEN),
            ..default()
        })),
        Transform::from_xyz(-1.2, 0.0, -1.2),
    ));
    commands.spawn((
        Mesh3d(tube),
        MeshMaterial3d(standard_materials.add(StandardMaterial {
            base_color: Color::from(BLUE),
            ..default()
        })),
        Transform::from_xyz(-1.2, -1.2, 0.0)
            .with_rotation(Quat::from_axis_angle(Vec3::X, PI / 2.0)),
    ));

    let matcap_texture = asset_server.load("textures/matcap/583629_2E1810_765648_3C1C14-512px.png");

    let sphere_material = morpheus_sphere_materials.add(MorpheusSphereMaterial {
        matcap_texture: Some(matcap_texture.clone()),
        alpha_mode: AlphaMode::Blend,
    });
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(2.0, 2.0, 2.0)))),
        MeshMaterial3d(sphere_material),
        Transform::from_xyz(2.0, 0.0, 0.0),
    ));

    let union_material = morpheus_union_materials.add(MorpheusUnionMaterial {
        matcap_texture: Some(matcap_texture.clone()),
        alpha_mode: AlphaMode::Blend,
    });
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(2.0, 2.0, 2.0)))),
        MeshMaterial3d(union_material),
        Transform::from_xyz(0.0, 0.0, 2.0),
    ));

    let alien_material = morpheus_alien_materials.add(MorpheusAlienMaterial {
        matcap_texture: Some(matcap_texture),
        alpha_mode: AlphaMode::Blend,
    });
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(2.0, 2.0, 2.0)))),
        MeshMaterial3d(alien_material),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

#[derive(Component)]
struct CameraPivot {
    sensitivity: f32,
}

impl CameraPivot {
    fn default() -> Self {
        Self { sensitivity: 200.0 }
    }
}

fn populate_camera_and_lights(mut commands: Commands) {
    // use bevy::render::camera::ScalingMode;

    info!("** populate_camera_and_lights **");

    // lights
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 5.0e6,
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
            illuminance: light_consts::lux::OVERCAST_DAY,
            ..default()
        },
        Transform::from_translation(Vec3::Y).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // camera
    commands
        .spawn((
            Transform::from_translation(Vec3::ZERO),
            CameraPivot::default(),
            InheritedVisibility::VISIBLE,
        ))
        .with_child((
            Transform::from_xyz(0.0, 2.0, -5.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
            Camera3d::default(),
        ));
}

fn animate_camera(
    mut query: Query<(&mut Transform, &CameraPivot)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
) {
    let Ok((mut transform, pivot)) = query.get_single_mut() else {
        return;
    };
    if mouse_input.pressed(MouseButton::Left) {
        let delta = mouse_motion.delta;
        transform.rotation *=
            Quat::from_axis_angle(Vec3::X, PI / 2.0 * delta.y / pivot.sensitivity);
        transform.rotation *=
            Quat::from_axis_angle(Vec3::Y, -PI / 2.0 * delta.x / pivot.sensitivity);
    }
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        transform.rotation = Quat::IDENTITY;
    }
}

//////////////////////////////////////////////////////////////////////

const SPHERE_SHADER_ASSET_PATH: &str = "shaders/morpheus/sphere.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct MorpheusSphereMaterial {
    #[texture(0)]
    #[sampler(1)]
    matcap_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

impl Material for MorpheusSphereMaterial {
    fn vertex_shader() -> ShaderRef {
        SPHERE_SHADER_ASSET_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        SPHERE_SHADER_ASSET_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

const UNION_SHADER_ASSET_PATH: &str = "shaders/morpheus/union.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct MorpheusUnionMaterial {
    #[texture(0)]
    #[sampler(1)]
    matcap_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

impl Material for MorpheusUnionMaterial {
    fn vertex_shader() -> ShaderRef {
        UNION_SHADER_ASSET_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        UNION_SHADER_ASSET_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

const ALIEN_SHADER_ASSET_PATH: &str = "shaders/morpheus/alien.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct MorpheusAlienMaterial {
    #[texture(0)]
    #[sampler(1)]
    matcap_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

impl Material for MorpheusAlienMaterial {
    fn vertex_shader() -> ShaderRef {
        ALIEN_SHADER_ASSET_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        ALIEN_SHADER_ASSET_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}
