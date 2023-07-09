use bevy::a11y::accesskit::Orientation;
use bevy::prelude::*;

use bevy::window::{WindowMode, PrimaryWindow};
use bevy_egui::EguiContexts;
use bevy_egui::egui::Visuals;
use strum_macros::{EnumIter};

use type_eq::TypeEq;
use type_eq_derive::TypeEq;

use crate::gui::screen::Screen;
use crate::gui::{self, Component};


pub struct AsyncPacket {
    sync_element: Option<Item>,
    async_element: Option<Item>
}

impl AsyncPacket {
    pub fn new(sync_element: Item) -> Self {
        Self {
            sync_element: Some(sync_element),
            async_element: None
        }
    }

    pub fn get_sync(&self) -> &Option<Item> {
        &self.sync_element
    }

    pub fn _get_sync_mut(&mut self) -> &mut Option<Item> {
        &mut self.sync_element
    }

    pub fn _get_async(&self) -> &Option<Item> {
        &self.async_element
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Mode {
    Preview,
    Prepare,
    ForceAnalytics,
    Monitor
}

#[derive(PartialEq, Clone, Copy, Debug, EnumIter, TypeEq)]
pub enum Item {
    ToolbarWidth(Option<f32>),
    SettingsWidth(Option<f32>),
    LayerValue(Option<u32>),
    TimeValue(Option<f32>),
    Mode(Option<Mode>),
    Orientation(Option<Orientation>)
}

#[derive(Resource)]
pub struct AsyncWrapper {
    data: Vec<AsyncPacket>
}

impl AsyncWrapper {

    pub fn new(map: Vec<AsyncPacket>) -> Self {
        Self {
            data: map
        }
    }

    pub fn get_data(&mut self) -> &mut Vec<AsyncPacket> {
        &mut self.data
    }

    pub fn find_packet_mut(&mut self, item: Item) -> Option<&mut AsyncPacket> {
        self.data.iter_mut().find(|packet| packet.get_sync().unwrap().type_eq(item))
    }

    pub fn find_packet(&self, item: Item) -> Option<&AsyncPacket> {
        self.data.iter().find(|packet| packet.get_sync().unwrap().type_eq(item))
    }

    pub fn register(
        &mut self,
        item: Item
    ) {
        let packet = self.find_packet_mut(item).unwrap();
        packet.sync_element = Some(item);
    }

    pub fn _register_with_ref<V>(
        &mut self,
        default: Item,
        register_ref: fn(&mut Item, V),
        ref_ctx: V
    ) {
        let packet = self.find_packet_mut(default).unwrap();

        if packet.get_sync().is_none() {
            packet.sync_element = Some(default);
        }

        if let Some(item) = packet._get_sync_mut() {
            register_ref(item, ref_ctx);
        }   
    }

}


#[derive(Resource)]
pub struct FiberSlice {
    screen: Screen,
}

impl FiberSlice {

    pub fn new() -> Self {
        
        Self {
            screen: gui::screen::Screen::new()
        }
    }

    pub fn ui_frame(&mut self, 
        ctx: &bevy_egui::egui::Context, 
        gui_interface: &mut ResMut<gui::Interface>,
        item_wrapper: &mut ResMut<AsyncWrapper>,    
        events: &mut EventWriter<Item>
    ) {

        match gui_interface.theme() {
            gui::Theme::Light => ctx.set_visuals(Visuals::light()),
            gui::Theme::Dark => ctx.set_visuals(Visuals::dark()),
        };

        self.screen.show(ctx, None, None, gui_interface, item_wrapper);

        for packet in item_wrapper.get_data().iter_mut() {

            if packet.sync_element.is_some() {
                let event = packet.sync_element.unwrap();

                if packet.async_element.is_some() && packet.async_element.unwrap() != packet.sync_element.unwrap() {
                    println!("Item Event -> {:?}", event);
    
                    events.send(event);
                }
            }

            packet.async_element = packet.sync_element;
        }

    }

}

pub fn ui_frame(
    mut contexts: EguiContexts, 
    mut fiberslice: ResMut<FiberSlice>, 
    mut gui_interface: ResMut<gui::Interface>,
    mut item_wrapper: ResMut<AsyncWrapper>,
    mut events_resize: EventWriter<Item>
) {

    let ctx = contexts.ctx_mut();
    
    fiberslice.ui_frame(ctx, &mut gui_interface, &mut item_wrapper, &mut events_resize);
}

pub fn maximize_window(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = windows.single_mut();
    window.set_maximized(true);
}

pub fn hotkeys_window(mut windows: Query<&mut Window, With<PrimaryWindow>>, keyboard_input: Res<Input<KeyCode>>) {

    let mut window = windows.single_mut();

    if keyboard_input.pressed(KeyCode::F11) {
        if window.mode == WindowMode::Fullscreen {
            window.mode = WindowMode::Windowed;
        } else {
            window.mode = WindowMode::Fullscreen;
        }
    }

}