//! Shows how to render simple primitive shapes with a single color.
//!
//! You can toggle wireframes with the space bar except on wasm. Wasm does not support
//! `POLYGON_MODE_LINE` on the gpu.

use bevy::animation::{animated_field, AnimationTargetId};
use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        #[cfg(not(target_arch = "wasm32"))]
        Wireframe2dPlugin,
    ))
    .add_systems(Startup, setup);
    #[cfg(not(target_arch = "wasm32"))]
    app.add_systems(Update, toggle_wireframe);
    app.run();
}

const X_EXTENT: f32 = 900.;
const NUM_VERTICES: u32 = 16;

fn setup(
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

    let mut animation = AnimationClip::default();
    let planet_animation_target_id = AnimationTargetId::from_name(&planet);
    animation.add_curve_to_target(
        planet_animation_target_id,
        AnimatableCurve::new(
            animated_field!(Transform::translation),
            UnevenSampleAutoCurve::new([0.0, 1.0, 2.0, 3.0, 4.0].into_iter().zip([
                Vec3::new(-X_EXTENT / 2., 0.0, 0.0),
                Vec3::new(-X_EXTENT / 2., 1.0, 0.0),
                Vec3::new(-X_EXTENT / 2., 0.0, 0.0),
                Vec3::new(-X_EXTENT / 2., -1.0, 0.0),
                // in case seamless looping is wanted, the last keyframe should
                // be the same as the first one
                Vec3::new(-X_EXTENT / 2., 0.0, 0.0),
            ]))
            .expect("should be able to build translation curve because we pass in valid samples"),
        ),
    );

    #[cfg(not(target_arch = "wasm32"))]
    commands.spawn((
        Text::new("Animated Lorentz transform"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
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
