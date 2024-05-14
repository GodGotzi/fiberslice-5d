use super::*;
use components::{addons, menubar, modebar, settingsbar, taskbar, toolbar};
use three_d::Viewport;

pub struct Screen {
    inner_components: Vec<Box<dyn InnerComponent>>,

    quick_settings: settingsbar::Settingsbar,
    menubar: menubar::Menubar,
    taskbar: taskbar::Taskbar,
    modebar: modebar::Modebar,
    toolbar: toolbar::Toolbar,
}

impl Screen {
    pub fn new() -> Self {
        let inner_components: Vec<Box<dyn InnerComponent>> = vec![Box::new(addons::Addons::new())];

        Self {
            inner_components,
            quick_settings: settingsbar::Settingsbar::new(),
            menubar: menubar::Menubar::new(),
            taskbar: taskbar::Taskbar::new(),
            modebar: modebar::Modebar::new(),
            toolbar: toolbar::Toolbar::new(),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, ui_ctx: &mut UiData) {
        let frame = egui::containers::Frame {
            fill: egui::Color32::TRANSPARENT,
            ..Default::default()
        };

        if *self.quick_settings.get_enabled_mut() {
            self.quick_settings.show(ctx, ui_ctx);
        }

        if *self.menubar.get_enabled_mut() {
            self.menubar.show(ctx, ui_ctx);
        }

        if *self.taskbar.get_enabled_mut() {
            self.taskbar.show(ctx, ui_ctx);
        }

        if *self.modebar.get_enabled_mut() {
            self.modebar.show(ctx, ui_ctx);
        }

        if *self.toolbar.get_enabled_mut() {
            self.toolbar.show(ctx, ui_ctx);
        }

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            /*
            self.icontable
                .get_orientation_icon(crate::view::Orientation::Default)
                .show(ui);
            */

            for component in self.inner_components.iter_mut() {
                if *component.get_enabled_mut() {
                    component.show(ctx, ui, ui_ctx);
                }
            }
        });
    }

    pub fn construct_viewport(&self, frame_input: &FrameInput) -> Viewport {
        let height = frame_input.viewport.height
            - ((self.taskbar.get_boundary().get_height()
                + self.modebar.get_boundary().get_height()
                + self.menubar.get_boundary().get_height())
                * frame_input.device_pixel_ratio) as u32;
        //let extra = (height as f32 * 0.3) as u32;

        let viewport = Viewport {
            x: (self.toolbar.get_boundary().get_width() * frame_input.device_pixel_ratio) as i32,
            y: (((self.taskbar.get_boundary().get_height()
                + self.modebar.get_boundary().get_height())
                * frame_input.device_pixel_ratio) as i32),
            width: frame_input.viewport.width
                - ((self.toolbar.get_boundary().get_width()
                    + self.quick_settings.get_boundary().get_width())
                    * frame_input.device_pixel_ratio) as u32,
            height,
        };

        viewport
    }
}
