use crate::track;

use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

use bevy::prelude::*;

// const PINK: Color = Color::hsv(270.0, 0.27, 0.87);

use bevy::color::palettes::basic::LIME;
use bevy::color::palettes::basic::YELLOW;
use bevy::color::palettes::css::GOLD;
use bevy::color::palettes::css::LIGHT_PINK;

use std::f32::consts::PI;

//////////////////////////////////////////////////////////////////////

pub struct VehiclePlugin;

impl Plugin for VehiclePlugin {
    fn build(&self, app: &mut App) {
        info!("** build_vehicle **");

        app.add_systems(Startup, setup_vehicles);
        app.add_systems(Update, update_vehicle_physics);
        app.add_systems(Update, resolve_checkpoints);
        app.add_systems(Update, update_best_place);
    }
}

//////////////////////////////////////////////////////////////////////

#[derive(Clone)]
enum Player {
    One,
    Two,
    Three,
}

impl fmt::Display for Player {
    fn fmt(&self, buffer: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::One => write!(buffer, "P1"),
            Player::Two => write!(buffer, "P2"),
            Player::Three => write!(buffer, "P3"),
        }
    }
}

#[derive(Clone)]
struct LapStat {
    top_start: Duration,
    checkpoint_to_tops: HashMap<u8, Duration>,
    top_finish: Duration,
}

impl LapStat {
    fn from(top: Duration) -> Self {
        Self {
            top_start: top,
            checkpoint_to_tops: HashMap::new(),
            top_finish: top,
        }
    }

    fn elapsed_secs(&self) -> f32 {
        if self.top_start == Duration::MAX || self.top_finish == Duration::MAX {
            return 0.0;
        }
        if self.top_start == self.top_finish {
            return 0.0;
        }
        assert!(self.top_start != Duration::MAX);
        assert!(self.top_finish != Duration::MAX);
        assert!(self.top_start < self.top_finish);
        (self.top_finish - self.top_start).as_secs_f32()
    }
}

#[derive(Component, Clone)]
struct BoatData {
    player: Player,
    position_previous: Vec2,
    position_current: Vec2,
    angle_current: f32,
    current_stat: LapStat,
    maybe_best_stat: Option<LapStat>,
    lap_count: u32,
}

impl BoatData {
    fn from_player(player: Player) -> Self {
        const POS_P1: Vec3 = Vec3::new(-11.5, 0.0, 0.0);
        const POS_P2: Vec3 = Vec3::new(-12.5, 0.0, 0.0);
        const POS_P3: Vec3 = Vec3::new(-12.0, 0.0, 0.0);
        match player {
            Player::One => BoatData {
                player: Player::One,
                position_previous: POS_P1.xz(),
                position_current: POS_P1.xz(),
                angle_current: PI,
                current_stat: LapStat::from(Duration::MAX),
                maybe_best_stat: None,
                lap_count: 0,
            },
            Player::Two => BoatData {
                player: Player::Two,
                position_previous: POS_P2.xz(),
                position_current: POS_P2.xz(),
                angle_current: PI,
                current_stat: LapStat::from(Duration::MAX),
                maybe_best_stat: None,
                lap_count: 0,
            },
            Player::Three => BoatData {
                player: Player::Three,
                position_previous: POS_P3.xz(),
                position_current: POS_P3.xz(),
                angle_current: PI,
                current_stat: LapStat::from(Duration::MAX),
                maybe_best_stat: None,
                lap_count: 0,
            },
        }
    }
}

#[derive(Component)]
struct StatusMarker;

#[derive(Component)]
struct FirstPlaceMarker;

fn setup_vehicles(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    server: Res<AssetServer>,
) {
    info!("** setup_vehicles **");

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
        Mesh3d(my_mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::from(LIGHT_PINK),
            ..StandardMaterial::default()
        })),
        Transform::from_scale(Vec3::ONE * 0.15),
        BoatData::from_player(Player::Two),
    ));
    commands.spawn((
        Mesh3d(my_mesh),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::from(LIME),
            ..StandardMaterial::default()
        })),
        Transform::from_scale(Vec3::ONE * 0.15),
        BoatData::from_player(Player::Three),
    ));

    commands.spawn((
        Text::new("best"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
        TextFont {
            // font_color: Color::from(RED),
            font_size: 25.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        TextColor(GOLD.into()),
        FirstPlaceMarker,
    ));

    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(75.0),
            ..default()
        })
        .with_children(|parent| {
            let node = Node {
                margin: UiRect {
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            };
            let font = TextFont {
                font_size: 16.0,
                ..default()
            };
            let layout = TextLayout::new_with_justify(JustifyText::Right);
            parent.spawn((
                Text::new("status p1"),
                font.clone(),
                layout,
                node.clone(),
                StatusMarker,
            ));
            parent.spawn((
                Text::new("status p2"),
                font.clone(),
                layout,
                node.clone(),
                StatusMarker,
            ));
            parent.spawn((
                Text::new("status p3"),
                font.clone(),
                layout,
                node.clone(),
                StatusMarker,
            ));
        });
}

