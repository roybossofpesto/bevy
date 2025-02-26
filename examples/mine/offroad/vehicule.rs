use kd_tree::{KdPoint, KdTree};
use std::collections::HashSet;

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
        app.add_systems(Update, update_checkpoints);
        app.add_systems(Update, bump_vehicules);
    }
    // fn finish(&self, app: &mut App) {
    //     info!("** simu_finish **");
    //     let render_app = app.sub_app_mut(RenderApp);
    //     render_app.init_resource::<SimuPipeline>();
    // }
}

//////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
enum Player {
    One,
    Two,
}

#[derive(Component, Clone, Debug)]
struct BoatData {
    player: Player,
    position_prev: [f32; 2],
    position_current: [f32; 2],
    angle_current: f32,
    crossed_checkpoints: HashSet<u8>,
}

#[derive(Component)]
struct StatusMarker;

#[derive(Component)]
struct LabelMarker;

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
                crossed_checkpoints: HashSet::new(),
            },
            Player::Two => BoatData {
                player: Player::Two,
                position_prev: POS_P2.xz().into(),
                position_current: POS_P2.xz().into(),
                angle_current: PI,
                crossed_checkpoints: HashSet::new(),
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

    // let my_mesh: Handle<Mesh> = server.load("models/offroad/boat.gltf#Mesh0/Primitive0");
    // let my_mesh: Handle<Mesh> = server.load("models/animated/Fox.glb");
    let my_mesh: Handle<Mesh> = server.load("models/offroad/boat.glb#Mesh0/Primitive0");

    commands.spawn((
        Mesh3d(my_mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::from(YELLOW),
            alpha_mode: AlphaMode::Blend,
            ..StandardMaterial::default()
        })),
        Transform::from_scale(Vec3::ONE * 0.15),
        BoatData::from_player(Player::One),
    ));
    commands.spawn((
        Mesh3d(my_mesh),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::from(PURPLE),
            alpha_mode: AlphaMode::Blend,
            ..StandardMaterial::default()
        })),
        Transform::from_scale(Vec3::ONE * 0.15),
        BoatData::from_player(Player::Two),
    ));

    commands.spawn((
        Text::new("coucou"),
        TextFont {
            font_size: 22.0,
            ..default()
        },
        LabelMarker,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        },
    ));

    commands.spawn((
        StatusMarker,
        Text::new("status"),
        TextFont {
            font_size: 22.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        },

    ));
}

fn update_checkpoints(
    mut boat_query: Query<&mut BoatData>,
    mut status_query: Query<&mut Text, With<StatusMarker>>,
    tracks: Res<Assets<crate::track::Track>>,
) {
    let track = tracks.get(&crate::track::TRACK0_HANDLE).unwrap();
    assert!(track.is_looping);

    // for mut boat_data in &mut boat_query {
    //     info!("dksjf {:?}", boat_data.position_current);
    //     for (index, (aa, bb)) in track.checkpoint_to_segments.iter() {}
    // }

    let Ok(mut status) = status_query.get_single_mut() else {
        return;
    };

    *status = format!("p1 {} p2 {}", 12, 456).into();
}

impl KdPoint for BoatData {
    type Scalar = f32;
    type Dim = typenum::U2; // 2 dimensional tree.
    fn at(&self, k: usize) -> f32 {
        self.position_current[k]
    }
}

fn bump_vehicules(boats: Query<&BoatData>, mut labels: Query<&mut Text, With<LabelMarker>>) {
    if boats.is_empty() {
        return;
    }

    let mut items: Vec<BoatData> = vec![];
    for boat in boats {
        items.push(boat.clone());
    }

    let kdtree: KdTree<BoatData> = KdTree::build_by_ordered_float(items);

    let foo = kdtree.nearest(&[0.0, 2.0]).unwrap();

    let Ok(mut label) = labels.get_single_mut() else {
        return;
    };

    *label = format!("kdtree {:?} {:.02e}", foo.item.player, foo.squared_distance).into();
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
        force: Vec2,
        dt: f32,
    }

    impl BoatPhysics {
        fn from_dt(dt: f32) -> Self {
            Self {
                mass: 100.0,                      // kg
                friction: Vec2::new(5e-2, 10e-3), // 0 <= f < 1
                thrust: 500.0,                    // m / s^2 / kg ~ N
                turning_speed: PI / 2.0,          // rad / s
                force: Vec2::ZERO,                // m / s^2 /kg ~ N
                dt,                               // s
            }
        }
    }

    impl BoatPhysics {
        fn compute_next_pos(&self, pos_prev: Vec2, pos_current: Vec2, angle_current: f32) -> Vec2 {
            let accel = self.force / self.mass / 2.0;
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
                    physics.force += physics.thrust * dir_current
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
                    physics.force += physics.thrust * dir_current
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
