use macroquad::prelude::{collections::storage, RenderTarget};

use crate::{
    editor::{
        actions::{UiAction, UiActionExt},
        state::EditorTool,
        util::{EguiCompatibleVec, EguiTextureHandler, Resizable},
        view::LevelView,
    },
    map::MapObjectKind,
    Resources,
};

use super::super::State;

impl State {
    pub(super) fn draw_level(
        &self,
        egui_ctx: &egui::Context,
        level_render_target: &mut RenderTarget,
        level_view: &LevelView,
    ) -> Option<UiAction> {
        let mut action = None;

        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(egui_ctx, |ui| {
                let texture_id = level_render_target.texture.egui_id();
                let level_img = egui::Image::new(texture_id, ui.available_size())
                    .sense(egui::Sense::click_and_drag());

                let response = ui.add(level_img);

                let resources = storage::get::<Resources>();
                for (_layer_name, layer) in self.map_resource.map.layers.iter() {
                    for object in layer.objects.iter() {
                        match object.kind {
                            MapObjectKind::Decoration => {
                                let sprite = &resources.decoration[&object.id].sprite;
                            }
                            MapObjectKind::Item => (),
                            MapObjectKind::Environment => (),
                        }
                    }
                }

                action.then_do(self.draw_level_overlays(egui_ctx, ui, &response, level_view));

                let (width, height) = (response.rect.width() as u32, response.rect.height() as u32);
                level_render_target.resize_if_appropiate(width, height);
            });

        action
    }

    fn draw_level_overlays(
        &self,
        egui_ctx: &egui::Context,
        ui: &mut egui::Ui,
        level_response: &egui::Response,
        level_view: &LevelView,
    ) -> Option<UiAction> {
        let action;

        if level_response.hovered() {
            let map = &self.map_resource.map;
            let tile_size = map.tile_size.to_egui();

            let cursor_screen_pos = ui.input().pointer.interact_pos().unwrap();
            let cursor_px_pos = screen_to_world_pos(
                cursor_screen_pos,
                level_response.rect.min.to_vec2(),
                level_view,
            );
            let cursor_tile_pos = (cursor_px_pos.to_vec2() / tile_size).floor().to_pos2();

            action = self.draw_level_placement_overlay(
                egui_ctx,
                level_response,
                cursor_tile_pos,
                level_view,
            );

            let level_top_left = level_response.rect.min;
            self.draw_level_pointer_pos_overlay(
                egui_ctx,
                ui,
                level_top_left,
                cursor_px_pos,
                cursor_tile_pos,
            );
        } else {
            action = None;
        }

        action
    }

    fn draw_level_placement_overlay(
        &self,
        egui_ctx: &egui::Context,
        level_response: &egui::Response,
        cursor_tile_pos: egui::Pos2,
        level_view: &LevelView,
    ) -> Option<UiAction> {
        let action;

        action = match self.selected_tool {
            EditorTool::TilePlacer => self.draw_level_tile_placement_overlay(
                egui_ctx,
                level_response,
                cursor_tile_pos,
                level_view,
            ),
            // TODO: Spawnpoint placement overlay
            // TODO: Object placement overlay
            _ => None,
        };

        action
    }

