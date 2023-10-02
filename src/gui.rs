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
pub mod response;
pub mod settingsbar;
pub mod taskbar;
pub mod toolbar;

use std::cell::RefCell;

use bevy::prelude::{EventWriter, Plugin, Res, ResMut, Resource, Update};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use egui::{Response, Visuals};
use strum::EnumCount;

use crate::{
    prelude::Context,
    view::{Mode, Orientation},
};

use crate::gui::response::ButtonResponse;

use self::components::addons;

pub type UiData<'a> = &'a UiDataPacket<'a>;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Screen::new())
            .insert_resource(ButtonResponses::new())
            .insert_resource(RawUiData::new(Theme::Dark, Mode::Prepare))
            .add_plugins(EguiPlugin)
            .add_systems(Update, ui_frame);
    }
}

pub struct EventWriters<'a> {
    orientation: RefCell<EventWriter<'a, Orientation>>,
}

#[derive(Resource)]
pub struct ButtonResponses {
    orientation: [ButtonResponse; Orientation::COUNT],
}

impl ButtonResponses {
    pub fn new() -> Self {
        Self {
            orientation: [
                ButtonResponse::new(),
                ButtonResponse::new(),
                ButtonResponse::new(),
                ButtonResponse::new(),
                ButtonResponse::new(),
                ButtonResponse::new(),
            ],
        }
    }
}

pub struct UiDataPacket<'a> {
    pub raw: RefCell<ResMut<'a, RawUiData>>,
    pub button_responses: RefCell<ResMut<'a, ButtonResponses>>,
    pub context: Res<'a, Context>,
    writers: EventWriters<'a>,
}

impl<'a> UiDataPacket<'a> {
    pub fn new(
        raw: ResMut<'a, RawUiData>,
        button_responses: ResMut<'a, ButtonResponses>,
        context: Res<'a, Context>,
        writers: EventWriters<'a>,
    ) -> Self {
        Self {
            raw: RefCell::new(raw),
            button_responses: RefCell::new(button_responses),
            context,
            writers,
        }
    }

    pub fn orienation_writer(&self) -> &RefCell<EventWriter<'a, Orientation>> {
        &self.writers.orientation
    }

    pub fn update_orientation_response(&self, response: &Response, orientation: Orientation) {
        self.button_responses.borrow_mut().orientation[orientation as usize].update(response);
    }

    pub fn get_orientation_response(&self, orientation: Orientation) -> ButtonResponse {
        self.button_responses.borrow().orientation[orientation as usize]
    }
}

#[derive(Resource)]
pub struct RawUiData {
    pub theme: Theme,
    pub mode: Mode,
    pub boundary_holder: BoundaryHolder,
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
    buttons_responses: ResMut<'_, ButtonResponses>,
    context: Res<'_, Context>,
    orientation_writer: EventWriter<Orientation>,
) {
    let ctx = contexts.ctx_mut();

    let writers = EventWriters {
        orientation: RefCell::new(orientation_writer),
    };

    let data = UiDataPacket::new(data, buttons_responses, context, writers);

    match data.raw.borrow_mut().theme {
        Theme::Light => ctx.set_visuals(Visuals::light()),
        Theme::Dark => ctx.set_visuals(Visuals::dark()),
    };

    let mut visuals = ctx.style().visuals.clone();
    visuals.selection.bg_fill = egui::Color32::from_rgb(76, 255, 0);
    visuals.selection.stroke.color = egui::Color32::from_rgb(0, 0, 0);

    ctx.set_visuals(visuals);

    screen.show(ctx, &data);
}

pub trait SuperComponent<T> {
    fn show<'a>(&'a mut self, ctx: &egui::Context, data: &'a UiDataPacket<'a>);
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
    menubar: Option<Boundary>,
    taskbar: Option<Boundary>,
    modebar: Option<Boundary>,
    toolbar: Option<Boundary>,
    settingsbar: Option<Boundary>,
}

impl BoundaryHolder {
    pub fn set_menubar(&mut self, boundary: Boundary) {
        self.menubar = Some(boundary);
    }

    pub fn set_taskbar(&mut self, boundary: Boundary) {
        self.taskbar = Some(boundary);
    }

    pub fn set_modebar(&mut self, boundary: Boundary) {
        self.modebar = Some(boundary);
    }

    pub fn set_toolbar(&mut self, boundary: Boundary) {
        self.toolbar = Some(boundary);
    }

    pub fn set_settingsbar(&mut self, boundary: Boundary) {
        self.settingsbar = Some(boundary);
    }

    pub fn initialized(&self) -> bool {
        self.menubar.is_some()
            && self.taskbar.is_some()
            && self.modebar.is_some()
            && self.toolbar.is_some()
            && self.settingsbar.is_some()
    }

    pub fn menubar(&self) -> &Boundary {
        self.menubar.as_ref().unwrap()
    }

    pub fn taskbar(&self) -> &Boundary {
        self.taskbar.as_ref().unwrap()
    }

    pub fn modebar(&self) -> &Boundary {
        self.modebar.as_ref().unwrap()
    }

    pub fn toolbar(&self) -> &Boundary {
        self.toolbar.as_ref().unwrap()
    }

    pub fn settingsbar(&self) -> &Boundary {
        self.settingsbar.as_ref().unwrap()
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
    fn show<'a>(&'a mut self, ctx: &egui::Context, data: &'a UiDataPacket<'a>) {
        let frame = egui::containers::Frame {
            fill: egui::Color32::TRANSPARENT,
            ..Default::default()
        };

        self.menubar.show(ctx, data);
        self.taskbar.show(ctx, data);

        //self.addons.show(ctx, None, app);

        self.settings.show(ctx, data);
        self.toolbar.show(ctx, data);
        self.modebar.show(ctx, data);

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            /*
            self.icontable
                .get_orientation_icon(crate::view::Orientation::Default)
                .show(ui);
            */

            self.addons.show(ctx, ui, data);
        });
    }
}
