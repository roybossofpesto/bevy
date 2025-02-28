//! Shows how to render simple primitive shapes with a single color.
//!
//! You can toggle wireframes with the space bar except on wasm. Wasm does not support
//! `POLYGON_MODE_LINE` on the gpu.

use bevy::animation::{animated_field, AnimationTarget, AnimationTargetId};
use bevy::color::palettes::basic::{BLACK, BLUE, GRAY, GREEN, RED};
#[cfg(feature = "bevy_dev_tools")]
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::math::ops;
use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};
use bevy::ui::RelativeCursorPosition;

use core::f32::consts::PI;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        #[cfg(not(target_arch = "wasm32"))]
        Wireframe2dPlugin,
    ));
    #[cfg(feature = "bevy_dev_tools")]
    app.add_plugins(FpsOverlayPlugin {
        config: FpsOverlayConfig::default(),
    });

    app.add_systems(Startup, setup_scene);

    #[cfg(not(target_arch = "wasm32"))]
    app.add_systems(Update, toggle_wireframe);
    #[cfg(feature = "bevy_dev_tools")]
    app.add_systems(Update, toggle_fps);

    app.add_systems(Startup, setup_ui_widgets);
    app.add_systems(Update, update_ui_widget_buttons);
    app.add_systems(
        Update,
        (
            drag_ui_widget_sliders,
            update_ui_widget_sliders,
            update_lorentz_points,
        )
            .chain(),
    );

    app.run();
}

const NUM_VERTICES: u32 = 65;
const VERTICES_EXTENT: f32 = 600.;

#[derive(Component)]
struct LorentzPoint {
    init_xx: f32,
    init_yy: f32,
}

impl LorentzPoint {
    const fn new(init_xx: f32, init_yy: f32) -> Self {
        Self { init_xx, init_yy }
    }
}

const ANIM_DEPTH: f32 = 2.0;
const ANIM_DURATION: f32 = 4.0; // sec
const NUM_ANIM_STEPS: u32 = 64;

enum WidgetType {
    Button(ButtonData),
    Slider(SliderData),
}

#[derive(Clone, Component)]
struct ButtonData {
    label: &'static str,
    count: u32,
}

impl ButtonData {
    const fn new(label: &'static str) -> Self {
        Self { label, count: 0 }
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

static UI_WIDGETS: [WidgetType; 5] = [
    WidgetType::Button(ButtonData::new("Kikou")),
    WidgetType::Button(ButtonData::new("Lol")),
    WidgetType::Slider(SliderData::new("AA", 0)),
    WidgetType::Slider(SliderData::new("BB", 1)),
    WidgetType::Slider(SliderData::new("Angle", 2)),
];

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut animations: ResMut<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    info!("setup_scene");

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

    for ii in 0..NUM_VERTICES {
        for jj in 0..NUM_VERTICES {
            let alpha = (ii * NUM_VERTICES + jj) as f32 / (NUM_VERTICES * NUM_VERTICES - 1) as f32;
            let shape = meshes.add(Circle::new(2.0));
            let material = materials.add(Color::hsl(360.0 * alpha, 0.95, 0.7));

            let init_xx =
                -VERTICES_EXTENT / 2. + jj as f32 / (NUM_VERTICES - 1) as f32 * VERTICES_EXTENT;
            let init_yy =
                -VERTICES_EXTENT / 2. + ii as f32 / (NUM_VERTICES - 1) as f32 * VERTICES_EXTENT;
            commands.spawn((
                Mesh2d(shape),
                MeshMaterial2d(material),
                Transform::from_xyz(init_xx, init_yy, alpha),
                LorentzPoint::new(init_xx, init_yy),
            ));
        }
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
            ops::cos(angle) * 200.0,
            ops::sin(angle) * 200.0,
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

fn drag_ui_widget_sliders(
    mut interaction_query: Query<
        (&Interaction, &RelativeCursorPosition, &mut SliderData),
        With<Button>,
    >,
) {
    for (interaction, relative_cursor, mut data) in &mut interaction_query {
        if !matches!(*interaction, Interaction::Pressed) {
            continue;
        }
        let Some(pos) = relative_cursor.normalized else {
            continue;
        };
        data.ratio = pos.x.clamp(0.0, 1.0);
    }
}

fn update_lorentz_points(
    slider_query: Query<&SliderData, Changed<SliderData>>,
    mut point_query: Query<(&LorentzPoint, &mut Transform)>,
) {
    for slider_data in &slider_query {
        if slider_data.index != 2 {
            continue;
        };

        let angle = (slider_data.ratio - 0.5) * PI / 4.0;
        let cha = ops::cosh(angle);
        let sha = ops::sinh(angle);

        for (point_data, mut point_transform) in &mut point_query {
            let xx = cha * point_data.init_xx - sha * point_data.init_yy;
            let yy = cha * point_data.init_yy - sha * point_data.init_xx;
            *point_transform = Transform::from_xyz(xx, yy, 0.0);
        }
    }
}

fn update_ui_widget_sliders(
    query: Query<(&Children, &SliderData), Changed<SliderData>>,
    mut node_query: Query<&mut Node, Without<Text>>,
) {
    for (children, data) in &query {
        let mut node_iter = node_query.iter_many_mut(children);
        if let Some(mut node) = node_iter.fetch_next() {
            // All nodes are the same width, so `NODE_RECTS[0]` is as good as any other.
            node.width = Val::Percent(100.0 * data.ratio);
        }
    }
}

fn update_ui_widget_buttons(
    mut query: Query<
        (&Interaction, &mut BorderColor, &mut ButtonData),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut border_color, mut data) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                data.count += 1;
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

    let mut make_widget = |widget: &WidgetType| match widget {
        WidgetType::Button(data) => {
            frame.with_children(|parent| {
                let mut container = parent.spawn((
                    Button,
                    Node {
                        border: UiRect::all(Val::Px(5.0)),
                        padding: UiRect::all(Val::Px(5.0)),
                        margin: UiRect::right(Val::Px(5.0)),
                        ..default()
                    },
                    BorderColor(BLACK.into()),
                ));

                container.insert((Interaction::None, data.clone()));

                container.with_child(Text::new(data.label));
            });
        }
        WidgetType::Slider(data) => {
            frame.with_children(|parent| {
                let mut container = parent.spawn((
                    Button,
                    Node {
                        border: UiRect::all(Val::Px(5.0)),
                        padding: UiRect::all(Val::Px(5.0)),
                        margin: UiRect::right(Val::Px(5.0)),
                        width: Val::Px(100.0),
                        ..default()
                    },
                    BorderColor(BLACK.into()),
                ));

                container.insert((
                    Interaction::None,
                    RelativeCursorPosition::default(),
                    data.clone(),
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

                container.with_child(Text::new(data.label));
            });
        }
    };

    for widget in UI_WIDGETS.iter() {
        make_widget(widget);
    }

    frame.with_children(|parent| {
        parent.spawn(Text::new("hello world"));
    });
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

#[cfg(feature = "bevy_dev_tools")]
fn toggle_fps(mut fps_config: ResMut<FpsOverlayConfig>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::KeyQ) {
        fps_config.enabled = !fps_config.enabled;
    }
}
