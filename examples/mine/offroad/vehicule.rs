use bevy::prelude::*;

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

use bevy::color::palettes::basic::RED;

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
    let my_mesh: Handle<Mesh> = server.load("models/offroad/boat.glb");

    commands.spawn((
        Mesh3d(my_mesh),
        MeshMaterial3d(materials.add(Color::from(RED))),
        Transform::from_xyz(0.0, 5.0, 0.0),
    ));

    // spawn a whole scene
    // let my_scene: Handle<Scene> = server.load("my_scene.gltf#Scene0");
    // commands.spawn(bevy::prelude::SceneBundle {
    //     scene: my_scene,
    //     ..Default::default()
    // });
}
