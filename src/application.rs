use strum::IntoEnumIterator;
use three_d::egui::{self, Visuals};

use crate::{
    gui::*,
    prelude::{AsyncPacket, AsyncWrapper, Item},
    view::Mode,
};

pub struct Application {
    screen: Screen,
    context: ApplicationContext,
}

impl Application {
    pub fn new() -> Self {
        Self {
            screen: Screen::new(),
            context: ApplicationContext::new(),
        }
    }

    pub fn ui_frame(&mut self, ctx: &egui::Context) {
        match self.context.theme() {
            Theme::Light => ctx.set_visuals(Visuals::light()),
            Theme::Dark => ctx.set_visuals(Visuals::dark()),
        };

        self.screen.show(ctx, &mut self.context);
        self.context.event_wrapping().next_frame();
    }

    pub fn boundaries(&self) -> &BoundaryHolder {
        self.context.boundaries()
    }
}

pub struct ApplicationContext {
    theme: Theme,
    mode: Mode,
    wrapper: AsyncWrapper,
    boundaries: BoundaryHolder,
}

impl ApplicationContext {
    pub fn new() -> Self {
        let mut list: Vec<AsyncPacket> = Vec::new();

        for item in Item::iter() {
            list.push(AsyncPacket::new(item));
        }

        Self {
            theme: Theme::Dark,
            mode: Mode::Prepare,
            wrapper: AsyncWrapper::new(list),
            boundaries: BoundaryHolder::default(),
        }
    }

    pub fn toggle_theme(&mut self) {
        self.theme = match self.theme {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }

    pub fn boundaries(&self) -> &BoundaryHolder {
        &self.boundaries
    }

    pub fn boundaries_mut(&mut self) -> &mut BoundaryHolder {
        &mut self.boundaries
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
}
