use std::collections::HashMap;

use bevy::prelude::*;

use bevy::window::{WindowMode, PrimaryWindow};
use bevy_egui::EguiContexts;
use bevy_egui::egui::Visuals;
use strum_macros::EnumIter;

use crate::gui::screen::Screen;
use crate::gui::{self, Component};
use crate::utils::Creation;

pub struct AsyncPacket<E> {
    sync_element: Option<E>,
    async_element: Option<E>
}

impl <E> AsyncPacket<E> {
    pub fn new() -> Self {
        Self {
            sync_element: None,
            async_element: None
        }
    }

    pub fn get_sync(&self) -> &Option<E> {
        &self.sync_element
    }

    pub fn get_sync_mut(&mut self) -> &mut Option<E> {
        &mut self.sync_element
    }

    pub fn _get_async(&self) -> &Option<E> {
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

#[derive(Hash, PartialEq, Eq, Debug, EnumIter)]
pub enum ItemType {
    ToolbarWidth,
    SettingsWidth,
    LayerValue,
    TimeValue,
    ModeChanged
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Item {
    ToolbarWidth(f32),
    SettingsWidth(f32),
    LayerValue(u32),
    TimeValue(f32),
    ModeChanged(Mode)
}

#[derive(Resource)]
pub struct AsyncWrapper<T, K> {
    pub packet_map: HashMap<T, AsyncPacket<K>>
}

impl <T, K> AsyncWrapper<T, K> {

    pub fn new(map: HashMap<T, AsyncPacket<K>>) -> Self {
        Self {
            packet_map: map
        }
    }

    pub fn register(
        event_type: ItemType, 
        event: Item, 
        gui_events: &mut HashMap<ItemType, AsyncPacket<Item>>
    ) {
        let event_wrapper = gui_events.get_mut(&event_type).unwrap();
        event_wrapper.sync_element = Some(event);
    }

    pub fn _register_with_ref<V>(
        default: Item,
        event_type: ItemType,
        register_ref: fn(&mut Item, V),
        ref_ctx: V,
        gui_events: &mut HashMap<ItemType, AsyncPacket<Item>>
    ) {
        let packet = gui_events.get_mut(&event_type).unwrap();

        if packet.get_sync().is_none() {
            packet.sync_element = Some(default);
        }

        if let Some(item) = packet.get_sync_mut() {
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
            screen: Screen::create()
        }
    }

    pub fn ui_frame(&mut self, 
        ctx: &bevy_egui::egui::Context, 
        gui_interface: &mut ResMut<gui::Interface>,
        item_wrapper: &mut ResMut<AsyncWrapper<ItemType, Item>>,    
        events: &mut EventWriter<Item>
    ) {

        match gui_interface.theme() {
            gui::Theme::Light => ctx.set_visuals(Visuals::light()),
            gui::Theme::Dark => ctx.set_visuals(Visuals::dark()),
        };

        self.screen.show(ctx, None, None, gui_interface, &mut item_wrapper.packet_map);

        for entry in item_wrapper.packet_map.iter_mut() {
            let packet = entry.1;

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
    mut item_wrapper: ResMut<AsyncWrapper<ItemType, Item>>,
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