use crate::common_map::CommonMap;
use egui::{Align2, CentralPanel, Context, RichText, Ui, Window};
use walkers::MapMemory;

pub struct MainMap {
    map: CommonMap,
}

impl MainMap {
    pub fn new(map: CommonMap) -> Self {
        Self { map }
    }

    pub fn show(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            self.map.show(ui);
            Self::zoom_buttons(ui, &mut self.map.map_memory);
        });
    }

    fn zoom_buttons(ui: &Ui, map_memory: &mut MapMemory) {
        Window::new("Zoom")
            .collapsible(false)
            .resizable(false)
            .title_bar(false)
            .anchor(Align2::LEFT_BOTTOM, [10., -10.])
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    if ui.button(RichText::new("➕").heading()).clicked() {
                        let _ = map_memory.zoom_in();
                    }

                    if ui.button(RichText::new("➖").heading()).clicked() {
                        let _ = map_memory.zoom_out();
                    }
                });
            });
    }
}
