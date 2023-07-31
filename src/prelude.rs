use strum_macros::EnumIter;

use type_eq::TypeEq;
use type_eq_derive::TypeEq;

pub use crate::error::Error;
use crate::view::Mode;

#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, Error>;

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

    #[allow(dead_code)]
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

    pub fn next_frame(&mut self) {
        for packet in self.get_data().iter_mut() {
            if packet.sync_element.is_some() {
                let event = packet.sync_element.unwrap();

                if packet.async_element.is_some()
                    && packet.async_element.unwrap() != packet.sync_element.unwrap()
                {
                    println!("Item Event -> {:?}", event);
                }
            }

            packet.async_element = packet.sync_element;
        }
    }
}
