//! Shows how to render simple primitive shapes with a single color.
//!
//! You can toggle wireframes with the space bar except on wasm. Wasm does not support
//! `POLYGON_MODE_LINE` on the gpu.

use bevy::animation::{animated_field, AnimationTarget, AnimationTargetId};
use bevy::color::palettes::basic::{BLUE, GRAY, GREEN, RED};
use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};
use std::f32::consts::PI;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        #[cfg(not(target_arch = "wasm32"))]
        Wireframe2dPlugin,
    ));
    app.add_systems(Startup, setup_scene);

    #[cfg(not(target_arch = "wasm32"))]
    app.add_systems(Update, toggle_wireframe);

    app.add_systems(Startup, setup_ui_widgets);
    app.add_systems(Update, update_ui_buttons);

    app.run();
}

const X_EXTENT: f32 = 900.;
const NUM_VERTICES: u32 = 16;
const ANIM_DEPTH: f32 = 1e-2;
const ANIM_DURATION: f32 = 4.0; // sec
const NUM_ANIM_STEPS: u32 = 64;

enum WidgetType {
    Button(ButtonData),
    Slider(SliderData),
}

#[derive(Clone, Component)]
struct ButtonData {
    label: &'static str,
    index: u32,
}

impl ButtonData {
    const fn new(label: &'static str, index: u32) -> Self {
        Self { label, index }
    }
}

#[derive(Clone, Component)]
struct SliderData {
    label: &'static str,
    index: u32,
    ratio: f32,
}

impl SliderData {
    const fn new(label: &'static str, index: u32) -> Self {
        Self {
            label,
            index,
            ratio: 0.5,
        }
    }
}

static UI_WIDGETS: [WidgetType; 4] = [
    WidgetType::Button(ButtonData::new("Kikou", 0)),
    WidgetType::Button(ButtonData::new("Lol", 1)),
    WidgetType::Slider(SliderData::new("AA", 0)),
    WidgetType::Slider(SliderData::new("BB", 1)),
];

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut animations: ResMut<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    info!("setup");

    commands.spawn(Camera2d);

    // meshes.add(Circle::new(50.0)),
    // meshes.add(CircularSector::new(50.0, 1.0)),
    // meshes.add(CircularSegment::new(50.0, 1.25)),
    // meshes.add(Ellipse::new(25.0, 50.0)),
    // meshes.add(Annulus::new(25.0, 50.0)),
    // meshes.add(Capsule2d::new(25.0, 50.0)),
    // meshes.add(Rhombus::new(75.0, 100.0)),
    // meshes.add(Rectangle::new(50.0, 100.0)),
    // meshes.add(RegularPolygon::new(50.0, 6)),
    // meshes.add(Triangle2d::new(
    //     Vec2::Y * 50.0,
    //     Vec2::new(-50.0, -50.0),
    //     Vec2::new(50.0, -50.0),
    // )),

    for kk in 0..NUM_VERTICES {
        let shape = meshes.add(Circle::new(10.0 + kk as f32));
        let material = materials.add(Color::hsl(36. * kk as f32, 0.95, 0.7));

        commands.spawn((
            Mesh2d(shape),
            MeshMaterial2d(material),
            Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                -X_EXTENT / 2. + kk as f32 / (NUM_VERTICES - 1) as f32 * X_EXTENT,
                0.0,
                0.0,
            ),
        ));
    }

    let planet = Name::new("planet");

    let mut tts = Vec::new();
    let mut pps = Vec::new();
    let mut rrs = Vec::new();
    for kk in 0..NUM_ANIM_STEPS {
        let aa = kk as f32 / (NUM_ANIM_STEPS - 1) as f32;
        let tt = ANIM_DURATION * aa;
        let angle = 2.0 * PI * aa;
        tts.push(tt);
        pps.push(Vec3::new(
            angle.cos() * 200.0,
            angle.sin() * 200.0,
            ANIM_DEPTH,
        ));
        rrs.push(Quat::from_axis_angle(Vec3::Z, angle + PI / 2.0));
    }

    let mut animation = AnimationClip::default();
    let planet_animation_target_id = AnimationTargetId::from_name(&planet);
    animation.add_curve_to_target(
        planet_animation_target_id,
        AnimatableCurve::new(
            animated_field!(Transform::translation),
            UnevenSampleAutoCurve::new(tts.clone().into_iter().zip(pps)).unwrap(),
        ),
    );
    animation.add_curve_to_target(
        planet_animation_target_id,
        AnimatableCurve::new(
            animated_field!(Transform::rotation),
            UnevenSampleAutoCurve::new(tts.into_iter().zip(rrs)).unwrap(),
        ),
    );

    let (graph, animation_index) = AnimationGraph::from_clip(animations.add(animation));
    let mut player = AnimationPlayer::default();
    player.play(animation_index).repeat();

    let planet_shape = meshes.add(Capsule2d::new(10.0, 50.0));
    let planet_material = materials.add(Color::srgb(1.0, 0.0, 1.0));
    let planet_entity_id = commands
        .spawn((
            Mesh2d(planet_shape),
            MeshMaterial2d(planet_material),
            planet,
            AnimationGraphHandle(graphs.add(graph)),
            player,
        ))
        .id();

    commands.entity(planet_entity_id).insert(AnimationTarget {
        id: planet_animation_target_id,
        player: planet_entity_id,
    });
}

fn update_ui_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    text_query: Query<&Text>,
) {
    for (interaction, mut border_color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let label = text_query.get(children[children.len() - 1]).unwrap();
                info!("clicked on {1:?} {0}x", children.len(), label);
                border_color.0 = RED.into();
            }
            Interaction::Hovered => {
                border_color.0 = GREEN.into();
            }
            Interaction::None => {
                border_color.0 = BLUE.into();
            }
        }
    }
}

fn setup_ui_widgets(mut commands: Commands) {
    let mut frame = commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        left: Val::Px(10.0),
        bottom: Val::Px(10.0),
        align_items: AlignItems::End,
        ..default()
    });

    frame.with_children(|parent| {
        parent.spawn((
            Text::new("Lorentz!"),
            Node {
                margin: UiRect::all(Val::Px(5.0)),
                ..default()
            },
        ));
    });

    for widget in UI_WIDGETS.iter() {
        match widget {
            WidgetType::Button(data) => {
                frame.with_children(|parent| {
                    parent
                        .spawn((
                            Button,
                            Node {
                                border: UiRect::all(Val::Px(5.0)),
                                margin: UiRect::all(Val::Px(5.0)),
                                ..default()
                            },
                        ))
                        .with_child(Text::new(format!("{} [{}]", data.label, data.index)));
                });
            }
            WidgetType::Slider(data) => {
                frame.with_children(|parent| {
                    let mut container = parent.spawn((
                        Button,
                        Node {
                            border: UiRect::all(Val::Px(5.0)),
                            margin: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                    ));

                    container.with_child((
                        Node {
                            position_type: PositionType::Absolute,
                            top: Val::Px(0.0),
                            left: Val::Px(0.0),
                            height: Val::Percent(100.0),
                            width: Val::Percent(100.0 * data.ratio),
                            ..default()
                        },
                        BackgroundColor(GRAY.into()),
                    ));

                    container.with_child(Text::new(format!("{} !{}!", data.label, data.index)));
                });
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn toggle_wireframe(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}
