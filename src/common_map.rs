use crate::draw_plugin::DrawPlugin;
use egui::Ui;
use std::sync::{Arc, Mutex};
use walkers::{HttpTiles, Map, MapMemory, lat_lon};

pub type SharedTiles = Arc<Mutex<HttpTiles>>;
pub type SharedDrawPlugin = Arc<Mutex<DrawPlugin>>;

#[derive(Clone)]
pub struct CommonMap {
    tiles: SharedTiles,
    draw_plugin: SharedDrawPlugin,
    pub map_memory: MapMemory,
}
impl CommonMap {
    pub fn new(tiles: SharedTiles, draw_plugin: SharedDrawPlugin) -> Self {
        Self {
            tiles,
            draw_plugin,
            map_memory: MapMemory::default(),
        }
    }
    pub fn show(&mut self, ui: &mut Ui) {
        let mut tiles = self.tiles.lock().unwrap();
        let mut draw_plugin = self.draw_plugin.lock().unwrap();
        let map = Map::new(
            Some(&mut *tiles),
            &mut self.map_memory,
            lat_lon(46.2044, 6.1432),
        )
        .with_plugin(&mut *draw_plugin);
        ui.add(map);
    }
}
