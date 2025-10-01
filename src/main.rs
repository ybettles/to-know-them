use bevy::{prelude::*, window::WindowResolution};
use std::{sync::LazyLock};

static GAME_BACKGROUND_COLOR: LazyLock<Color> = LazyLock::new(|| Color::srgb_u8(56, 47, 30));

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("To Know Them {}", env!("CARGO_PKG_VERSION")),
                resolution: WindowResolution::new(
                    1600,
                    900,
                ),
                ..default()
            }),
            ..default()
        }),
    // Add plugins here
    );
    app.add_systems(
        Startup, setup_sys,
        // Add other systems here
    );

    app.insert_resource(ClearColor(*GAME_BACKGROUND_COLOR));

    app.run();
}

fn setup_sys(
    mut cmd: Commands
) {
    cmd.spawn(Camera2d);
    // Do other stuff here
}
