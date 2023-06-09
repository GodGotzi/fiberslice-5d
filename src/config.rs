
//S = Size (Width and Height)
//H = height
//W = width

pub mod default {
    use bevy::prelude::Vec2;

    //pub const WINDOW_S: Vec2 = Vec2::new(0., 0.);
    pub const WINDOW_S: Vec2 = Vec2::new(1200., 900.);

}

pub mod gui {
    use bevy_egui::egui::{self};


    pub const fn shaded_color(darkmode: bool) -> egui::Color32 {
        match darkmode {
            true =>  egui::Color32::from_rgba_premultiplied(25, 25, 25, 100),
            false => egui::Color32::from_rgba_premultiplied(145, 145, 145, 50),
        }
    }

    pub const MENUBAR_H: f32 = 17.0; 
    pub const MODEBAR_H: f32 = 17.0; 
    pub const TASKBAR_H: f32 = 20.0; 
    pub const TOOLBAR_W: f32 = 35.0;

    pub mod addons {
        
    }

    pub mod default {
        pub const SETTINGSBAR_W: f32 =  350.0;
    }
}