fn update_best_place(
    // boats: Query<&BoatData>,
    first_place_labels: Query<&mut Text, With<FirstPlaceMarker>>,
) {
    let mut foo: Option<(Duration, Player)> = None;
    /*
    for boat in &boats {
        let Some(best_stat) = &boat.maybe_best_stat else {
            return;
        };

        let lap_duration = best_stat.top_finish - best_stat.top_start;

        best_duration_player = match best_duration_player {
            Some(foo) => {
                if (best_lap_duration > foo.0) {
                    Some((best_lap_duration, boat.player.clone()))
                } else {
                    Some(foo)
                }
            }
            None => Some((best_lap_duration, boat.player.clone())),
        };
    }
    */

    let name = match foo {
        Some((_, player)) => format!("{}", player),
        None => "??".into(),
    };

    for mut first_place_label in first_place_labels {
        *first_place_label = format!("BEST\n{}", name).into();
    }

}

fn resolve_checkpoints(
    mut boats: Query<&mut BoatData>,
    status_labels: Query<&mut Text, With<StatusMarker>>,
    tracks: Res<Assets<track::Track>>,
    time: Res<Time>,
) {
    let Some(track) = tracks.get(&track::TRACK0_HANDLE) else {
        return;
    };

    if boats.is_empty() {
        return;
    }

    assert!(track.is_looping);
    assert!(!track.track_kdtree.is_empty());
    assert!(!track.checkpoint_kdtree.is_empty());

    // bounce track boundary
    for mut boat in &mut boats {
        let query_segment =
            track::Segment::from_endpoints(boat.position_current, boat.position_previous);
        let closest_segment = track.track_kdtree.nearest(&query_segment).unwrap();
        assert!(query_segment.ii == 255);
        assert!(closest_segment.item.ii == 0 || closest_segment.item.ii == 1);
        if track::Segment::clips(closest_segment.item, &query_segment) {
            boat.position_previous = closest_segment.item.mirror(boat.position_previous);
            boat.position_current = closest_segment.item.mirror(boat.position_current);
        }
    }

    // update crossed checkpoints
    let top_now = time.elapsed();
    for mut boat in &mut boats {
        boat.current_stat.top_finish = top_now;
        let query_segment =
            track::Segment::from_endpoints(boat.position_current, boat.position_previous);
        let closest_segment = track.checkpoint_kdtree.nearest(&query_segment).unwrap();
        assert!(query_segment.ii == 255);
        assert!(closest_segment.item.ii != 255);
        if closest_segment.item.intersects(&query_segment) {
            if closest_segment.item.ii == 0 {
                if boat.current_stat.top_start == Duration::MAX {
                    boat.current_stat.top_start = top_now;
                } else {
                    let mut crossed_all_checkpoints = true;
                    for kk in 1..track.checkpoint_count {
                        crossed_all_checkpoints &=
                            boat.current_stat.checkpoint_to_tops.contains_key(&kk);
                    }
                    if crossed_all_checkpoints {
                        warn!(
                            "player {} completed a lap in {:>6.3}",
                            boat.player,
                            boat.current_stat.elapsed_secs(),
                        );
                        boat.maybe_best_stat = Some(match &boat.maybe_best_stat {
                            None => boat.current_stat.clone(),
                            Some(best_stat) => {
                                if boat.current_stat.elapsed_secs() < best_stat.elapsed_secs() {
                                    boat.current_stat.clone()
                                } else {
                                    best_stat.clone()
                                }
                            }
                        });
                        boat.lap_count += 1;
                        boat.current_stat = LapStat::from(top_now);
                    }
                }
            } else {
                boat.current_stat
                    .checkpoint_to_tops
                    .insert(closest_segment.item.ii, top_now);
            }
        }
    }

    // prepare ui status label
    assert!(boats.iter().len() == status_labels.iter().len());
    for (boat, mut status_label) in boats.iter().zip(status_labels) {
        let mut ss: Vec<String> = vec![];
        ss.push(format!(
            "{} lap{} {:>6.3} {:>6.3}",
            boat.player,
            boat.lap_count,
            boat.current_stat.elapsed_secs(),
            match &boat.maybe_best_stat {
                None => 0.0,
                Some(best_stat) => best_stat.elapsed_secs(),
            },
        ));
        match &boat.maybe_best_stat {
            None => {
                for kk in 1..track.checkpoint_count {
                    ss.push(match boat.current_stat.checkpoint_to_tops.get(&kk) {
                        Some(duration) => format!(
                            "#{} {:>6.3}       ",
                            kk,
                            (*duration - boat.current_stat.top_start).as_secs_f32()
                        ),
                        None => "_       ".into(),
                    });
                }
            }
            Some(best_stat) => {
                for kk in 1..track.checkpoint_count {
                    let best_duration = best_stat.checkpoint_to_tops.get(&kk).unwrap();
                    let best_delta = (*best_duration - best_stat.top_start).as_secs_f32();
                    ss.push(match boat.current_stat.checkpoint_to_tops.get(&kk) {
                        Some(current_duration) => {
                            let current_delta =
                                (*current_duration - boat.current_stat.top_start).as_secs_f32();
                            format!(
                                "#{} {:>6.3} {:>+5.3}",
                                kk,
                                current_delta,
                                current_delta - best_delta,
                            )
                        }
                        None => format!("_ {:>6.3}", best_delta),
                    });
                }
            }
        }
        *status_label = ss.join("\n").into();
    }
}

