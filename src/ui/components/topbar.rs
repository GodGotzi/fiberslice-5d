use egui::{Button, Layout, RichText, TopBottomPanel};
use egui::{Context, SidePanel};

use crate::config;
use crate::ui::boundary::Boundary;
use crate::ui::tools::ToolState;
use crate::ui::Component;
use crate::ui::ComponentState;
use crate::ui::UiState;
use crate::GlobalState;
use crate::RootEvent;

#[derive(Debug)]
pub struct TopBarState {
    enabled: bool,
    boundary: Boundary,
}

impl TopBarState {
    pub fn new() -> Self {
        Self {
            enabled: true,
            boundary: Boundary::zero(),
        }
    }
}

impl ComponentState for TopBarState {
    fn get_boundary(&self) -> Boundary {
        self.boundary
    }

    fn get_enabled(&mut self) -> &mut bool {
        &mut self.enabled
    }

    fn get_name(&self) -> &str {
        "Topbar"
    }
}

pub struct Topbar<'a> {
    state: &'a mut TopBarState,
    tools: &'a mut [&'a mut dyn ToolState],
}

impl<'a> Topbar<'a> {
    pub fn with_state(state: &'a mut TopBarState) -> Self {
        Self {
            state,
            tools: &mut [],
        }
    }

    pub fn with_tools(mut self, tools: &'a mut [&'a mut dyn ToolState]) -> Self {
        self.tools = tools;
        self
    }
}

impl<'a> Component for Topbar<'a> {
    fn show(&mut self, ctx: &Context, _shared_state: &(UiState, GlobalState<RootEvent>)) {
        if self.state.enabled {
            self.state.boundary = TopBottomPanel::top("topbar")
                .resizable(false)
                .show(ctx, |ui| {
                    const VERSION: &str = env!("CARGO_PKG_VERSION");

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new(format!("Version: {}", VERSION)));
                    });
                })
                .response
                .into();
        }
    }
}
