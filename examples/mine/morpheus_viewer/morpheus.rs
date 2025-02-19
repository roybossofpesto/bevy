use bevy::input::mouse::AccumulatedMouseMotion;

use bevy::prelude::*;

use std::f32::consts::PI;

//////////////////////////////////////////////////////////////////////

pub struct MorpheusPlugin;

impl Plugin for MorpheusPlugin {
    fn build(&self, app: &mut App) {
        info!("** build_morpheus_plugin **");

        app.add_systems(Startup, populate_camera_and_lights);
        app.add_systems(Startup, populate_models);
        app.add_systems(Update, animate_camera);
    }
}

//////////////////////////////////////////////////////////////////////

fn populate_models(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(2.0, 2.0, 2.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(images.add(make_texture())),
            ..default()
        })),
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
            Transform::from_xyz(-5.0, 2.0, 0.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
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
            Quat::from_axis_angle(Vec3::Z, -PI / 2.0 * delta.y / pivot.sensitivity);
        transform.rotation *=
            Quat::from_axis_angle(Vec3::Y, -PI / 2.0 * delta.x / pivot.sensitivity);
    }
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        transform.rotation = Quat::IDENTITY;
    }
}

//////////////////////////////////////////////////////////////////////

fn make_texture() -> Image {
    use bevy::image::ImageSampler;
    use bevy::render::render_asset::RenderAssetUsages;
    use bevy::render::render_resource::Extent3d;
    use bevy::render::render_resource::TextureDimension;
    use bevy::render::render_resource::TextureFormat;

    const TEXTURE_SIZE: usize = 8;

    // let mut palette: [u8; 32] = [
    //     255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
    //     198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    // ];

    let uu_color: [u8; 4] = [255, 0, 0, 255];
    let vv_color: [u8; 4] = [0, 255, 0, 255];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    texture_data.fill(128);

    for ii in 0..TEXTURE_SIZE {
        let offset = ii * 4;
        texture_data[offset..(offset + 4)].copy_from_slice(&uu_color);
    }

    for jj in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * jj * 4;
        texture_data[offset..(offset + 4)].copy_from_slice(&vv_color);
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
