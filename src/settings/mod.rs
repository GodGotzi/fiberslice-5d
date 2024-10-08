use egui::{DragValue, Response, TextEdit, Ui};

use crate::ui::InnerComponent;

pub mod tree;

impl InnerComponent for slicer::Settings {
    fn show(
        &mut self,
        ui: &mut egui::Ui,
        (ui_state, shared_state): &(crate::ui::UiState, crate::GlobalState<crate::RootEvent>),
    ) {
        show_f32(&mut self.layer_height, "", Some("mm"), ui);
        show_f32(&mut self.extrusion_width, "", Some("mm"), ui);
    }
}

pub fn show_str(str: &mut String, description: &str, unit: Option<&str>, ui: &mut Ui) -> Response {
    crate::config::gui::settings::SETTINGS_LABEL.label(ui, description);

    let response = ui.add(TextEdit::singleline(str));

    if let Some(unit) = unit {
        ui.label(unit);
    }

    response
}

pub fn show_f32(f32: &mut f64, description: &str, unit: Option<&str>, ui: &mut Ui) -> Response {
    crate::config::gui::settings::SETTINGS_LABEL.label(ui, description);

    let response = ui.add(DragValue::new(f32).max_decimals(3));

    if let Some(unit) = unit {
        ui.label(unit);
    }

    response
}

pub fn show_bool(bool: &mut bool, description: &str, unit: Option<&str>, ui: &mut Ui) -> Response {
    crate::config::gui::settings::SETTINGS_LABEL.label(ui, description);

    let response = ui.checkbox(bool, "");

    if let Some(unit) = unit {
        ui.label(unit);
    }

    response
}
