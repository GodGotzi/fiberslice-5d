use bevy_egui::egui::{Ui, WidgetText};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StaticSizedLabel {
    allocate_width: f32,
}

impl StaticSizedLabel {
    pub const fn new(allocate_width: f32) -> Self {
        Self { allocate_width }
    }
}

impl StaticSizedLabel {
    pub fn label(&self, ui: &mut Ui, native: impl Into<WidgetText>) {
        let response = ui.label(native);
        ui.add_space((self.allocate_width - response.rect.width()).max(0.0))
    }
}
