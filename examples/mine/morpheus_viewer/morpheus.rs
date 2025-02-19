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

        app.add_plugins(MaterialPlugin::<MorpheusBasicMaterial>::default());

        app.add_systems(Startup, populate_camera_and_lights);
        app.add_systems(Startup, populate_models);
        app.add_systems(Update, animate_camera);
        app.add_systems(Update, animate_morpheus_basic_materials);
    }
}

//////////////////////////////////////////////////////////////////////

fn populate_models(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut morpheus_materials: ResMut<Assets<MorpheusBasicMaterial>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
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

    let basic_material = morpheus_materials.add(make_morpheus_basic_material(
        images.add(make_debug_texture()),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(2.0, 2.0, 2.0)))),
        MeshMaterial3d(basic_material),
        // MeshMaterial3d(materials.add(StandardMaterial {
        //     base_color_texture: Some(images.add(make_texture())),
        //     ..default()
        // })),
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

fn animate_morpheus_basic_materials(
    material_handles: Query<&MeshMaterial3d<MorpheusBasicMaterial>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut materials: ResMut<Assets<MorpheusBasicMaterial>>,
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
    delta /= 10.0;
    for material_handle in material_handles.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            material.cursor_data.x += delta.x;
            material.cursor_data.y += delta.y;
        }
    }
}

//////////////////////////////////////////////////////////////////////

fn make_debug_texture() -> Image {
    use bevy::image::ImageSampler;
    use bevy::render::render_asset::RenderAssetUsages;
    use bevy::render::render_resource::Extent3d;
    use bevy::render::render_resource::TextureDimension;
    use bevy::render::render_resource::TextureFormat;

    const TEXTURE_SIZE: usize = 64;

    let uu_color: [u8; 4] = [255, 0, 0, 255];
    let uu_color_: [u8; 4] = [0, 255, 255, 255];
    let vv_color: [u8; 4] = [0, 255, 0, 255];
    let vv_color_: [u8; 4] = [255, 0, 255, 255];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    texture_data.fill(0);

    for ii in 0..TEXTURE_SIZE {
        let offset = ii * 4;
        let offset_ = ii * 4 + (TEXTURE_SIZE - 1) * TEXTURE_SIZE * 4;
        texture_data[offset..(offset + 4)].copy_from_slice(&uu_color);
        texture_data[offset_..(offset_ + 4)].copy_from_slice(&uu_color_);
    }

    for jj in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * jj * 4;
        let offset_ = TEXTURE_SIZE * jj * 4 + (TEXTURE_SIZE - 1) * 4;
        texture_data[offset..(offset + 4)].copy_from_slice(&vv_color);
        texture_data[offset_..(offset_ + 4)].copy_from_slice(&vv_color_);
    }

    let mut image = Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );

    image.sampler = ImageSampler::nearest();

    image
}

const SHADER_ASSET_PATH: &str = "shaders/morpheus/basic.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct MorpheusBasicMaterial {
    #[texture(0)]
    #[sampler(1)]
    color_texture: Option<Handle<Image>>,
    #[uniform(2)]
    cursor_data: Vec4,
    alpha_mode: AlphaMode,
}

impl Material for MorpheusBasicMaterial {
    fn vertex_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

fn make_morpheus_basic_material(color_texture: Handle<Image>) -> MorpheusBasicMaterial {
    MorpheusBasicMaterial {
        color_texture: Some(color_texture),
        cursor_data: Vec4::new(0.0, 0.0, 0.1, 0.0),
        alpha_mode: AlphaMode::Blend,
    }
}
