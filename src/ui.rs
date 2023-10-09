/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

pub mod components;
pub mod data;

mod icon;
mod response;
mod visual;

use bevy::prelude::{EventWriter, Plugin, Res, ResMut, Update};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
pub use components::size_fixed;
use egui::Visuals;

use crate::{
    prelude::Context,
    settings::{FilamentSettings, PrinterSettings, SliceSettings},
    view::{Mode, Orientation},
};

use visual::customize_look_and_feel;

use data::*;
use response::Responses;

pub enum Theme {
    Light,
    Dark,
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(screen::Screen::new())
            .insert_resource(Responses::new())
            .insert_resource(RawUiData::new(Theme::Dark, Mode::Prepare))
            .add_plugins(EguiPlugin)
            .add_systems(Update, ui_frame);
    }
}

type UiContext<'a, 'b> = (
    EguiContexts<'a, 'b>,
    ResMut<'a, screen::Screen>,
    ResMut<'a, RawUiData>,
    ResMut<'a, Responses>,
);

type Settings<'a> = (
    ResMut<'a, SliceSettings>,
    ResMut<'a, FilamentSettings>,
    ResMut<'a, PrinterSettings>,
);

type Writers<'a> = EventWriter<'a, Orientation>;

pub fn ui_frame(
    context: Res<'_, Context>,
    (mut ui_ctx, mut screen, mut data, buttons_responses): UiContext,
    (slice_settinsg, filament_settings, printer_settings): Settings,
    orientation_writer: Writers,
) {
    data.holder.delete_cache();

    let ctx = ui_ctx.ctx_mut();

    let settings = SettingBundle::wrap((slice_settinsg, filament_settings, printer_settings));

    let writers = EventWriterBundle::wrap(orientation_writer);

    let data = UiDataBundle::wrap((data, buttons_responses, context, settings, writers));

    match data.raw.borrow_mut().theme {
        Theme::Light => ctx.set_visuals(Visuals::light()),
        Theme::Dark => ctx.set_visuals(Visuals::dark()),
    };

    ctx.set_visuals(customize_look_and_feel(ctx.style().visuals.clone()));

    screen.show(ctx, &data);
}

pub trait SuperComponent {
    fn show<'a>(&'a mut self, ctx: &egui::Context, data: &'a UiDataBundle<'a>);
}

pub trait Component {
    fn show(&mut self, ctx: &egui::Context, data: UiData);
}

pub trait InnerComponent {
    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, data: UiData);
}

pub trait TextComponent {
    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui);
}

pub trait InnerTextComponent<P> {
    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, prefix: P, suffix: P);
}

pub mod boundary {
    use bevy_egui::egui::{self, Response};

    #[derive(Default, Clone, Copy)]
    pub struct Boundary {
        pub location: egui::Pos2,
        pub size: egui::Vec2,
    }

    impl Boundary {
        pub fn zero() -> Self {
            Self {
                location: egui::Pos2::ZERO,
                size: egui::Vec2::ZERO,
            }
        }

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
}

pub mod screen {
    use bevy::prelude::Resource;
    use bevy_egui::egui;

    use super::*;
    use components::{addons, menubar, modebar, settingsbar, taskbar, toolbar};

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

    impl SuperComponent for Screen {
        fn show<'a>(&'a mut self, ctx: &egui::Context, data: &'a UiDataBundle<'a>) {
            let frame = egui::containers::Frame {
                fill: egui::Color32::TRANSPARENT,
                ..Default::default()
            };

            self.menubar.show(ctx, data);

            if data.raw.borrow_mut().holder.taskbar.enabled {
                self.taskbar.show(ctx, data);
            }

            //self.addons.show(ctx, None, app);
            if data.raw.borrow_mut().holder.settingsbar.enabled {
                self.settings.show(ctx, data);
            }

            if data.raw.borrow_mut().holder.toolbar.enabled {
                self.toolbar.show(ctx, data);
            }

            if data.raw.borrow_mut().holder.modebar.enabled {
                self.modebar.show(ctx, data);
            }

            egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
                /*
                self.icontable
                    .get_orientation_icon(crate::view::Orientation::Default)
                    .show(ui);
                */

                if data.raw.borrow_mut().holder.addons.enabled {
                    self.addons.show(ctx, ui, data);
                }
            });
        }
    }
}
