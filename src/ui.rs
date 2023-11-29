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

pub use components::size_fixed;
use three_d::egui;

use self::data::UiData;

#[derive(Clone)]
pub enum Theme {
    Light,
    Dark,
}

pub trait SuperComponent {
    fn show<'a>(&'a mut self, ctx: &egui::Context, ui_ctx: &mut UiData);
}

pub trait Component {
    fn show(&mut self, ctx: &egui::Context, ui_ctx: &mut UiData);
}

pub trait InnerComponent {
    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, ui_ctx: &mut UiData);
}

pub trait TextComponent {
    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui);
}

pub trait InnerTextComponent<P> {
    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, prefix: P, suffix: P);
}

pub mod boundary {
    use egui::Response;
    use three_d::egui;

    #[derive(Default, Clone, Copy)]
    pub struct Boundary {
        pub location: egui::Pos2,
        size: egui::Vec2,
    }

    impl Boundary {
        pub fn new(location: egui::Pos2, size: egui::Vec2) -> Self {
            Self { location, size }
        }

        pub fn zero() -> Self {
            Self {
                location: egui::Pos2::ZERO,
                size: egui::Vec2::ZERO,
            }
        }

        pub fn get_width(&self) -> f32 {
            self.size.x
        }

        pub fn get_height(&self) -> f32 {
            self.size.y
        }

        pub fn get_size(&self) -> egui::Vec2 {
            self.size
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

            if ui_ctx.get_components().taskbar.enabled {
                self.taskbar.show(ctx, ui_ctx);
            }

            //self.addons.show(ctx, None, app);
            if ui_ctx.get_components().settingsbar.enabled {
                self.settings.show(ctx, ui_ctx);
            }

            if ui_ctx.get_components().toolbar.enabled {
                self.toolbar.show(ctx, ui_ctx);
            }

            if ui_ctx.get_components().modebar.enabled {
                self.modebar.show(ctx, ui_ctx);
            }

            egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
                /*
                self.icontable
                    .get_orientation_icon(crate::view::Orientation::Default)
                    .show(ui);
                */

                if ui_ctx.get_components().addons.enabled {
                    self.addons.show(ctx, ui, ui_ctx);
                }
            });
        }
    }
}
