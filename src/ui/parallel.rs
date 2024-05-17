use std::cell::RefCell;

use egui_glow::Painter;
use three_d::{Event, Key, Modifiers, MouseButton, Viewport};

use egui::{ClippedPrimitive, TexturesDelta};

pub struct ParallelUi {
    //painter: RefCell<Painter>,
    egui_context: egui::Context,
    output: RefCell<Option<egui::FullOutput>>,
    viewport: Viewport,
    modifiers: Modifiers,
}

struct WrapKey(egui::Key);

impl From<&Key> for WrapKey {
    fn from(key: &Key) -> Self {
        use three_d::Key::*;
        WrapKey(match key {
            ArrowDown => egui::Key::ArrowDown,
            ArrowLeft => egui::Key::ArrowLeft,
            ArrowRight => egui::Key::ArrowRight,
            ArrowUp => egui::Key::ArrowUp,
            Escape => egui::Key::Escape,
            Tab => egui::Key::Tab,
            Backspace => egui::Key::Backspace,
            Enter => egui::Key::Enter,
            Space => egui::Key::Space,
            Insert => egui::Key::Insert,
            Delete => egui::Key::Delete,
            Home => egui::Key::Home,
            End => egui::Key::End,
            PageUp => egui::Key::PageUp,
            PageDown => egui::Key::PageDown,
            Num0 => egui::Key::Num0,
            Num1 => egui::Key::Num1,
            Num2 => egui::Key::Num2,
            Num3 => egui::Key::Num3,
            Num4 => egui::Key::Num4,
            Num5 => egui::Key::Num5,
            Num6 => egui::Key::Num6,
            Num7 => egui::Key::Num7,
            Num8 => egui::Key::Num8,
            Num9 => egui::Key::Num9,
            B => egui::Key::B,
            A => egui::Key::A,
            C => egui::Key::C,
            D => egui::Key::D,
            E => egui::Key::E,
            F => egui::Key::F,
            G => egui::Key::G,
            H => egui::Key::H,
            I => egui::Key::I,
            J => egui::Key::J,
            K => egui::Key::K,
            L => egui::Key::L,
            M => egui::Key::M,
            N => egui::Key::N,
            O => egui::Key::O,
            P => egui::Key::P,
            Q => egui::Key::Q,
            R => egui::Key::R,
            S => egui::Key::S,
            T => egui::Key::T,
            U => egui::Key::U,
            V => egui::Key::V,
            W => egui::Key::W,
            X => egui::Key::X,
            Y => egui::Key::Y,
            Z => egui::Key::Z,
        })
    }
}

struct WrapModifiers(egui::Modifiers);

impl From<&Modifiers> for WrapModifiers {
    fn from(modifiers: &Modifiers) -> Self {
        Self(egui::Modifiers {
            alt: modifiers.alt,
            ctrl: modifiers.ctrl,
            shift: modifiers.shift,
            command: modifiers.command,
            mac_cmd: cfg!(target_os = "macos") && modifiers.command,
        })
    }
}

struct WrapPointerButton(egui::PointerButton);

impl From<&MouseButton> for WrapPointerButton {
    fn from(button: &MouseButton) -> Self {
        match button {
            MouseButton::Left => WrapPointerButton(egui::PointerButton::Primary),
            MouseButton::Right => WrapPointerButton(egui::PointerButton::Secondary),
            MouseButton::Middle => WrapPointerButton(egui::PointerButton::Middle),
        }
    }
}

impl ParallelUi {
    pub fn new() -> Self {
        Self {
            egui_context: egui::Context::default(),
            //painter: RefCell::new(Painter::new(context, "", None).unwrap()),
            output: RefCell::new(None),
            viewport: Viewport::new_at_origo(1, 1),
            modifiers: Modifiers::default(),
        }
    }

