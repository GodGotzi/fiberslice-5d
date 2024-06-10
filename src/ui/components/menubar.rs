use egui::Ui;

use crate::config;
use crate::ui::boundary::Boundary;
use crate::ui::Component;
use crate::ui::UiState;
use crate::GlobalState;
use crate::RootEvent;

pub struct Menubar {
    //enabled: bool,
    boundary: Boundary,
    enabled: bool,
}

impl Menubar {
    pub fn new() -> Self {
        Self {
            boundary: Boundary::zero(),
            enabled: true,
        }
    }
}

impl Component for Menubar {
    fn show(&mut self, ctx: &egui::Context, shared_state: &(UiState, GlobalState<RootEvent>)) {
        if self.enabled {
            self.boundary = egui::TopBottomPanel::top("menubar")
                .default_height(config::gui::MENUBAR_H)
                .show(ctx, |ui: &mut Ui| {
                    egui::menu::bar(ui, |ui| {
                        file_button(ui, shared_state);
                        edit_button(ui, shared_state);
                        window_button(ui, shared_state);
                        view_button(ui, shared_state);
                        //settings_button(ui, data);
                        help_button(ui, shared_state);
                    });
                })
                .response
                .into();
        }
    }

    fn get_enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }

    fn get_boundary(&self) -> &Boundary {
        &self.boundary
    }
}

fn file_button(ui: &mut Ui, _shared_state: &(UiState, GlobalState<RootEvent>)) {
    ui.menu_button("File", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);

        //let manipulator = gui_context.manipulator.clone();
        //let context = gui_context.context.clone();

        /*

        build_sub_menu(ui, "Load GCode", || load_gcode(data));
        build_sub_menu(ui, "Import Intersection Object", || {
            import_intersection_object(data)
        });

        ui.separator();

        build_sub_menu(ui, "Save As", || save_as_gcode(data));

        build_sub_menu(ui, "Save", || save_gcode(data));

        ui.separator();

        build_sub_menu(ui, "Exit", || exit(data));
        */
    });
}

fn edit_button(ui: &mut Ui, _shared_state: &(UiState, GlobalState<RootEvent>)) {
    ui.menu_button("Edit", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);
    });
}

fn window_button(ui: &mut Ui, _shared_state: &(UiState, GlobalState<RootEvent>)) {
    ui.menu_button("Window", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);

        /*
        ui.checkbox(
            &mut data.borrow_mut_ui_state().components.addons.enabled,
            "Addons",
        );
        ui.separator();

        ui.checkbox(
            &mut data.borrow_mut_ui_state().components.modebar.enabled,
            "ModeBar",
        );
        ui.checkbox(
            &mut data.borrow_mut_ui_state().components.toolbar.enabled,
            "ToolBar",
        );
        ui.checkbox(
            &mut data.borrow_mut_ui_state().components.taskbar.enabled,
            "TaskBar",
        );
        ui.checkbox(
            &mut data.borrow_mut_ui_state().components.settingsbar.enabled,
            "Settings",
        );
        */
    });
}

fn view_button(ui: &mut Ui, _shared_state: &(UiState, GlobalState<RootEvent>)) {
    ui.menu_button("View", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);
    });
}

fn help_button(ui: &mut Ui, _shared_state: &(UiState, GlobalState<RootEvent>)) {
    ui.menu_button("Help", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);
    });
}

fn build_sub_menu(ui: &mut Ui, title: &str, action: impl FnOnce()) {
    if ui.button(title).clicked() {
        action();
    }
}
