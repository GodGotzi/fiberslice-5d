use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui::{Pos2, self}};

use crate::view::ViewInterface;

use super::FiberSlice;

pub trait GuiComponent<T> {
    fn show(&mut self, ctx: &egui::Context, 
        view_interface: &mut ResMut<ViewInterface>,
        gui_interface: &mut ResMut<GuiInterface>,          
        events_resize: &mut EventWriter<GuiResizeEvent>
    );
}

pub enum GuiResizeEvent {
    Side(f32)
}

pub struct Boundary {
    pub location: Vec2,
    pub size: Vec2
}

impl Boundary {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            location: Vec2::new(x, y),
            size: Vec2::new(width, height),
        }
    }
}

#[derive(Resource)]
pub struct GuiInterface {

    touch: bool,
    pub toggle_theme: bool,
    pub side_ratio: f32,
    pub side_boundary: Option<Boundary>,
    pub menubar_boundary: Option<Boundary>,   
    pub taskbar_boundary: Option<Boundary>,
    pub popup_boundaries: Option<[Boundary; 10]>,
}

impl GuiInterface {

    pub fn new() -> Self {
        Self {
            touch: false,
            toggle_theme: true,
            side_ratio: 2.0/5.0,
            side_boundary: None,
            menubar_boundary: None,
            taskbar_boundary: None,
            popup_boundaries: None,
        }
    }

    pub fn is_touch(&self) -> bool {
        self.touch
    }
}

pub fn ui_frame(
    mut contexts: EguiContexts, 
    mut fiberslice: ResMut<FiberSlice>, 
    mut view_interface: ResMut<crate::view::ViewInterface>,
    mut gui_interface: ResMut<GuiInterface>,  
    mut events_resize: EventWriter<GuiResizeEvent>
) {
    let ctx = contexts.ctx_mut();
    fiberslice.ui_frame(ctx, &mut view_interface, &mut gui_interface, &mut events_resize);
}

pub fn check_touch(
    mut gui_interface: ResMut<GuiInterface>,
    mut contexts: EguiContexts,
    mouse_buttons: Res<Input<MouseButton>>,
) {
    let ctx = contexts.ctx_mut();

    if ctx.is_using_pointer() {
        gui_interface.touch = true;
        return;
    }

    let opt_cursor = ctx.pointer_hover_pos();

    if opt_cursor.is_none() {
        return;
    }

    let cursor_pos = opt_cursor.unwrap();

    if !mouse_buttons.pressed(MouseButton::Left) && !mouse_buttons.pressed(MouseButton::Middle) {
        gui_interface.touch = false;
        return;
    }

    check_boundaries(cursor_pos, &mut gui_interface);

}

fn check_boundaries(cursor_pos: Pos2, gui_interface: &mut ResMut<GuiInterface>) {
    let cursor_vec = Vec2::new(cursor_pos.x, cursor_pos.y);
    
    if let Some(boundary) = &gui_interface.side_boundary {
        if check_boundary(boundary, 0., cursor_vec) {
            gui_interface.touch = true;
            return;
        } else {
            gui_interface.touch = false;
        }
    }

    if let Some(boundary) = &gui_interface.menubar_boundary {
        if check_boundary(boundary, 0., cursor_vec) {
            gui_interface.touch = true;
        } else {
            gui_interface.touch = false;
        }
    }
}

fn check_boundary(boundary: &Boundary, additional_broder: f32, cursor_vec: Vec2) -> bool {

    if boundary.location.x - additional_broder <= cursor_vec.x && boundary.location.x + boundary.size.x + additional_broder >= cursor_vec.x 
        && boundary.location.y - additional_broder <= cursor_vec.y && boundary.location.y + boundary.size.y + additional_broder >= cursor_vec.y
        {
        
        return true;
    }

    false
}