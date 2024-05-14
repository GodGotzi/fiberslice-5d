use three_d::egui::{self, Color32, ImageButton, Painter, Pos2, Rect, Sense, Ui, Vec2};

use crate::ui::{icon, UiData};

pub struct DecoradedButton {
    pub border: f32,
    pub size: (f32, f32),
}