    fn draw_level_tile_placement_overlay(
        &self,
        egui_ctx: &egui::Context,
        level_response: &egui::Response,
        cursor_tile_pos: egui::Pos2,
        level_view: &LevelView,
    ) -> Option<UiAction> {
        let mut action = None;
        let map = &self.map_resource.map;
        let tile_size = map.tile_size.to_egui();
        let level_top_left = level_response.rect.min.to_vec2();

        if cursor_tile_pos.x >= 0. && cursor_tile_pos.y >= 0. {
            if let (Some(selected_tile), Some(selected_layer)) =
                (&self.selected_tile, &self.selected_layer)
            {
                egui::containers::Area::new("selected tile overlay")
                    .order(egui::Order::Background)
                    .interactable(false)
                    .fixed_pos(world_to_screen_pos(
                        (cursor_tile_pos.to_vec2() * tile_size).to_pos2(),
                        level_top_left,
                        level_view,
                    ))
                    .show(egui_ctx, |ui| {
                        let tileset = &map.tilesets[&selected_tile.tileset];
                        let texture_id = storage::get::<Resources>().textures[&tileset.texture_id]
                            .texture
                            .egui_id();
                        let tileset_uv_tile_size = egui::Vec2::splat(1.)
                            / egui::vec2(tileset.grid_size.x as f32, tileset.grid_size.y as f32);
                        let tileset_x = (selected_tile.tile_id % tileset.grid_size.x) as f32
                            * tileset_uv_tile_size.x;
                        let tileset_y = (selected_tile.tile_id / tileset.grid_size.x) as f32
                            * tileset_uv_tile_size.y;
                        let uv = egui::Rect::from_min_size(
                            egui::Pos2 {
                                x: tileset_x,
                                y: tileset_y,
                            },
                            tileset_uv_tile_size,
                        );
                        let (response, painter) =
                            ui.allocate_painter(tile_size, egui::Sense::hover());
                        let mut tile_mesh = egui::Mesh::with_texture(texture_id);
                        tile_mesh.add_rect_with_uv(
                            response.rect,
                            uv,
                            egui::Color32::from_rgba_unmultiplied(0xff, 0xff, 0xff, 200),
                        );

                        painter.add(egui::Shape::mesh(tile_mesh));
                    });

                if level_response.clicked() || level_response.dragged() {
                    let input = egui_ctx.input();
                    if input.pointer.button_down(egui::PointerButton::Primary) {
                        action.then_do_some(UiAction::PlaceTile {
                            id: selected_tile.tile_id,
                            layer_id: selected_layer.clone(),
                            tileset_id: selected_tile.tileset.clone(),
                            coords: macroquad::math::UVec2::new(
                                cursor_tile_pos.x as u32,
                                cursor_tile_pos.y as u32,
                            ),
                        });
                    } else if input.pointer.button_down(egui::PointerButton::Secondary) {
                        action.then_do_some(UiAction::RemoveTile {
                            layer_id: selected_layer.clone(),
                            coords: macroquad::math::UVec2::new(
                                cursor_tile_pos.x as u32,
                                cursor_tile_pos.y as u32,
                            ),
                        });
                    }
                }
            }
        }

        action
    }

    fn draw_level_pointer_pos_overlay(
        &self,
        egui_ctx: &egui::Context,
        ui: &mut egui::Ui,
        level_top_left: egui::Pos2,
        cursor_px_pos: egui::Pos2,
        cursor_tile_pos: egui::Pos2,
    ) {
        egui::containers::Area::new("pointer pos overlay")
            .order(egui::Order::Tooltip)
            .fixed_pos(
                level_top_left
                    + egui::vec2(
                        ui.spacing().window_margin.left,
                        ui.spacing().window_margin.top,
                    ),
            )
            .interactable(false)
            .drag_bounds(egui::Rect::EVERYTHING) // disable clip rect
            .show(egui_ctx, |ui| {
                egui::Frame::popup(&egui_ctx.style())
                    .show(ui, |ui| {
                        ui.label(format!(
                            "Cursor position: ({}, {}) in pixels: ({:.2}, {:.2})",
                            cursor_tile_pos.x, cursor_tile_pos.y, cursor_px_pos.x, cursor_px_pos.y,
                        ))
                    })
                    .inner
            });
    }
}

// TODO: Factor in level view scale
fn screen_to_world_pos(
    p: egui::Pos2,
    level_top_left: egui::Vec2,
    level_view: &LevelView,
) -> egui::Pos2 {
    p - level_top_left + level_view.position.to_egui()
}

fn world_to_screen_pos(
    p: egui::Pos2,
    level_top_left: egui::Vec2,
    level_view: &LevelView,
) -> egui::Pos2 {
    p + level_top_left - level_view.position.to_egui()
}
