use crate::common_map::CommonMap;
use egui::{Context, Window};

pub struct ZoomWindow {
    id: usize,
    map: CommonMap,
}
impl ZoomWindow {
    pub fn new(id: usize, map: CommonMap) -> Self {
        Self { id, map }
    }
    pub fn show(&mut self, ctx: &Context) {
        Window::new(format!("Zoomed Map #{}", self.id))
            .resizable(true)
            .show(ctx, |ui| {
                self.map.show(ui);
            });
    }
}
