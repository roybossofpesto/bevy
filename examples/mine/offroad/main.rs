//! offroad ftw

mod scene;
mod simu;
mod track;
mod vehicle;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.insert_resource(bevy::pbr::DirectionalLightShadowMap { size: 2048 });
    app.add_plugins(DefaultPlugins);

    #[cfg(feature = "bevy_dev_tools")]
    {
        // fps overlay
        use bevy::color::palettes::basic::YELLOW;
        use bevy::dev_tools::fps_overlay::FpsOverlayConfig;
        use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;
        app.add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_color: Color::from(YELLOW),
                ..default()
            },
        });
    }

    /*
    #[cfg(feature = "bevy_dev_tools")]
    {
        // wireframe toggle
        use bevy::color::palettes::basic::WHITE;
        use bevy::pbr::wireframe::WireframeConfig;
        use bevy::pbr::wireframe::WireframePlugin;
        app.insert_resource(WireframeConfig {
            global: false,
            default_color: WHITE.into(),
        });
        app.add_plugins(WireframePlugin);
        app.add_systems(
            Update,
            |mut wireframe_config: ResMut<WireframeConfig>,
             keyboard: Res<ButtonInput<KeyCode>>|
             -> () {
                if keyboard.just_pressed(KeyCode::Space) {
                    wireframe_config.global = !wireframe_config.global;
                }
            },
        );
    }
    */

    app.add_plugins(scene::ScenePlugin);
    app.add_plugins(simu::SimuPlugin);
    app.add_plugins(track::TrackPlugin);
    app.add_plugins(vehicle::VehiclePlugin);

    app.run();
}
