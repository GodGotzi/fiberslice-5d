/*
    Copyright (c) 2023 Elias Gottsbacher, Jan Traussnigg, Nico Huetter (HTBLA Kaindorf)
    All rights reserved.
    Note: The complete copyright description for this software thesis can be found at the beginning of each file.
    Please refer to the terms and conditions stated therein.
*/

pub mod api;
pub mod boundary;
pub mod components;
pub mod screen;
pub mod tools;

mod icon;
pub mod visual;

use std::sync::atomic::AtomicBool;

use egui_toast::ToastOptions;
use egui_wgpu_backend::ScreenDescriptor;
use egui_winit_platform::{Platform, PlatformDescriptor};
use screen::Screen;

use egui::{FontDefinitions, InnerResponse, Pos2, Rect, Ui, Visuals};
use winit::event::WindowEvent;

use crate::{
    prelude::{
        Adapter, Error, FrameHandle, Mode, Shared, SharedMut, WgpuContext, WrappedSharedMut,
    },
    GlobalState, RootEvent,
};

use self::boundary::Boundary;

#[derive(Debug, Clone)]
pub enum UiEvent {
    ShowInfo(String),
    ShowSuccess(String),
    ShowError(String),
    OpenPopup,
}

#[derive(Debug, Clone)]
pub struct UiState {
    pub pointer_in_use: Shared<AtomicBool>,
    pub theme: WrappedSharedMut<Option<Theme>>,
    pub mode: WrappedSharedMut<Mode>,

    pub layer_max: WrappedSharedMut<u16>,
    pub time_stamp: WrappedSharedMut<u16>,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            pointer_in_use: Shared::new(AtomicBool::new(false)),
            theme: WrappedSharedMut::from_inner(Option::Some(Theme::Light)),
            mode: WrappedSharedMut::from_inner(Mode::Prepare),

            layer_max: WrappedSharedMut::from_inner(u16::MAX),
            time_stamp: WrappedSharedMut::from_inner(u16::MAX),
        }
    }
}

impl UiState {
    pub fn toggle_theme(&self) {
        self.theme.write_with_fn(|theme| {
            let current_theme = theme.clone().expect("Theme is not set");

            theme.replace(match current_theme {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::Light,
            });
        });
    }
}

pub fn ui_temp_mut<T>(
    ui: &mut Ui,
    temp_value: T,
    get_dest: impl Fn(&mut Ui) -> &mut T,
    add_contents: impl FnOnce(&mut Ui),
) {
    let old_value = std::mem::replace(get_dest(ui), temp_value);
    add_contents(ui);
    let _ = std::mem::replace(get_dest(ui), old_value);
}

pub struct UiUpdateOutput {
    pub paint_jobs: Vec<egui::ClippedPrimitive>,
    pub tdelta: egui::TexturesDelta,
    pub screen_descriptor: ScreenDescriptor,
}

pub struct UiAdapter {
    state: UiState,
    screen: Screen,
    platform: Platform,
}

impl UiAdapter {
    fn update(
        &mut self,
        event: &winit::event::Event<RootEvent>,
        start_time: std::time::Instant,
        wgpu_context: &WgpuContext,
    ) {
        self.platform.handle_event(event);
        self.platform
            .update_time(start_time.elapsed().as_secs_f64());

        let (x, y, width, height) = self.screen.construct_viewport(wgpu_context);

        let is_pointer_over_viewport = {
            if let Some(pos) = self.platform.context().pointer_latest_pos() {
                pos.x >= x && pos.x <= x + width && pos.y >= y && pos.y <= y + height
            } else {
                false
            }
        };

        self.state.pointer_in_use.store(
            !is_pointer_over_viewport || self.platform.context().is_using_pointer(),
            std::sync::atomic::Ordering::Relaxed,
        );
    }
}

