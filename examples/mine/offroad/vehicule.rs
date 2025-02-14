use bevy::prelude::*;

use bevy::color::palettes::basic::PURPLE;
use bevy::color::palettes::basic::YELLOW;

//////////////////////////////////////////////////////////////////////

pub struct VehiculePlugin;

impl Plugin for VehiculePlugin {
    fn build(&self, app: &mut App) {
        info!("** build_vehicule **");

        app.add_systems(Startup, setup_vehicule);
        app.add_systems(Update, update_vehicule_physics);
    }
    // fn finish(&self, app: &mut App) {
    //     info!("** simu_finish **");
    //     let render_app = app.sub_app_mut(RenderApp);
    //     render_app.init_resource::<SimuPipeline>();
    // }
}

//////////////////////////////////////////////////////////////////////

enum Player {
    One,
    Two,
}

#[derive(Component)]
struct BoatData {
    player: Player,
    position_prev: [f32; 2],
    position_current: [f32; 2],
}

fn setup_vehicule(
    mut commands: Commands,
    // mut images: ResMut<Assets<Image>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    server: Res<AssetServer>,
) {
    info!("** setup_vehicule **");

    // get a specific mesh
    // let my_mesh: Handle<Mesh> = server.load("models/offroad/boat.gltf#Mesh0/Primitive0");
    let my_mesh: Handle<Mesh> = server.load("models/offroad/boat.glb#Mesh0/Primitive0");
    // let my_mesh: Handle<Mesh> = server.load("models/animated/Fox.glb");

    let red_pos = Vec3::new(-11.5, 0.0, 0.0);
    let blue_pos = Vec3::new(-12.5, 0.0, 0.0);

    commands.spawn((
        Mesh3d(my_mesh.clone()),
        MeshMaterial3d(materials.add(Color::from(YELLOW))),
        Transform::from_translation(red_pos)
            .looking_at(red_pos + Vec3::Z, Vec3::Y)
            .with_scale(Vec3::ONE * 0.15),
        BoatData {
            player: Player::One,
            position_prev: red_pos.xz().into(),
            position_current: red_pos.xz().into(),
        },
    ));
    commands.spawn((
        Mesh3d(my_mesh),
        MeshMaterial3d(materials.add(Color::from(PURPLE))),
        Transform::from_translation(blue_pos)
            .looking_at(blue_pos + Vec3::Z, Vec3::Y)
            .with_scale(Vec3::ONE * 0.15),
        BoatData {
            player: Player::Two,
            position_prev: blue_pos.xz().into(),
            position_current: blue_pos.xz().into(),
        },
    ));
}

fn update_vehicule_physics(
    mut query: Query<(&mut BoatData, &mut Transform)>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for (mut data, mut transform) in &mut query {
        let pos_prev = Vec2::from_array(data.position_prev);
        let pos_current = Vec2::from_array(data.position_current);
        let mut accel_current = Vec2::ZERO;
        match data.player {
            Player::One => {
                if keyboard.just_pressed(KeyCode::Enter) {
                    accel_current.x = 1.0;
                }
                // if keyboard.just_pressed(KeyCode::ArrowRight) {
                //     delta.x += 1.0;
                // }
                // if keyboard.just_pressed(KeyCode::ArrowUp) {
                //     delta.y += 1.0;
                // }
                // if keyboard.just_pressed(KeyCode::ArrowDown) {
                //     delta.y -= 1.0;
                // }
                // transform.translation += delta;
            }
            Player::Two => {
                if keyboard.just_pressed(KeyCode::KeyA) {
                    accel_current.y = 1.0;
                }
                // if keyboard.just_pressed(KeyCode::ArrowRight) {
                //     delta.x += 1.0;
                // }
                // if keyboard.just_pressed(KeyCode::ArrowUp) {
                //     delta.y += 1.0;
                // }
                // if keyboard.just_pressed(KeyCode::ArrowDown) {
                //     delta.y -= 1.0;
                // }
                // transform.translation += delta;
            }
        }
        accel_current *= 1e-3;
        let dt = time.elapsed_secs();
        let pos_next = 2.0 * pos_current - pos_prev + accel_current * dt * dt;
        data.position_prev = data.position_current;
        data.position_current = pos_next.into();
        transform.translation = Vec3::new(pos_next.x, 0.0, pos_next.y);
    }
}
