use egui_extras::RetainedImage;
use three_d::egui::{Color32, ImageButton, Response, TextureId, Ui};

pub struct ResponsiveButton {
    image_button: ImageButton,
    size: (f32, f32),
    color: Color32,
}

impl ResponsiveButton {
    pub fn new(
        icon: &RetainedImage,
        texture_id: TextureId,
        size: (f32, f32),
        color: Color32,
    ) -> Self {
        Self {
            image_button: ImageButton::new(texture_id, icon.size_vec2()).frame(false),
            size,
            color,
        }
    }

    pub fn show(&self, hovered: bool, ui: &mut Ui) -> Response {
        if hovered {
            ui.painter()
                .rect_filled(ui.available_rect_before_wrap(), 0.0, self.color);
        }

        ui.add_sized([30., 30.], self.image_button.clone())
    }
}
