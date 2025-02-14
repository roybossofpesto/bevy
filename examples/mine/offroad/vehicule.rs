use bevy::prelude::*;

use bevy::color::palettes::basic::PURPLE;
use bevy::color::palettes::basic::YELLOW;

//////////////////////////////////////////////////////////////////////

pub struct VehiculePlugin;

impl Plugin for VehiculePlugin {
    fn build(&self, app: &mut App) {
        info!("** build_vehicule **");

        app.add_systems(Startup, setup_vehicule);
    }
    // fn finish(&self, app: &mut App) {
    //     info!("** simu_finish **");
    //     let render_app = app.sub_app_mut(RenderApp);
    //     render_app.init_resource::<SimuPipeline>();
    // }
}

//////////////////////////////////////////////////////////////////////

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
    ));
    commands.spawn((
        Mesh3d(my_mesh),
        MeshMaterial3d(materials.add(Color::from(PURPLE))),
        Transform::from_translation(blue_pos)
            .looking_at(blue_pos + Vec3::Z, Vec3::Y)
            .with_scale(Vec3::ONE * 0.15),
    ));
}
