use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;

pub struct PickPlugin;

impl Plugin for PickPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPickingPlugins);
    }
}
