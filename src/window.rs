use bevy::{
    prelude::{Entity, NonSend, Query, With},
    window::PrimaryWindow,
    winit::WinitWindows,
};
use winit::window::Icon;

pub fn update_window(
    windows: NonSend<WinitWindows>,
    primary_window_query: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_window_entity = primary_window_query.single();
    let primary_window = windows.get_window(primary_window_entity).unwrap();

    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/icons/main_icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    primary_window.set_window_icon(Some(icon));
}
