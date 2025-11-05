use bevy::prelude::*;

mod hud;
mod main_menu;

use hud::HudPlugin;
use main_menu::MainMenuPlugin;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MainMenuPlugin, HudPlugin));
    }
}
