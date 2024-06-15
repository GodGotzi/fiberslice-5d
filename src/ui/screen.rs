use super::*;
use components::{
    addons,
    menubar::{self, MenubarState},
    modebar::{self, ModebarState},
    quick_settingsbar, taskbar,
    toolbar::{self, ToolBarState},
};
use egui::{Align2, Margin};
use egui_toast::Toasts;

pub struct Screen {
    toasts: Toasts,

    tools: tools::Tools,
    addons_state: addons::AddonsState,

    quick_settings_state: quick_settingsbar::SettingsbarState,
    menubar_state: MenubarState,
    taskbar_state: taskbar::TaskbarState,
    modebar_state: ModebarState,
    toolbar_state: ToolBarState,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            tools: tools::Tools::default(),
            toasts: Toasts::new()
                .anchor(Align2::CENTER_TOP, (0.0, 10.0))
                .direction(egui::Direction::TopDown),
            addons_state: addons::AddonsState::new(),
            quick_settings_state: quick_settingsbar::SettingsbarState::new(),
            menubar_state: MenubarState::new(),
            taskbar_state: taskbar::TaskbarState::new(),
            modebar_state: ModebarState::new(),
            toolbar_state: ToolBarState::new(),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, shared_state: &(UiState, GlobalState<RootEvent>)) {
        let frame = egui::containers::Frame {
            fill: egui::Color32::TRANSPARENT,
            outer_margin: Margin::symmetric(10.0, 10.0),
            ..Default::default()
        };

        menubar::Menubar::with_state(&mut self.menubar_state)
            .with_component_states(&mut [
                &mut self.addons_state,
                &mut self.quick_settings_state,
                &mut self.taskbar_state,
                &mut self.modebar_state,
                &mut self.toolbar_state,
            ])
            .show(ctx, shared_state);

        taskbar::Taskbar::with_state(&mut self.taskbar_state).show(ctx, shared_state);

        quick_settingsbar::Settingsbar::with_state(&mut self.quick_settings_state)
            .show(ctx, shared_state);

        toolbar::Toolbar::with_state(&mut self.toolbar_state)
            .with_tools(&mut [
                &mut self.tools.gcode_tool,
                &mut self.tools.camera_tool,
                #[cfg(debug_assertions)]
                &mut self.tools.profile_tool,
            ])
            .show(ctx, shared_state);

        modebar::Modebar::with_state(&mut self.modebar_state).show(ctx, shared_state);

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            addons::Addons::with_state(&mut self.addons_state).show(ui, shared_state);
        });

        self.tools.show(ctx, shared_state);
        self.toasts.show(ctx);
    }

    pub fn add_toast(&mut self, toast: egui_toast::Toast) {
        self.toasts.add(toast);
    }

    pub fn construct_viewport(&self, wgpu_context: &WgpuContext) -> (f32, f32, f32, f32) {
        let height = wgpu_context.surface_config.height as f32
            - self.taskbar_state.get_boundary().get_height()
            - self.modebar_state.get_boundary().get_height()
            - self.menubar_state.get_boundary().get_height();

        (
            self.toolbar_state.get_boundary().get_width(),
            self.taskbar_state.get_boundary().get_height()
                + self.modebar_state.get_boundary().get_height(),
            wgpu_context.surface_config.width as f32
                - self.toolbar_state.get_boundary().get_width()
                - self.quick_settings_state.get_boundary().get_width(),
            height,
        )
    }
}
