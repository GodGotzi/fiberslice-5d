use bevy_egui::egui::{Response, Ui};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StaticSizedLabel {
    spaces: usize,
}

impl StaticSizedLabel {
    pub const fn new(spaces: usize) -> Self {
        Self { spaces }
    }
}

impl StaticSizedLabel {
    pub fn label(&self, ui: &mut Ui, native: impl Into<String>) -> Response {
        let mut label = String::new();
        let native = native.into();

        label.push_str(&native);

        for _ in 0..self.spaces - native.len() {
            label.push('-');
        }
        ui.label(label)
    }
}
