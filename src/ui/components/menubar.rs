use egui::Ui;
use nfde::DialogResult;
use nfde::FilterableDialogBuilder;
use nfde::Nfd;
use nfde::SingleFileDialogBuilder;

use crate::config;
use crate::model::gcode;
use crate::model::gcode::DisplaySettings;
use crate::model::gcode::MeshSettings;
use crate::prelude::UnparallelSharedMut;
use crate::render;
use crate::ui::boundary::Boundary;
use crate::ui::Component;
use crate::ui::UiState;
use crate::GlobalState;
use crate::RootEvent;

pub struct Menubar {
    //enabled: bool,
    boundary: Boundary,
    enabled: UnparallelSharedMut<bool>,
}

impl Menubar {
    pub fn new() -> Self {
        Self {
            boundary: Boundary::zero(),
            enabled: UnparallelSharedMut::from_inner(true),
        }
    }
}

impl Component for Menubar {
    fn show(&mut self, ctx: &egui::Context, shared_state: &(UiState, GlobalState<RootEvent>)) {
        if *self.enabled.inner().borrow() {
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

    fn get_boundary(&self) -> &Boundary {
        &self.boundary
    }

    fn get_enabled(&self) -> UnparallelSharedMut<bool> {
        self.enabled.clone()
    }
}

fn file_button(ui: &mut Ui, (_ui_state, global_state): &(UiState, GlobalState<RootEvent>)) {
    ui.menu_button("File", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);

        let mesh_settings = MeshSettings {};
        let display_settings = DisplaySettings {
            diameter: 0.45,
            horizontal: 0.425,
            vertical: 0.325,
        };

        //let manipulator = gui_context.manipulator.clone();
        //let context = gui_context.context.clone();

        let global_state = global_state.clone();

        build_sub_menu(ui, "Load GCode", || {
            tokio::spawn(async move {
                // let path = nfd::open_file_dialog(Some("gcode"),
                let nfd = Nfd::new().unwrap();
                let result = nfd.open_file().add_filter("Gcode", "gcode").unwrap().show();

                match result {
                    DialogResult::Ok(path) => {
                        let content = std::fs::read_to_string(&path).unwrap();
                        let gcode: gcode::GCode = gcode::parser::parse_content(&content).unwrap();

                        let part = gcode::PrintPart::from_gcode(
                            (content.lines(), gcode),
                            &mesh_settings,
                            &display_settings,
                        );

                        global_state
                            .proxy
                            .send_event(RootEvent::RenderEvent(
                                render::RenderEvent::AddGCodeToolpath(part),
                            ))
                            .unwrap();
                    }
                    _ => {
                        println!("No file selected")
                    }
                }
            });
        });
        // build_sub_menu(ui, "Import Intersection Object", || {
        //    import_intersection_object(data)
        // });

        ui.separator();

        // build_sub_menu(ui, "Save As", || save_as_gcode(data));

        // build_sub_menu(ui, "Save", || save_gcode(data));

        ui.separator();

        // build_sub_menu(ui, "Exit", || exit(data));
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
