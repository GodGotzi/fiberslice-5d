use egui_toast::Toast;

pub const PROGRESS_BAR_TOAST: u32 = 0;

pub fn progress_bar_toast(ui: &mut egui::Ui, toast: &mut Toast) -> egui::Response {
    egui::Frame::window(ui.style())
        .show(ui, |ui| ui.add(egui::widgets::ProgressBar::new(0.25)))
        .response
}
