use bevy::prelude::*;

mod main_menu;
mod hud;

use main_menu::MainMenuPlugin;
use hud::HudPlugin;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MainMenuPlugin, HudPlugin));
    }
}

