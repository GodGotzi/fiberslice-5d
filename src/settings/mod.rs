use crate::ui::InnerComponent;

pub mod tree;

impl InnerComponent for slicer::Settings {
    fn show(
        &mut self,
        ui: &mut egui::Ui,
        (ui_state, shared_state): &(crate::ui::UiState, crate::GlobalState<crate::RootEvent>),
    ) {
        todo!()
    }
}