    pub fn update(
        &mut self,
        events: &mut [Event],
        accumulated_time_in_ms: f64,
        viewport: Viewport,
        device_pixel_ratio: f32,
        callback: impl FnOnce(&egui::Context),
    ) -> bool {
        self.egui_context
            .set_pixels_per_point(device_pixel_ratio as f32);
        self.viewport = viewport;
        let egui_input = egui::RawInput {
            screen_rect: Some(egui::Rect {
                min: egui::Pos2 {
                    x: viewport.x as f32 / device_pixel_ratio as f32,
                    y: viewport.y as f32 / device_pixel_ratio as f32,
                },
                max: egui::Pos2 {
                    x: viewport.x as f32 / device_pixel_ratio as f32
                        + viewport.width as f32 / device_pixel_ratio as f32,
                    y: viewport.y as f32 / device_pixel_ratio as f32
                        + viewport.height as f32 / device_pixel_ratio as f32,
                },
            }),
            time: Some(accumulated_time_in_ms * 0.001),
            modifiers: WrapModifiers::from(&self.modifiers).0,
            events: events
                .iter()
                .filter_map(|event| match event {
                    Event::KeyPress {
                        kind,
                        modifiers,
                        handled,
                    } => {
                        if !handled {
                            Some(egui::Event::Key {
                                key: WrapKey::from(kind).0,
                                pressed: true,
                                modifiers: WrapModifiers::from(modifiers).0,
                                repeat: false,
                                physical_key: None,
                            })
                        } else {
                            None
                        }
                    }
                    Event::KeyRelease {
                        kind,
                        modifiers,
                        handled,
                    } => {
                        if !handled {
                            Some(egui::Event::Key {
                                key: WrapKey::from(kind).0,
                                pressed: false,
                                modifiers: WrapModifiers::from(modifiers).0,
                                repeat: false,
                                physical_key: None,
                            })
                        } else {
                            None
                        }
                    }
                    Event::MousePress {
                        button,
                        position,
                        modifiers,
                        handled,
                    } => {
                        if !handled {
                            Some(egui::Event::PointerButton {
                                pos: egui::Pos2 {
                                    x: position.x / device_pixel_ratio as f32,
                                    y: (viewport.height as f32 - position.y)
                                        / device_pixel_ratio as f32,
                                },
                                button: WrapPointerButton::from(button).0,
                                pressed: true,
                                modifiers: WrapModifiers::from(modifiers).0,
                            })
                        } else {
                            None
                        }
                    }
                    Event::MouseRelease {
                        button,
                        position,
                        modifiers,
                        handled,
                    } => {
                        if !handled {
                            Some(egui::Event::PointerButton {
                                pos: egui::Pos2 {
                                    x: position.x / device_pixel_ratio as f32,
                                    y: (viewport.height as f32 - position.y)
                                        / device_pixel_ratio as f32,
                                },
                                button: WrapPointerButton::from(button).0,
                                pressed: false,
                                modifiers: WrapModifiers::from(modifiers).0,
                            })
                        } else {
                            None
                        }
                    }
                    Event::MouseMotion {
                        position, handled, ..
                    } => {
                        if !handled {
                            Some(egui::Event::PointerMoved(egui::Pos2 {
                                x: position.x / device_pixel_ratio as f32,
                                y: (viewport.height as f32 - position.y)
                                    / device_pixel_ratio as f32,
                            }))
                        } else {
                            None
                        }
                    }
                    Event::Text(text) => Some(egui::Event::Text(text.clone())),
                    Event::MouseLeave => Some(egui::Event::PointerGone),
                    Event::MouseWheel {
                        delta,
                        handled,
                        modifiers,
                        ..
                    } => {
                        if !handled {
                            Some(match modifiers.ctrl {
                                true => egui::Event::Zoom((delta.1 as f32 / 200.0).exp()),
                                false => egui::Event::Scroll(match modifiers.shift {
                                    true => egui::Vec2::new(delta.1 as f32, delta.0 as f32),
                                    false => egui::Vec2::new(delta.0 as f32, delta.1 as f32),
                                }),
                            })
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
                .collect::<Vec<_>>(),
            ..Default::default()
        };

        self.egui_context.begin_frame(egui_input);
        callback(&self.egui_context);
        *self.output.borrow_mut() = Some(self.egui_context.end_frame());

        for event in events.iter_mut() {
            if let Event::ModifiersChange { modifiers } = event {
                self.modifiers = *modifiers;
            }
            if self.egui_context.wants_pointer_input() {
                match event {
                    Event::MousePress {
                        ref mut handled, ..
                    } => {
                        *handled = true;
                    }
                    Event::MouseRelease {
                        ref mut handled, ..
                    } => {
                        *handled = true;
                    }
                    Event::MouseWheel {
                        ref mut handled, ..
                    } => {
                        *handled = true;
                    }
                    Event::MouseMotion {
                        ref mut handled, ..
                    } => {
                        *handled = true;
                    }
                    _ => {}
                }
            }

            if self.egui_context.wants_keyboard_input() {
                match event {
                    Event::KeyRelease {
                        ref mut handled, ..
                    } => {
                        *handled = true;
                    }
                    Event::KeyPress {
                        ref mut handled, ..
                    } => {
                        *handled = true;
                    }
                    _ => {}
                }
            }
        }
        self.egui_context.wants_pointer_input() || self.egui_context.wants_keyboard_input()
    }

    ///
    /// Render the GUI defined in the [update](Self::update) function.
    /// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
    ///
    pub fn construct_output(&self, camera_viewport: Viewport) -> ParallelUiOutput {
        let output = self
            .output
            .borrow_mut()
            .take()
            .expect("need to call GUI::update before GUI::render");
        let scale = self.egui_context.pixels_per_point();
        let clipped_meshes = self.egui_context.tessellate(output.shapes, scale);

        ParallelUiOutput {
            clipped_meshes,
            scale,
            viewport: self.viewport,
            textures_delta: output.textures_delta,
            pointer_use: self.egui_context.is_using_pointer(),
            camera_viewport,
        }
    }
}

#[derive(Clone)]
pub struct ParallelUiOutput {
    clipped_meshes: Vec<ClippedPrimitive>,
    scale: f32,
    viewport: Viewport,
    textures_delta: TexturesDelta,

    pub pointer_use: bool,
    pub camera_viewport: Viewport,
}

impl ParallelUiOutput {
    pub fn render(&self, painter: &RefCell<Painter>) {
        painter.borrow_mut().paint_and_update_textures(
            [self.viewport.width, self.viewport.height],
            self.scale,
            &self.clipped_meshes,
            &self.textures_delta,
        );

        #[cfg(not(target_arch = "wasm32"))]
        #[allow(unsafe_code)]
        unsafe {
            use glow::HasContext as _;
            painter.borrow().gl().disable(glow::FRAMEBUFFER_SRGB);
        }
    }
}
