use crate::track;

use std::cmp::min;
use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

use bevy::prelude::*;

use bevy::color::palettes::basic::PURPLE;
use bevy::color::palettes::basic::YELLOW;
use std::f32::consts::PI;

//////////////////////////////////////////////////////////////////////

pub struct VehiculePlugin;

impl Plugin for VehiculePlugin {
    fn build(&self, app: &mut App) {
        info!("** build_vehicule **");

        app.add_systems(Startup, setup_vehicules);
        app.add_systems(Update, update_vehicule_physics);
        app.add_systems(Update, resolve_checkpoints);
    }
}

//////////////////////////////////////////////////////////////////////

#[derive(Clone)]
enum Player {
    One,
    Two,
}

impl fmt::Display for Player {
    fn fmt(&self, buffer: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::One => write!(buffer, "P1"),
            Player::Two => write!(buffer, "P2"),
        }
    }
}

#[derive(Component, Clone)]
struct BoatData {
    player: Player,
    position_previous: Vec2,
    position_current: Vec2,
    angle_current: f32,
    crossed_checkpoints: HashMap<u8, Duration>,
    lap_count: u32,
    outside_track: bool,
}

#[derive(Component)]
struct StatusMarker;

impl BoatData {
    fn from_player(player: Player) -> Self {
        const POS_P1: Vec3 = Vec3::new(-11.5, 0.0, 0.0);
        const POS_P2: Vec3 = Vec3::new(-12.5, 0.0, 0.0);
        match player {
            Player::One => BoatData {
                player: Player::One,
                position_previous: POS_P1.xz().into(),
                position_current: POS_P1.xz().into(),
                angle_current: PI,
                crossed_checkpoints: HashMap::new(),
                lap_count: 0,
                outside_track: false,
            },
            Player::Two => BoatData {
                player: Player::Two,
                position_previous: POS_P2.xz().into(),
                position_current: POS_P2.xz().into(),
                angle_current: PI,
                crossed_checkpoints: HashMap::new(),
                lap_count: 0,
                outside_track: false,
            },
        }
    }
}

fn setup_vehicules(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    server: Res<AssetServer>,
) {
    info!("** setup_vehicules **");

    let my_mesh: Handle<Mesh> = server.load("models/offroad/boat.glb#Mesh0/Primitive0");

    commands.spawn((
        Mesh3d(my_mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::from(YELLOW),
            ..StandardMaterial::default()
        })),
        Transform::from_scale(Vec3::ONE * 0.15),
        BoatData::from_player(Player::One),
    ));
    commands.spawn((
        Mesh3d(my_mesh),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::from(PURPLE),
            ..StandardMaterial::default()
        })),
        Transform::from_scale(Vec3::ONE * 0.15),
        BoatData::from_player(Player::Two),
    ));

    commands.spawn((
        Text::new("status"),
        TextFont {
            font_size: 22.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
        StatusMarker,
    ));
}

