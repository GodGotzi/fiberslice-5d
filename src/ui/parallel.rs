use std::cell::RefCell;

use three_d::{
    egui::{self, ClippedPrimitive, TexturesDelta},
    Event, Modifiers, Viewport, GUI,
};

pub struct ParallelUi {
    //painter: RefCell<Painter>,
    egui_context: egui::Context,
    output: RefCell<Option<egui::FullOutput>>,
    viewport: Viewport,
    modifiers: Modifiers,
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
            pixels_per_point: Some(device_pixel_ratio as f32),
            time: Some(accumulated_time_in_ms * 0.001),
            modifiers: (&self.modifiers).into(),
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
                                key: kind.into(),
                                pressed: true,
                                modifiers: modifiers.into(),
                                repeat: false,
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
                                key: kind.into(),
                                pressed: false,
                                modifiers: modifiers.into(),
                                repeat: false,
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
                                    x: position.x,
                                    y: position.y,
                                },
                                button: button.into(),
                                pressed: true,
                                modifiers: modifiers.into(),
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
                                    x: position.x,
                                    y: position.y,
                                },
                                button: button.into(),
                                pressed: false,
                                modifiers: modifiers.into(),
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
                                x: position.x,
                                y: position.y,
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
    pub fn construct_output(&self) -> ParallelUiOutput {
        let output = self
            .output
            .borrow_mut()
            .take()
            .expect("need to call GUI::update before GUI::render");
        let clipped_meshes = self.egui_context.tessellate(output.shapes);
        let scale = self.egui_context.pixels_per_point();

        ParallelUiOutput {
            clipped_meshes,
            scale,
            viewport: self.viewport,
            textures_delta: output.textures_delta,
        }
    }
}

pub struct ParallelUiOutput {
    clipped_meshes: Vec<ClippedPrimitive>,
    scale: f32,
    viewport: Viewport,
    textures_delta: TexturesDelta,
}
