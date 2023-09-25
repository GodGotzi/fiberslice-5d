/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

mod components;
mod icon;
pub mod menubar;
pub mod modebar;
pub mod settingsbar;
pub mod taskbar;
pub mod toolbar;

use bevy::prelude::{Mut, ResMut, Resource};
use bevy_egui::{egui, EguiContexts};
use egui::{Response, Visuals};

use crate::view::Mode;

use self::components::addons;

pub type UiData<'a> = Mut<'a, RawUiData>;

#[derive(Resource)]
pub struct RawUiData {
    pub(super) theme: Theme,
    pub(super) mode: Mode,
    pub(super) boundary_holder: BoundaryHolder,
}

impl RawUiData {
    pub fn new(theme: Theme, mode: Mode) -> Self {
        Self {
            theme,
            mode,
            boundary_holder: BoundaryHolder::default(),
        }
    }

    pub fn toggle_theme(&mut self) {
        self.theme = match self.theme {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        };
    }
}

pub fn ui_frame(
    mut contexts: EguiContexts,
    mut screen: ResMut<'_, Screen>,
    data: ResMut<'_, RawUiData>,
) {
    let ctx = contexts.ctx_mut();

    match data.theme {
        Theme::Light => ctx.set_visuals(Visuals::dark()),
        Theme::Dark => ctx.set_visuals(Visuals::dark()),
    };

    let mut visuals = ctx.style().visuals.clone();
    visuals.selection.bg_fill = egui::Color32::from_rgb(76, 255, 0);
    visuals.selection.stroke.color = egui::Color32::from_rgb(0, 0, 0);

    ctx.set_visuals(visuals);

    screen.show(ctx, data);
}

pub trait SuperComponent<T> {
    fn show(&mut self, ctx: &egui::Context, data: ResMut<RawUiData>);
}

pub trait Component<T> {
    fn show(&mut self, ctx: &egui::Context, data: UiData);
}

pub trait InnerComponent<T> {
    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, data: UiData);
}

#[derive(Default)]
pub struct Boundary {
    location: egui::Pos2,
    size: egui::Vec2,
}

impl Boundary {
    #[allow(dead_code)]
    pub fn offset_x(&self) -> f32 {
        self.location.x
    }

    #[allow(dead_code)]
    pub fn offset_y(&self) -> f32 {
        self.location.y
    }

    pub fn width(&self) -> f32 {
        self.size.x
    }

    pub fn height(&self) -> f32 {
        self.size.y
    }
}

impl From<Response> for Boundary {
    fn from(response: Response) -> Self {
        Self {
            location: response.rect.min,
            size: response.rect.size(),
        }
    }
}

#[derive(Default)]
pub struct BoundaryHolder {
    menubar: Boundary,
    taskbar: Boundary,
    modebar: Boundary,
    toolbar: Boundary,
    settingsbar: Boundary,
}

impl BoundaryHolder {
    pub fn set_menubar(&mut self, boundary: Boundary) {
        self.menubar = boundary;
    }

    pub fn set_taskbar(&mut self, boundary: Boundary) {
        self.taskbar = boundary;
    }

    pub fn set_modebar(&mut self, boundary: Boundary) {
        self.modebar = boundary;
    }

    pub fn set_toolbar(&mut self, boundary: Boundary) {
        self.toolbar = boundary;
    }

    pub fn set_settingsbar(&mut self, boundary: Boundary) {
        self.settingsbar = boundary;
    }

    pub fn menubar(&self) -> &Boundary {
        &self.menubar
    }

    pub fn taskbar(&self) -> &Boundary {
        &self.taskbar
    }

    pub fn modebar(&self) -> &Boundary {
        &self.modebar
    }

    pub fn toolbar(&self) -> &Boundary {
        &self.toolbar
    }

    pub fn settingsbar(&self) -> &Boundary {
        &self.settingsbar
    }
}

pub enum Theme {
    Light,
    Dark,
}

#[derive(Resource)]
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

impl SuperComponent<Screen> for Screen {
    fn show(&mut self, ctx: &egui::Context, mut data: ResMut<RawUiData>) {
        let frame = egui::containers::Frame {
            fill: egui::Color32::TRANSPARENT,
            ..Default::default()
        };

        self.menubar.show(ctx, data.reborrow());
        self.taskbar.show(ctx, data.reborrow());

        //self.addons.show(ctx, None, app);

        self.settings.show(ctx, data.reborrow());
        self.toolbar.show(ctx, data.reborrow());
        self.modebar.show(ctx, data.reborrow());

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            /*
            self.icontable
                .get_orientation_icon(crate::view::Orientation::Default)
                .show(ui);
            */

            self.addons.show(ctx, ui, data.reborrow());
        });
    }
}
