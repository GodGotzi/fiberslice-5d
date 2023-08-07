use std::sync::{Arc, Mutex};

use strum::IntoEnumIterator;
use three_d::egui::{self, Visuals};

use crate::{
    gui::*,
    prelude::{AsyncPacket, AsyncWrapper, Item},
    utils::task::VirtualTask,
    view::{visualization::VisualizerContext, Mode},
};

#[allow(dead_code)]
pub struct Application {
    screen: Screen,
    task_handler: TaskHandler,
    visualizer: VisualizerContext,
    context: ApplicationContext,
}

impl Application {
    pub fn new() -> Self {
        Self {
            screen: Screen::new(),
            task_handler: TaskHandler::new(),
            visualizer: VisualizerContext::new(),
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

    pub fn task_handler(&mut self) -> &mut TaskHandler {
        &mut self.task_handler
    }

    pub fn visualizer(&mut self) -> &mut VisualizerContext {
        &mut self.visualizer
    }

    pub fn save(&mut self) {}

    pub fn kill(&mut self) {
        for task in self.task_handler.tasks.iter_mut() {
            if let Ok(mut task) = task.lock() {
                task.kill();
            }
        }

        self.task_handler.tasks.clear();
    }
}

pub struct TaskHandler {
    tasks: Vec<Arc<Mutex<dyn VirtualTask>>>,
}

impl TaskHandler {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task: Arc<Mutex<dyn VirtualTask>>) {
        self.tasks.push(task);
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