fn resolve_checkpoints(
    mut boats: Query<&mut BoatData>,
    mut labels: Query<&mut Text, With<StatusMarker>>,
    tracks: Res<Assets<track::Track>>,
    time: Res<Time>,
) {
    let Some(track) = tracks.get(&track::TRACK0_HANDLE) else {
        return;
    };

    if boats.is_empty() {
        return;
    }

    let Ok(mut label) = labels.get_single_mut() else {
        return;
    };

    assert!(track.is_looping);
    assert!(!track.track_kdtree.is_empty());
    assert!(!track.checkpoint_kdtree.is_empty());

    let top_now = time.elapsed();

    // bounce track boundary
    for mut boat in &mut boats {
        let query_segment =
            track::Segment::from_endpoints(boat.position_current, boat.position_previous);
        let closest_segment = track.track_kdtree.nearest(&query_segment).unwrap();
        assert!(query_segment.ii == 255);
        assert!(closest_segment.item.ii == 255);
        boat.outside_track = !track::Segment::clips(closest_segment.item, &query_segment);
    }

    // update crossed checkpoints & lap counts
    for mut boat in &mut boats {
        let query_segment =
            track::Segment::from_endpoints(boat.position_current, boat.position_previous);
        let closest_segment = track.checkpoint_kdtree.nearest(&query_segment).unwrap();
        assert!(query_segment.ii == 255);
        assert!(closest_segment.item.ii != 255);
        if track::Segment::intersects(closest_segment.item, &query_segment) {
            if closest_segment.item.ii == 0 {
                let mut crossed_all_checkpoints = true;
                for kk in 0..track.checkpoint_count {
                    crossed_all_checkpoints &= boat.crossed_checkpoints.contains_key(&kk);
                }
                if crossed_all_checkpoints {
                    boat.lap_count += 1;
                    boat.crossed_checkpoints.clear();
                }
            }
            boat.crossed_checkpoints
                .insert(closest_segment.item.ii, top_now);
        }
    }

    // prepare ui status label
    let mut ss: Vec<String> = vec![];
    for boat in &boats {
        let mut lap_duration = top_now;
        let mut rr = String::new();
        for kk in 0..track.checkpoint_count {
            let foo = match boat.crossed_checkpoints.get(&kk) {
                Some(duration) => {
                    lap_duration = min(lap_duration, *duration);
                    "X"
                }
                None => "_",
            };
            rr = format!("{}{}", rr, foo)
        }
        lap_duration = top_now - lap_duration;
        ss.push(format!(
            "{} {:>6.3} {} {} {}",
            boat.player,
            lap_duration.as_secs_f32(),
            rr,
            boat.lap_count,
            match boat.outside_track {
                true => "O",
                false => "I",
            },
        ));
    }
    assert!(!label.is_empty());
    *label = ss.join("\n").into();
}

fn update_vehicule_physics(
    mut boats: Query<(&mut BoatData, &mut Transform)>,
    mut materials: ResMut<Assets<track::RacingLineMaterial>>,
    material_handles: Query<&MeshMaterial3d<track::RacingLineMaterial>>,
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
    for (mut boat, mut transform) in &mut boats {
        if keyboard.just_pressed(KeyCode::KeyR) {
            let player = boat.player.clone();
            *boat = BoatData::from_player(player);
        }
        let pos_prev = boat.position_previous;
        let pos_current = boat.position_current;
        let mut physics = BoatPhysics::from_dt(dt);
        match boat.player {
            Player::One => {
                if keyboard.pressed(KeyCode::ArrowLeft) {
                    boat.angle_current += physics.turning_speed * dt;
                }
                if keyboard.pressed(KeyCode::ArrowRight) {
                    boat.angle_current -= physics.turning_speed * dt;
                }
                let dir_current = Vec2::from_angle(3.0 * PI / 2.0 - boat.angle_current);
                if keyboard.pressed(KeyCode::ArrowUp) {
                    physics.force += physics.thrust * dir_current
                }
                if keyboard.pressed(KeyCode::ArrowDown) {
                    physics.friction = Vec2::ONE * 0.10;
                }
            }
            Player::Two => {
                if keyboard.pressed(KeyCode::KeyA) {
                    boat.angle_current += physics.turning_speed * dt;
                }
                if keyboard.pressed(KeyCode::KeyD) {
                    boat.angle_current -= physics.turning_speed * dt;
                }
                let dir_current = Vec2::from_angle(3.0 * PI / 2.0 - boat.angle_current);
                if keyboard.pressed(KeyCode::KeyW) {
                    physics.force += physics.thrust * dir_current
                }
                if keyboard.pressed(KeyCode::KeyS) {
                    physics.friction = Vec2::ONE * 0.10;
                }
            }
        };
        let pos_next = physics.compute_next_pos(pos_prev, pos_current, boat.angle_current);
        boat.position_previous = boat.position_current;
        boat.position_current = pos_next.into();
        transform.translation = Vec3::new(pos_next.x, 0.0, pos_next.y);
        transform.rotation = Quat::from_axis_angle(Vec3::Y, boat.angle_current);
        match boat.player {
            Player::One => {
                for material_handle in material_handles.iter() {
                    if let Some(material) = materials.get_mut(material_handle) {
                        let mut pos = pos_next;
                        pos -= vec2(-12.0, 0.0);
                        pos.x = -pos.x;
                        material.cursor_position = pos;
                    }
                }
            }
            _ => {}
        };
    }
}
