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
    pos_prev: [f32; 2],
    pos_current: [f32; 2],
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
            pos_prev: red_pos.xz().into(),
            pos_current: red_pos.xz().into(),
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
            pos_prev: blue_pos.xz().into(),
            pos_current: blue_pos.xz().into(),
        },
    ));
}

fn update_vehicule_physics(
    mut query: Query<(&mut BoatData, &mut Transform)>,
    // time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for (mut data, mut transform) in &mut query {
        let mut pos_next = data.pos_current;
        match data.player {
            Player::One => {
                if keyboard.just_pressed(KeyCode::Enter) {
                    pos_next[1] += 1.0;
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
                    pos_next[0] += 1.0;
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
        transform.translation = Vec3::new(pos_next[0], 0.0, pos_next[1]);
        data.pos_prev = data.pos_current;
        data.pos_current = pos_next;
    }
}
