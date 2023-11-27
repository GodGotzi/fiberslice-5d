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

use std::sync::{Arc, Mutex, MutexGuard};

pub use components::size_fixed;

use crate::view::Mode;

use self::data::ComponentDataHolder;

#[derive(Clone)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Clone)]
pub struct UiContext {
    pub theme: Theme,
    pub mode: Mode,

    component_data: Arc<Mutex<ComponentDataHolder>>,
}

impl Default for UiContext {
    fn default() -> Self {
        Self {
            theme: Theme::Light,
            mode: Mode::Preview,
            component_data: Arc::new(Mutex::new(ComponentDataHolder::default())),
        }
    }
}

impl UiContext {
    pub fn get_component_data_mut(&self) -> MutexGuard<ComponentDataHolder> {
        self.component_data.lock().unwrap()
    }
}

pub trait SuperComponent {
    fn show<'a>(&'a mut self, ctx: &egui::Context, ui_ctx: UiContext);
}

pub trait Component {
    fn show(&mut self, ctx: &egui::Context, ui_ctx: UiContext);
}

pub trait InnerComponent {
    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, ui_ctx: UiContext);
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
        fn show<'a>(&'a mut self, ctx: &egui::Context, ui_ctx: UiContext) {
            let frame = egui::containers::Frame {
                fill: egui::Color32::TRANSPARENT,
                ..Default::default()
            };

            self.menubar.show(ctx, ui_ctx);

            if ui_ctx.get_component_data_mut().taskbar.enabled {
                self.taskbar.show(ctx, ui_ctx);
            }

            //self.addons.show(ctx, None, app);
            if ui_ctx.get_component_data_mut().settingsbar.enabled {
                self.settings.show(ctx, ui_ctx);
            }

            if ui_ctx.get_component_data_mut().toolbar.enabled {
                self.toolbar.show(ctx, ui_ctx);
            }

            if ui_ctx.get_component_data_mut().modebar.enabled {
                self.modebar.show(ctx, ui_ctx);
            }

            egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
                /*
                self.icontable
                    .get_orientation_icon(crate::view::Orientation::Default)
                    .show(ui);
                */

                if ui_ctx.get_component_data_mut().addons.enabled {
                    self.addons.show(ctx, ui, ui_ctx);
                }
            });
        }
    }
}
