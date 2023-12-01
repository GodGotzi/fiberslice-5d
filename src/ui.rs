/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

pub mod components;
pub mod state;

mod icon;
mod response;
mod visual;

use std::rc::Rc;

pub use components::size_fixed;
use three_d::{egui, Context, FrameInput, GUI};

use crate::prelude::{Adapter, Error, FrameHandle};

use self::state::UiState;

pub struct UiAdapter {
    gui: GUI,
    screen: screen::Screen,
    state: Rc<UiState>,
}

impl UiAdapter {
    pub fn from_context(context: &Context) -> Self {
        Self {
            gui: GUI::new(context),
            screen: screen::Screen::new(),
            state: Rc::new(UiState::new()),
        }
    }

    pub fn borrow_gui(&self) -> &GUI {
        &self.gui
    }

    pub fn share_state(&self) -> Rc<UiState> {
        self.state.clone()
    }
}

impl FrameHandle<UiResult, ()> for UiAdapter {
    fn handle_frame(&mut self, frame_input: &FrameInput, context: ()) -> Result<UiResult, Error> {
        let mut result = UiResult::empty();

        self.gui.update(
            &mut frame_input.events.clone(),
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |ctx| {
                result.pointer_use = Some(ctx.is_using_pointer());
                self.screen.show(ctx, self.state.clone());
            },
        );

        Ok(result)
    }
}

impl Adapter<UiResult, ()> for UiAdapter {}

pub struct UiResult {
    pub pointer_use: Option<bool>,
}

impl UiResult {
    fn empty() -> Self {
        Self { pointer_use: None }
    }
}

#[derive(Clone)]
pub enum Theme {
    Light,
    Dark,
}

pub trait SuperComponent {
    fn show<'a>(&'a mut self, ctx: &egui::Context, state: Rc<UiState>);
}

pub trait Component {
    fn show(&mut self, ctx: &egui::Context, state: Rc<UiState>);
}

pub trait InnerComponent {
    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, state: Rc<UiState>);
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
        fn show(&mut self, ctx: &egui::Context, ui_ctx: Rc<UiState>) {
            let frame = egui::containers::Frame {
                fill: egui::Color32::TRANSPARENT,
                ..Default::default()
            };

            self.menubar.show(ctx, ui_ctx);

            if ui_ctx.components.taskbar.enabled {
                self.taskbar.show(ctx, ui_ctx);
            }

            //self.addons.show(ctx, None, app);
            if ui_ctx.components.settingsbar.enabled {
                self.settings.show(ctx, ui_ctx);
            }

            if ui_ctx.components.toolbar.enabled {
                self.toolbar.show(ctx, ui_ctx);
            }

            if ui_ctx.components.modebar.enabled {
                self.modebar.show(ctx, ui_ctx);
            }

            egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
                /*
                self.icontable
                    .get_orientation_icon(crate::view::Orientation::Default)
                    .show(ui);
                */

                if ui_ctx.components.addons.enabled {
                    self.addons.show(ctx, ui, ui_ctx);
                }
            });
        }
    }
}
