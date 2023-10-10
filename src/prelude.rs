use std::time::Instant;

pub use crate::error::Error;

#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, Error>;

use bevy::prelude::*;

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Context::new())
            .add_systems(Startup, window::init_window)
            .add_systems(Update, update_context);
    }
}

#[allow(dead_code)]
#[derive(Debug, Resource)]
pub struct Context {
    now: Instant,
    latest: Instant,
}

impl Context {
    pub fn new() -> Self {
        Self {
            now: Instant::now(),
            latest: Instant::now(),
        }
    }

    pub fn fps(&self) -> f32 {
        1.0 / (self.now - self.latest).as_secs_f32()
    }

    pub fn update(&mut self) {
        self.latest = self.now;
        self.now = Instant::now();
    }
}

fn update_context(mut context: ResMut<Context>) {
    context.update();
}

mod window {
    use bevy::{
        prelude::{Entity, NonSend, Query, With},
        window::PrimaryWindow,
        winit::WinitWindows,
    };
    use winit::window::Icon;

    pub fn init_window(
        windows: NonSend<WinitWindows>,
        primary_window_query: Query<Entity, With<PrimaryWindow>>,
    ) {
        let primary_window_entity = primary_window_query.single();
        let primary_window = windows.get_window(primary_window_entity).unwrap();
        primary_window.set_visible(true);

        // here we use the `image` crate to load our icon data from a png file
        // this is not a very bevy-native solution, but it will do
        let (icon_rgba, icon_width, icon_height) = {
            let image = image::open(uni_path::Path::new("assets/icons/main_icon.png").to_str())
                .expect("Failed to open icon path")
                .into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };

        let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

        primary_window.set_window_icon(Some(icon));
    }
}
