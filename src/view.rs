use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig};

pub mod camera;

#[derive(Resource)]
pub struct ViewInterface {
    new_view_color: Option<Color>,
}

impl ViewInterface {
    pub fn new() -> Self {
        Self {
            new_view_color: None
        }
    }

    pub fn change_view_color(&mut self, r: f32, g: f32, b: f32) {
        self.new_view_color = Some(Color::rgb(r, g, b));
    }

    pub fn need_view_color_changed(&mut self) -> Option<Color> {
        self.new_view_color
    }

    pub fn reset_need_view_color_changed(&mut self) {
        self.new_view_color = None;
    }
}

pub fn view_frame(mut camera_query: Query<&mut Camera3d>, mut view_interface: ResMut<ViewInterface>) {
    if let Some(color) = view_interface.need_view_color_changed() {
        view_interface.reset_need_view_color_changed();

        camera_query.for_each_mut(|mut camera| {
            camera.clear_color = ClearColorConfig::Custom(color)
        });
    }
}

pub fn light_setup(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

pub fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default())
        .insert(camera::CameraBundle::new(
            camera::CameraController::default(),
            Vec3::new(-2.0, 5.0, 5.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));
}