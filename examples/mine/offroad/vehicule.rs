use bevy::prelude::*;

use bevy::color::palettes::basic::PURPLE;
use bevy::color::palettes::basic::YELLOW;
use std::f32::consts::PI;

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

#[derive(Clone)]
enum Player {
    One,
    Two,
}

#[derive(Component)]
struct BoatData {
    player: Player,
    position_prev: [f32; 2],
    position_current: [f32; 2],
    angle_current: f32,
}

impl BoatData {
    fn from_player(player: Player) -> Self {
        const POS_P1: Vec3 = Vec3::new(-11.5, 0.0, 0.0);
        const POS_P2: Vec3 = Vec3::new(-12.5, 0.0, 0.0);
        match player {
            Player::One => BoatData {
                player: Player::One,
                position_prev: POS_P1.xz().into(),
                position_current: POS_P1.xz().into(),
                angle_current: PI,
            },
            Player::Two => BoatData {
                player: Player::Two,
                position_prev: POS_P2.xz().into(),
                position_current: POS_P2.xz().into(),
                angle_current: PI,
            },
        }
    }
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

    commands.spawn((
        Mesh3d(my_mesh.clone()),
        MeshMaterial3d(materials.add(Color::from(YELLOW))),
        Transform::from_scale(Vec3::ONE * 0.15),
        BoatData::from_player(Player::One),
    ));
    commands.spawn((
        Mesh3d(my_mesh),
        MeshMaterial3d(materials.add(Color::from(PURPLE))),
        Transform::from_scale(Vec3::ONE * 0.15),
        BoatData::from_player(Player::Two),
    ));
}

fn update_vehicule_physics(
    mut query: Query<(&mut BoatData, &mut Transform)>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    struct BoatPhysics {
        mass: f32,
        friction: Vec2,
        thrust: f32,
        turning_speed: f32,
        accel: Vec2,
        dt: f32,
    }

    impl BoatPhysics {
        fn from_dt(dt: f32) -> Self {
            Self {
                mass: 100.0,
                friction: Vec2::new(5e-2, 10e-3),
                thrust: 500.0,
                turning_speed: PI / 2.0,
                accel: Vec2::ZERO,
                dt,
            }
        }
    }

    impl BoatPhysics {
        fn compute_next_pos(&self, pos_prev: Vec2, pos_current: Vec2, angle_current: f32) -> Vec2 {
            let accel = self.accel / self.mass / 2.0;
            let pp = Mat2::from_angle(angle_current);
            let friction = pp.transpose() * Mat2::from_diagonal(self.friction) * pp;
            (2.0 * Mat2::IDENTITY - friction) * pos_current
                - (1.0 * Mat2::IDENTITY - friction) * pos_prev
                + accel * self.dt * self.dt
        }
    }

    let dt = time.delta_secs();
    for (mut data, mut transform) in &mut query {
        if keyboard.just_pressed(KeyCode::KeyR) {
            let player = data.player.clone();
            *data = BoatData::from_player(player);
        }
        let pos_prev = Vec2::from_array(data.position_prev);
        let pos_current = Vec2::from_array(data.position_current);
        let mut physics = BoatPhysics::from_dt(dt);
        match data.player {
            Player::One => {
                if keyboard.pressed(KeyCode::ArrowLeft) {
                    data.angle_current += physics.turning_speed * dt;
                }
                if keyboard.pressed(KeyCode::ArrowRight) {
                    data.angle_current -= physics.turning_speed * dt;
                }
                let dir_current = Vec2::from_angle(3.0 * PI / 2.0 - data.angle_current);
                if keyboard.pressed(KeyCode::ArrowUp) {
                    physics.accel += physics.thrust * dir_current
                }
                if keyboard.pressed(KeyCode::ArrowDown) {
                    physics.friction = Vec2::ONE * 0.10;
                }
            }
            Player::Two => {
                if keyboard.pressed(KeyCode::KeyA) {
                    data.angle_current += physics.turning_speed * dt;
                }
                if keyboard.pressed(KeyCode::KeyD) {
                    data.angle_current -= physics.turning_speed * dt;
                }
                let dir_current = Vec2::from_angle(3.0 * PI / 2.0 - data.angle_current);
                if keyboard.pressed(KeyCode::KeyW) {
                    physics.accel += physics.thrust * dir_current
                }
                if keyboard.pressed(KeyCode::KeyS) {
                    physics.friction = Vec2::ONE * 0.10;
                }
            }
        };
        let pos_next = physics.compute_next_pos(pos_prev, pos_current, data.angle_current);
        data.position_prev = data.position_current;
        data.position_current = pos_next.into();
        transform.translation = Vec3::new(pos_next.x, 0.0, pos_next.y);
        transform.rotation = Quat::from_axis_angle(Vec3::Y, data.angle_current);
    }
}
