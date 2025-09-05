use crate::common_map::CommonMap;
use crate::draw_plugin::DrawPlugin;
use crate::main_map::MainMap;
use crate::zoom_window::ZoomWindow;
use eframe::{App, Frame};
use egui::Context;
use std::sync::{Arc, Mutex};
use walkers::sources::OpenStreetMap;
use walkers::{HttpOptions, HttpTiles};

pub struct MyApp {
    main_map: MainMap,
    zoom_windows: Vec<ZoomWindow>,
}
impl MyApp {
    pub fn new(egui_ctx: Context) -> Self {
        let http_opts = HttpOptions {
            cache: Some(".cache".into()),
            ..Default::default()
        };
        let tiles = HttpTiles::with_options(OpenStreetMap, http_opts, egui_ctx);
        let tiles = Arc::new(Mutex::new(tiles));
        let draw_plugin = DrawPlugin::default();
        let draw_plugin = Arc::new(Mutex::new(draw_plugin));
        let common_map = CommonMap::new(tiles, draw_plugin);
        Self {
            main_map: MainMap::new(common_map.clone()),
            zoom_windows: vec![
                ZoomWindow::new(1, common_map.clone()),
                ZoomWindow::new(2, common_map.clone()),
            ],
        }
    }
}
impl App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.main_map.show(ctx);
        for window in &mut self.zoom_windows {
            window.show(ctx);
        }
    }
}
