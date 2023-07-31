use std::sync::{Arc, Mutex};

use strum::IntoEnumIterator;
use three_d::egui::{self, Visuals};

use crate::{
    gui::*,
    prelude::{AsyncPacket, AsyncWrapper, Item, Mode},
};

pub struct Application {
    screen: Arc<Mutex<Screen>>,
    theme: Theme,
    mode: Mode,
    wrapper: AsyncWrapper,
}

impl Application {
    pub fn new() -> Self {
        let mut list: Vec<AsyncPacket> = Vec::new();

        for item in Item::iter() {
            list.push(AsyncPacket::new(item));
        }

        Self {
            screen: Arc::new(Mutex::new(Screen::new())),
            theme: Theme::Light,
            mode: Mode::Preview,
            wrapper: AsyncWrapper::new(list),
        }
    }

    pub fn toggle_theme(&mut self) {
        self.theme = match self.theme {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }

    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    pub fn event_wrapping(&mut self) -> &mut AsyncWrapper {
        &mut self.wrapper
    }

    pub fn mode(&mut self) -> &mut Mode {
        &mut self.mode
    }

    pub fn ui_frame(&mut self, ctx: &egui::Context) {
        match self.theme {
            Theme::Light => ctx.set_visuals(Visuals::light()),
            Theme::Dark => ctx.set_visuals(Visuals::dark()),
        };

        self.screen.clone().lock().unwrap().show(ctx, self);
        self.wrapper.next_frame();
    }
}
