mod layers;
mod objects;
mod tileset;

use crate::map::MapLayerKind;

use super::super::Editor;

impl Editor {
    pub(super) fn draw_side_panel(&mut self, egui_ctx: &egui::Context) {
        egui::SidePanel::new(egui::containers::panel::Side::Right, "Side panel").show(
            egui_ctx,
            |ui| {
                self.draw_layer_info(ui);

                ui.separator();

                match self.selected_layer_type() {
                    Some(MapLayerKind::TileLayer) => {
                        self.draw_tileset_info(ui);
                    }
                    Some(MapLayerKind::ObjectLayer) => {
                        self.draw_object_info(ui);
                    }
                    None => (),
                }
            },
        );
    }
}
