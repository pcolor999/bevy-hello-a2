use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Add a 2D Camera (Notice we just use `Camera2d` now instead of the bundle)
    commands.spawn(Camera2d);

    // Add a red square (Notice we just use `Sprite` now)
    commands.spawn(Sprite {
        color: Color::srgb(1.0, 0.0, 0.0),
        custom_size: Some(Vec2::new(100.0, 100.0)),
        ..default()
    });
}