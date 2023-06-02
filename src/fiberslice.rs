use bevy::prelude::*;

use crate::view::{ViewInterface};

use self::{screen::{Screen, GuiResizeEvent}, gui::GuiInterface};

pub mod utils;
pub mod screen;
mod options;

#[derive(Resource)]
pub struct FiberSlice {
    screen: Screen,
}

impl FiberSlice {

    pub fn new() -> Self {
        Self {
            screen: Screen::new(),
        }
    }

    pub fn ui_frame(&mut self, ctx: &bevy_egui::egui::Context, 
        view_interface: &mut ResMut<ViewInterface>,
        gui_interface: &mut ResMut<GuiInterface>,       
        events_resize: &mut EventWriter<GuiResizeEvent>
    ) {
        self.screen.ui(ctx, view_interface, gui_interface, events_resize);
    }

}

pub mod gui {

    use bevy::prelude::*;
    use bevy_egui::{EguiContexts, egui::Pos2};

    use super::{screen::{Boundary, GuiResizeEvent}, FiberSlice};

    
    #[derive(Resource)]
    pub struct GuiInterface {
        touch: bool,
        pub side_boundary: Option<Boundary>,
        pub menu_boundary: Option<Boundary>   
    }

    impl GuiInterface {

        pub fn new() -> Self {
            Self {
                touch: false,
                side_boundary: None,
                menu_boundary: None
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

        if let Some(boundary) = &gui_interface.menu_boundary {
            if check_boundary(boundary, 0., cursor_vec) {
                gui_interface.touch = true;
                return;
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

        return false;
    }

}