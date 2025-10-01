use bevy::{prelude::*, window::WindowResolution};
use std::{sync::LazyLock};

static GAME_BACKGROUND_COLOR: LazyLock<Color> = LazyLock::new(|| Color::srgb_u8(100, 144, 199));
static TKT_GREEN: LazyLock<Color> = LazyLock::new(|| Color::srgb_u8(142, 177, 157));
static TKT_YELLOW: LazyLock<Color> = LazyLock::new(|| Color::srgb_u8(246, 239, 166));
static TKT_PINK: LazyLock<Color> = LazyLock::new(|| Color::srgb_u8(240, 210, 209));
static TKT_BLUE: LazyLock<Color> = LazyLock::new(|| Color::srgb_u8(77, 83, 130));
static TKT_VIOLET: LazyLock<Color> = LazyLock::new(|| Color::srgb_u8(81, 70, 99));

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