impl<'a>
    FrameHandle<
        'a,
        RootEvent,
        (Option<UiUpdateOutput>, (f32, f32, f32, f32)),
        GlobalState<RootEvent>,
    > for UiAdapter
{
    fn handle_frame(
        &'a mut self,
        event: &winit::event::Event<RootEvent>,
        start_time: std::time::Instant,
        wgpu_context: &WgpuContext,
        global_state: GlobalState<RootEvent>,
    ) -> Result<(Option<UiUpdateOutput>, (f32, f32, f32, f32)), Error> {
        puffin::profile_function!();

        self.state
            .pointer_in_use
            .store(false, std::sync::atomic::Ordering::Relaxed);

        self.update(event, start_time, wgpu_context);

        match event {
            winit::event::Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                self.platform.begin_frame();

                self.platform.context().style_mut(|style| {
                    catppuccin_egui::set_style_theme(style, catppuccin_egui::MOCHA);
                    style.visuals.popup_shadow = egui::epaint::Shadow::NONE;
                    style.visuals.window_shadow = egui::epaint::Shadow::NONE;
                });

                self.screen.show(
                    &self.platform.context(),
                    &(self.state.clone(), global_state),
                );

                let full_output = self.platform.end_frame(Some(&wgpu_context.window));

                let viewport = self.screen.construct_viewport(wgpu_context);

                let paint_jobs = self
                    .platform
                    .context()
                    .tessellate(full_output.shapes, full_output.pixels_per_point);

                let tdelta: egui::TexturesDelta = full_output.textures_delta;

                let screen_descriptor = ScreenDescriptor {
                    physical_width: wgpu_context.surface_config.width,
                    physical_height: wgpu_context.surface_config.height,
                    scale_factor: wgpu_context.window.scale_factor() as f32,
                };

                if self.platform.context().has_requested_repaint() {
                    wgpu_context.window.request_redraw();
                }

                return Ok((
                    Some(UiUpdateOutput {
                        paint_jobs,
                        tdelta,
                        screen_descriptor,
                    }),
                    viewport,
                ));
            }
            winit::event::Event::UserEvent(RootEvent::UiEvent(event)) => match event {
                UiEvent::ShowInfo(message) => {
                    self.screen.add_toast(egui_toast::Toast {
                        kind: egui_toast::ToastKind::Info,
                        text: message.into(),
                        options: ToastOptions::default()
                            .duration_in_seconds(5.0)
                            .show_progress(true),
                    });

                    wgpu_context.window.request_redraw();
                }
                UiEvent::ShowSuccess(message) => {
                    self.screen.add_toast(egui_toast::Toast {
                        kind: egui_toast::ToastKind::Success,
                        text: message.into(),
                        options: ToastOptions::default()
                            .duration_in_seconds(5.0)
                            .show_progress(false),
                    });

                    wgpu_context.window.request_redraw();
                }
                UiEvent::ShowError(message) => {
                    self.screen.add_toast(egui_toast::Toast {
                        kind: egui_toast::ToastKind::Error,
                        text: message.into(),
                        options: ToastOptions::default()
                            .duration_in_seconds(5.0)
                            .show_progress(true),
                    });

                    wgpu_context.window.request_redraw();
                }
                UiEvent::OpenPopup => {
                    self.screen.add_toast(egui_toast::Toast {
                        kind: egui_toast::ToastKind::Info,
                        text: "Popup opened".into(),
                        options: ToastOptions::default()
                            .duration_in_seconds(5.0)
                            .show_progress(true),
                    });

                    wgpu_context.window.request_redraw();
                }
            },
            _ => {}
        }

        Ok((None, self.screen.construct_viewport(wgpu_context)))
    }
}

impl<'a>
    Adapter<
        'a,
        RootEvent,
        UiState,
        (Option<UiUpdateOutput>, (f32, f32, f32, f32)),
        GlobalState<RootEvent>,
        UiEvent,
    > for UiAdapter
{
    fn from_context(context: &WgpuContext) -> (UiState, Self) {
        let platform = Platform::new(PlatformDescriptor {
            physical_width: context.window.inner_size().width,
            physical_height: context.window.inner_size().height,
            scale_factor: context.window.scale_factor(),
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });

        egui_extras::install_image_loaders(&platform.context());

        let state = UiState::default();
        let screen = Screen::new();

        // We use the egui_wgpu_backend crate as the render backend.

        (
            state.clone(),
            Self {
                state,
                screen,
                platform,
            },
        )
    }

    fn get_adapter_description(&self) -> String {
        "UiAdapter".to_string()
    }
}

impl From<&Theme> for Visuals {
    fn from(theme: &Theme) -> Self {
        match *theme {
            Theme::Light => Visuals::light(),
            Theme::Dark => Visuals::dark(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Theme {
    Light,
    Dark,
}

pub trait ComponentState {
    fn get_boundary(&self) -> Boundary;

    fn get_enabled(&mut self) -> &mut bool;

    fn get_name(&self) -> &str {
        "Component"
    }
}

pub trait Component {
    fn show(&mut self, ctx: &egui::Context, shared_state: &(UiState, GlobalState<RootEvent>));
}

pub trait InnerComponent {
    fn show(&mut self, ui: &mut egui::Ui, shared_state: &(UiState, GlobalState<RootEvent>));
}

pub trait AllocateInnerUiRect {
    fn allocate_ui_in_rect<R>(
        &mut self,
        inner: Rect,
        add_contents: impl FnOnce(&mut Self) -> R,
    ) -> InnerResponse<R>;
}

impl AllocateInnerUiRect for egui::Ui {
    fn allocate_ui_in_rect<R>(
        &mut self,
        inner: Rect,
        add_contents: impl FnOnce(&mut Self) -> R,
    ) -> InnerResponse<R> {
        let rect = self.available_rect_before_wrap();

        self.allocate_ui_at_rect(
            Rect::from_two_pos(
                Pos2::new(inner.left() + rect.left(), inner.top() + rect.top()),
                Pos2::new(
                    inner.left() + rect.left() + inner.width(),
                    inner.top() + rect.top() + inner.height(),
                ),
            ),
            add_contents,
        )
    }
}
