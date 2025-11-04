use bevy::prelude::*;

mod camera;
mod components;
mod game;
mod resources;
mod states;
mod ui;

use camera::CameraPlugin;
use game::GamePlugin;
use states::GameState;
use ui::UIPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "racecar".to_string(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_plugins((GamePlugin, CameraPlugin, UIPlugin))
        .run();
}
