use super::*;
use components::{addons, menubar, modebar, settingsbar, taskbar, toolbar};

pub struct Screen {
    settings: settingsbar::Settingsbar,
    addons: addons::Addons,
    menubar: menubar::Menubar,
    taskbar: taskbar::Taskbar,
    modebar: modebar::Modebar,
    toolbar: toolbar::Toolbar,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            settings: settingsbar::Settingsbar::new(),
            addons: addons::Addons::new(),
            menubar: menubar::Menubar::new(),
            taskbar: taskbar::Taskbar::new(),
            modebar: modebar::Modebar::new(),
            toolbar: toolbar::Toolbar::new(),
        }
    }
}

impl SuperComponent for Screen {
    fn show(&mut self, ctx: &egui::Context, ui_ctx: &mut UiData) {
        let frame = egui::containers::Frame {
            fill: egui::Color32::TRANSPARENT,
            ..Default::default()
        };

        self.menubar.show(ctx, ui_ctx);

        if ui_ctx.borrow_ui_state().components.taskbar.enabled {
            self.taskbar.show(ctx, ui_ctx);
        }

        //self.addons.show(ctx, None, app);
        if ui_ctx.borrow_ui_state().components.settingsbar.enabled {
            self.settings.show(ctx, ui_ctx);
        }

        if ui_ctx.borrow_ui_state().components.toolbar.enabled {
            self.toolbar.show(ctx, ui_ctx);
        }

        if ui_ctx.borrow_ui_state().components.modebar.enabled {
            self.modebar.show(ctx, ui_ctx);
        }

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            /*
            self.icontable
                .get_orientation_icon(crate::view::Orientation::Default)
                .show(ui);
            */

            let now = std::time::Instant::now();

            if ui_ctx.borrow_ui_state().components.addons.enabled {
                self.addons.show(ctx, ui, ui_ctx);
            }

            println!("addons: {:?}", now.elapsed());
        });
    }
}
