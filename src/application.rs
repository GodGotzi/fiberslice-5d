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
    pub frame_input_generator: FrameInputGenerator,
    task_handler: TaskHandler,
    visualizer: VisualizerContext,
    pub(super) context: ApplicationContext,
    frame: Option<FrameInput>,
}

impl Application {
    pub fn new(window: &Window) -> Self {
        Self {
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

    pub fn boundaries(&self) -> &BoundaryHolder {
        self.context.boundaries()
    }

    pub fn task_handler(&mut self) -> &mut TaskHandler {
        &mut self.task_handler
    }

    pub fn visualizer(&self) -> &VisualizerContext {
        &self.visualizer
    }

    pub fn context(&self) -> &ApplicationContext {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut ApplicationContext {
        &mut self.context
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

pub fn ui_frame(ctx: &egui::Context, screen: &mut Screen, mut gui_context: GuiContext) {
    match gui_context.application.context.theme() {
        Theme::Light => ctx.set_visuals(Visuals::dark()),
        Theme::Dark => ctx.set_visuals(Visuals::dark()),
    };

    let mut visuals = ctx.style().visuals.clone();
    visuals.selection.bg_fill = egui::Color32::from_rgb(58, 84, 1);
    ctx.set_visuals(visuals);

    screen.show(ctx, &mut gui_context);
    gui_context
        .application
        .context
        .event_wrapping()
        .next_frame();
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

pub struct AsyncAction<T> {
    action: Box<dyn FnOnce(T)>,
}

impl<T> AsyncAction<T> {
    pub fn new(action: Box<dyn FnOnce(T)>) -> Self {
        Self { action }
    }

    pub fn run(self, value: T) {
        (self.action)(value);
    }
}

pub struct AsyncManipulator<T> {
    buffer: Vec<AsyncAction<T>>,
}

impl<T: Sized + Clone> AsyncManipulator<T> {
    pub fn new(buffer: Vec<AsyncAction<T>>) -> Self {
        Self { buffer }
    }

    pub fn add_action(&mut self, action: AsyncAction<T>) {
        self.buffer.push(action);
    }

    pub fn next_frame(&mut self, val: T) {
        let mut buffer = Vec::new();

        buffer.append(&mut self.buffer);

        for action in buffer {
            action.run(val.clone());
        }
    }
}

unsafe impl<T> Sync for AsyncManipulator<T> {}
unsafe impl<T> Send for AsyncManipulator<T> {}

pub struct ApplicationContext {
    theme: Theme,
    mode: Mode,
    wrapper: AsyncWrapper,
    boundaries: BoundaryHolder,
    //object_buffer_manipulator: AsyncManipulator<ObjectBuffer<dyn Object>>,
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
            //object_buffer_manipulator: AsyncManipulator::new(Vec::new()),
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
