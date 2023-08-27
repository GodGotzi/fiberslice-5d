use std::sync::{Arc, Mutex};

use strum::IntoEnumIterator;
use three_d::{
    egui::{self, Visuals},
    FrameInput, FrameInputGenerator, WindowedContext,
};
use winit::{event::WindowEvent, window::Window};

use crate::{
    gui::*,
    prelude::{AsyncPacket, AsyncWrapper, Item},
    utils::task::Task,
    view::{visualization::VisualizerContext, Mode},
};

#[allow(dead_code)]
pub struct Application {
    screen: Screen,
    pub frame_input_generator: FrameInputGenerator,
    task_handler: TaskHandler,
    visualizer: VisualizerContext,
    context: ApplicationContext,
    frame: Option<FrameInput>,
}

impl Application {
    pub fn new(window: &Window) -> Self {
        Self {
            screen: Screen::new(),
            frame_input_generator: FrameInputGenerator::from_winit_window(window),
            task_handler: TaskHandler::default(),
            visualizer: VisualizerContext::default(),
            context: ApplicationContext::new(),
            frame: None,
        }
    }

    pub fn next_frame_input(&mut self, context: &WindowedContext) -> FrameInput {
        self.context.frame = Some(self.frame_input_generator.generate(context));
        self.context.frame.clone().unwrap()
    }

    pub fn handle_window_event(
        &mut self,
        event: &WindowEvent<'_>,
        context: &WindowedContext,
        control_flow: &mut winit::event_loop::ControlFlow,
    ) {
        self.frame_input_generator.handle_winit_window_event(event);

        match event {
            winit::event::WindowEvent::Resized(physical_size) => {
                context.resize(*physical_size);
            }
            winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                context.resize(**new_inner_size);
            }
            winit::event::WindowEvent::CloseRequested => {
                control_flow.set_exit();
            }
            _ => (),
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

    pub fn context(&self) -> &ApplicationContext {
        &self.context
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

#[derive(Default)]
pub struct TaskHandler {
    tasks: Vec<Arc<Mutex<dyn Task>>>,
}

impl TaskHandler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_task(&mut self, task: Arc<Mutex<dyn Task>>) {
        self.tasks.push(task);
    }
}

pub struct ApplicationContext {
    theme: Theme,
    mode: Mode,
    wrapper: AsyncWrapper,
    boundaries: BoundaryHolder,
    frame: Option<FrameInput>,
}

impl Default for ApplicationContext {
    fn default() -> Self {
        let mut list: Vec<AsyncPacket> = Vec::new();

        for item in Item::iter() {
            list.push(AsyncPacket::new(item));
        }

        Self {
            theme: Theme::Dark,
            mode: Mode::Prepare,
            wrapper: AsyncWrapper::new(list),
            boundaries: BoundaryHolder::default(),
            frame: None,
        }
    }
}

impl ApplicationContext {
    pub fn new() -> Self {
        Self::default()
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

    pub fn is_mode(&self, mode: Mode) -> bool {
        self.mode == mode
    }

    pub fn fps(&self) -> f64 {
        let frame_input = self.frame.as_ref().unwrap();

        1000.0 / frame_input.elapsed_time
    }
}
