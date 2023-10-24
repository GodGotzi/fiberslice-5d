use bevy::prelude::*;
use bevy_mod_raycast::{prelude::*, print_intersections};

pub struct PickPlugin;

impl Plugin for PickPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultRaycastingPlugin::<RaycastSet>::default())
            .add_systems(
                First,
                update_raycast_with_cursor.before(RaycastSystem::BuildRays::<RaycastSet>),
            )
            .add_systems(Update, print_intersections::<RaycastSet>);
    }
}

#[derive(Reflect)]
pub struct RaycastSet;

// Update our `RaycastSource` with the current cursor position every frame.
fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RaycastSource<RaycastSet>>,
) {
    // Grab the most recent cursor event if it exists:
    let Some(cursor_moved) = cursor.iter().last() else {
        return;
    };
    for mut pick_source in &mut query {
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_moved.position);
    }
}