fn update_vehicle_physics(
    mut boats: Query<(&mut BoatData, &mut Transform)>,
    mut materials: ResMut<Assets<track::RacingLineMaterial>>,
    material_handles: Query<&MeshMaterial3d<track::RacingLineMaterial>>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<(Entity, &Gamepad)>,
) {
    struct BoatPhysics {
        mass: f32,
        friction: Vec2,
        thrust: f32,
        brake: f32,
        turning_speed: f32,
        force: Vec2,
        dt: f32,
    }

    impl BoatPhysics {
        fn from_dt(dt: f32) -> Self {
            Self {
                mass: 100.0,                     // kg
                friction: Vec2::new(5e-2, 1e-2), // 0 <= f < 1
                thrust: 1500.0,                  // m / s^2 / kg ~ N
                brake: 800.0,                    // m / s^2 / kg ~ N
                turning_speed: 5.0 * PI / 4.0,   // rad / s
                force: Vec2::ZERO,               // m / s^2 /kg ~ N
                dt,                              // s
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
            let maybe_best_stat = boat.maybe_best_stat.clone();
            *boat = BoatData::from_player(player);
            boat.maybe_best_stat = maybe_best_stat;
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
                    physics.force += physics.thrust * dir_current;
                }
                if keyboard.pressed(KeyCode::ArrowDown) {
                    // physics.friction = Vec2::ONE * 0.10;
                    physics.force -= physics.brake * dir_current;
                }
            }
            Player::Three => {
                if keyboard.pressed(KeyCode::KeyA) {
                    boat.angle_current += physics.turning_speed * dt;
                }
                if keyboard.pressed(KeyCode::KeyD) {
                    boat.angle_current -= physics.turning_speed * dt;
                }
                let dir_current = Vec2::from_angle(3.0 * PI / 2.0 - boat.angle_current);
                if keyboard.pressed(KeyCode::KeyW) {
                    physics.force += physics.thrust * dir_current;
                }
                if keyboard.pressed(KeyCode::KeyS) {
                    // physics.friction = Vec2::ONE * 0.10;
                    physics.force -= physics.brake * dir_current;
                }
            }
            Player::Two => {
                for (_, gamepad) in &gamepads {
                    let left_stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap();
                    if left_stick_x.abs() > 0.01 {
                        boat.angle_current -= physics.turning_speed * left_stick_x * dt;
                    }
                    let dir_current = Vec2::from_angle(3.0 * PI / 2.0 - boat.angle_current);
                    if gamepad.pressed(GamepadButton::East) {
                        physics.force += physics.thrust * dir_current;
                    }
                    if gamepad.pressed(GamepadButton::North) {
                        // physics.friction = Vec2::ONE * 0.10;
                        physics.force -= physics.brake * dir_current;
                    }
                }
            }
        };
        let pos_next = physics.compute_next_pos(pos_prev, pos_current, boat.angle_current);
        boat.position_previous = boat.position_current;
        boat.position_current = pos_next;
        transform.translation = Vec3::new(pos_next.x, 0.0, pos_next.y);
        transform.rotation = Quat::from_axis_angle(Vec3::Y, boat.angle_current);
        if let Player::One = boat.player {
            for material_handle in material_handles.iter() {
                if let Some(material) = materials.get_mut(material_handle) {
                    let mut pos = pos_next;
                    pos -= vec2(-12.0, 0.0);
                    pos.x = -pos.x;
                    material.cursor_position = pos;
                }
            }
        }
    }
}
