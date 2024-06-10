use super::*;
use components::{addons, menubar, modebar, quick_settingsbar, taskbar, toolbar};
use egui::Margin;

pub struct Screen {
    addons: addons::Addons,

    quick_settings: quick_settingsbar::Settingsbar,
    menubar: menubar::Menubar,
    taskbar: taskbar::Taskbar,
    modebar: modebar::Modebar,
    toolbar: toolbar::Toolbar,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            addons: addons::Addons::new(),
            quick_settings: quick_settingsbar::Settingsbar::new(),
            menubar: menubar::Menubar::new(),
            taskbar: taskbar::Taskbar::new(),
            modebar: modebar::Modebar::new(),
            toolbar: toolbar::Toolbar::new(),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, shared_state: &(UiState, GlobalState<RootEvent>)) {
        let frame = egui::containers::Frame {
            fill: egui::Color32::TRANSPARENT,
            outer_margin: Margin::symmetric(10.0, 10.0),
            ..Default::default()
        };

        self.menubar.show(ctx, shared_state);

        self.taskbar.show(ctx, shared_state);

        self.quick_settings.show(ctx, shared_state);

        self.toolbar.show(ctx, shared_state);

        self.modebar.show(ctx, shared_state);

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            self.addons.show(ui, shared_state);
        });
    }

    pub fn construct_viewport(&self, wgpu_context: &WgpuContext) -> (f32, f32, f32, f32) {
        let height = wgpu_context.surface_config.height as f32
            - self.taskbar.get_boundary().get_height()
            - self.modebar.get_boundary().get_height()
            - self.menubar.get_boundary().get_height();

        let viewport = (
            self.toolbar.get_boundary().get_width(),
            self.taskbar.get_boundary().get_height() + self.modebar.get_boundary().get_height(),
            wgpu_context.surface_config.width as f32
                - self.toolbar.get_boundary().get_width()
                - self.quick_settings.get_boundary().get_width(),
            height,
        );

        viewport
    }
}
