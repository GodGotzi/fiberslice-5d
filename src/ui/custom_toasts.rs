use std::sync::Arc;

use egui_toast::Toast;
use shared::process::Process;

const PROGRESS_BAR_WIDTH: f32 = 250.0;

const STAY_DURATION_MS: u64 = 1000;

pub const MODEL_LOAD_PROGRESS: u32 = 0;

pub fn model_load_progress(ui: &mut egui::Ui, toast: &mut Toast) -> egui::Response {
    let global_state = crate::GLOBAL_STATE.read();
    let global_state = global_state.as_ref().unwrap();

    global_state.progress_tracker.read_with_fn(|tracker| {
        let process = tracker.get(MODEL_LOAD_PROGRESS, toast.get_name()).unwrap();

        show_processes(ui, toast, process)
    })
}

pub const SLICING_PROGRESS: u32 = 1;

pub fn slicing_progress(ui: &mut egui::Ui, toast: &mut Toast) -> egui::Response {
    let global_state = crate::GLOBAL_STATE.read();
    let global_state = global_state.as_ref().unwrap();

    global_state.progress_tracker.read_with_fn(|tracker| {
        let process = tracker.get(SLICING_PROGRESS, toast.get_name()).unwrap();

        show_processes(ui, toast, process)
    })
}

pub fn show_processes(
    ui: &mut egui::Ui,
    toast: &mut Toast,
    process: &Arc<Process>,
) -> egui::Response {
    egui::Frame::window(ui.style())
        .show(ui, |ui| {
            if process.is_finished() && !process.is_closed() {
                toast.options = toast.options.duration_in_millis(STAY_DURATION_MS);
                toast.options.show_progress = false;

                process.close();
            }

            let progress = process.get();

            ui.label(toast.get_name());

            ui.separator();

            ui.label(process.task());

            ui.add(
                egui::widgets::ProgressBar::new(progress)
                    .show_percentage()
                    .animate(true)
                    .desired_width(PROGRESS_BAR_WIDTH),
            );
        })
        .response
}
