use super::*;
use components::{addons, menubar, modebar, settingsbar, taskbar, toolbar};
use egui::Margin;

pub struct Screen {
    addons: addons::Addons,

    quick_settings: settingsbar::Settingsbar,
    menubar: menubar::Menubar,
    taskbar: taskbar::Taskbar,
    modebar: modebar::Modebar,
    toolbar: toolbar::Toolbar,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            addons: addons::Addons::new(),
            quick_settings: settingsbar::Settingsbar::new(),
            menubar: menubar::Menubar::new(),
            taskbar: taskbar::Taskbar::new(),
            modebar: modebar::Modebar::new(),
            toolbar: toolbar::Toolbar::new(),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, ui_data: &mut UiData) {
        let frame = egui::containers::Frame {
            fill: egui::Color32::TRANSPARENT,
            outer_margin: Margin::symmetric(10.0, 10.0),
            ..Default::default()
        };

        if *self.menubar.get_enabled_mut() {
            self.menubar.show(ctx, ui_data);
        }

        if *self.taskbar.get_enabled_mut() {
            self.taskbar.show(ctx, ui_data);
        }

        if *self.quick_settings.get_enabled_mut() {
            self.quick_settings.show(ctx, ui_data);
        }

        if *self.modebar.get_enabled_mut() {
            self.modebar.show(ctx, ui_data);
        }

        if *self.toolbar.get_enabled_mut() {
            self.toolbar.show(ctx, ui_data);
        }

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            /*
            self.icontable
                .get_orientation_icon(crate::view::Orientation::Default)
                .show(ui);
            */
            self.addons.show(ctx, ui, ui_data);

            /*
            for component in self.inner_components.iter_mut() {
                if *component.get_enabled_mut() {
                    component.show(ctx, ui, ui_ctx);
                }
            }
            */
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
