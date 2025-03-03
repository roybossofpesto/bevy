//! offroad ftw

mod scene;
mod simu;
mod track;
mod track_datas;
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

    app.add_plugins(scene::ScenePlugin);
    app.add_plugins(simu::SimuPlugin);
    app.add_plugins(track::TrackPlugin);
    app.add_plugins(vehicle::VehiclePlugin);

    app.run();
}
