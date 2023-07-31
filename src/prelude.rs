use strum_macros::EnumIter;

use three_d::egui::Visuals;
use type_eq::TypeEq;
use type_eq_derive::TypeEq;

use crate::gui::screen::Screen;
use crate::gui::{self, Component};

pub struct AsyncPacket {
    sync_element: Option<Item>,
    async_element: Option<Item>,
}

impl AsyncPacket {
    pub fn new(sync_element: Item) -> Self {
        Self {
            sync_element: Some(sync_element),
            async_element: None,
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
    Monitor,
}

#[derive(PartialEq, Clone, Copy, Debug, EnumIter, TypeEq)]
pub enum Item {
    ToolbarWidth(Option<f32>),
    SettingsWidth(Option<f32>),
    LayerValue(Option<u32>),
    TimeValue(Option<f32>),
    Mode(Option<Mode>),
}

pub struct AsyncWrapper {
    data: Vec<AsyncPacket>,
}

impl AsyncWrapper {
    pub fn new(map: Vec<AsyncPacket>) -> Self {
        Self { data: map }
    }

    pub fn get_data(&mut self) -> &mut Vec<AsyncPacket> {
        &mut self.data
    }

    pub fn find_packet_mut(&mut self, item: Item) -> Option<&mut AsyncPacket> {
        self.data
            .iter_mut()
            .find(|packet| packet.get_sync().unwrap().type_eq(item))
    }

    pub fn find_packet(&self, item: Item) -> Option<&AsyncPacket> {
        self.data
            .iter()
            .find(|packet| packet.get_sync().unwrap().type_eq(item))
    }

    pub fn register(&mut self, item: Item) {
        let packet = self.find_packet_mut(item).unwrap();
        packet.sync_element = Some(item);
    }

    pub fn _register_with_ref<V>(
        &mut self,
        default: Item,
        register_ref: fn(&mut Item, V),
        ref_ctx: V,
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

pub struct FiberSlice {
    screen: Screen,
}

impl FiberSlice {
    pub fn new() -> Self {
        Self {
            screen: gui::screen::Screen::new(),
        }
    }

    pub fn ui_frame(
        &mut self,
        ctx: &three_d::egui::Context,
        gui_interface: &mut gui::Interface,
        item_wrapper: &mut AsyncWrapper,
    ) {
        match gui_interface.theme() {
            gui::Theme::Light => ctx.set_visuals(Visuals::light()),
            gui::Theme::Dark => ctx.set_visuals(Visuals::dark()),
        };

        self.screen
            .show(ctx, None, None, gui_interface, item_wrapper);

        for packet in item_wrapper.get_data().iter_mut() {
            if packet.sync_element.is_some() {
                let event = packet.sync_element.unwrap();

                if packet.async_element.is_some()
                    && packet.async_element.unwrap() != packet.sync_element.unwrap()
                {
                    println!("Item Event -> {:?}", event);

                    //events.send(event);
                }
            }

            packet.async_element = packet.sync_element;
        }
    }
}

pub fn ui_frame(
    ctx: &three_d::egui::Context,
    fiberslice: &mut FiberSlice,
    gui_interface: &mut gui::Interface,
    item_wrapper: &mut AsyncWrapper,
) {
    fiberslice.ui_frame(ctx, gui_interface, item_wrapper);
}
