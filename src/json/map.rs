pub mod tiled;

use std::{
    collections::HashMap,
    iter::FromIterator,
};

use macroquad::{
    color,
    prelude::*,
};

use serde::{
    Serialize,
    Deserialize,
};

use crate::{
    json,
    map::{Map, MapLayerKind, MapLayer, MapTile, MapObject, MapTileset, ObjectLayerKind, MapProperty, CollisionKind},
};

pub use tiled::TiledMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct MapDef {
    pub name: String,
    #[serde(default = "Map::default_background_color", with = "json::ColorDef")]
    pub background_color: Color,
    #[serde(with = "super::def_vec2", default)]
    pub world_offset: Vec2,
    #[serde(with = "super::def_uvec2")]
    pub grid_size: UVec2,
    #[serde(with = "super::def_vec2")]
    pub tile_size: Vec2,
    pub layers: Vec<MapLayerDef>,
    pub tilesets: Vec<MapTileset>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, MapProperty>,
}

impl Into<MapDef> for Map {
    fn into(self) -> MapDef {
        let layers = self.draw_order.iter().filter_map(|layer_id|  {
            if let Some(layer) = self.layers.get(layer_id) {
                let (tiles, objects) = match &layer.kind {
                    MapLayerKind::TileLayer => {
                        let tiles = layer.tiles.iter().map(|opt| match opt {
                            Some(tile) => {
                                let tileset = self.tilesets.get(&tile.tileset_id)
                                    .expect(&format!("Unable to find tileset with id '{}'!", tile.tileset_id));
                                tile.tile_id + tileset.first_tile_id
                            },
                            _ => 0,
                        }).collect();

                        (Some(tiles), None)
                    },
                    MapLayerKind::ObjectLayer(_) => {
                        let objects = layer.objects.clone();

                        (None, Some(objects))
                    },
                };

                let layer = MapLayerDef {
                    id: layer.id.clone(),
                    kind: layer.kind.clone(),
                    collision: layer.collision.clone(),
                    objects,
                    tiles,
                    is_visible: layer.is_visible,
                    properties: layer.properties.clone(),
                };
                Some(layer)
            } else {
                None
            }
        }).collect();

        let tilesets = self.tilesets
            .into_iter()
            .map(|(_, tileset)| tileset)
            .collect();

        MapDef {
            name: self.name,
            background_color: self.background_color,
            world_offset: self.world_offset,
            grid_size: self.grid_size,
            tile_size: self.tile_size,
            layers,
            tilesets,
            properties: self.properties,
        }
    }
}

impl From<MapDef> for Map {
    fn from(def: MapDef) -> Self {
        let tilesets = HashMap::from_iter(
            def.tilesets
                .clone()
                .into_iter()
                .map(|tileset| (tileset.id.clone(), tileset)));

        let draw_order = def.layers
            .iter()
            .map(|layer| layer.id.clone())
            .collect();

        let layers = HashMap::from_iter(
            def.layers
                .iter()
                .map(|layer| {
                    let tiles = layer.tiles
                        .clone()
                        .unwrap_or_default()
                        .into_iter()
                        .map(|tile_id| if tile_id == 0 { None } else {
                            let tile = match tilesets
                                .iter()
                                .find(|(_, tileset)| tile_id >= tileset.first_tile_id
                                    && tile_id < tileset.first_tile_id + tileset.tile_cnt) {
                                Some((_, tileset)) => {
                                    let tile_id = tile_id - tileset.first_tile_id;
                                    let mut attributes = Vec::new();
                                    if let Some(tile_attributes) = tileset.tile_attributes.get(&tile_id).cloned() {
                                        for attribute in tile_attributes {
                                            attributes.push(attribute);
                                        }
                                    }

                                    let tile = MapTile {
                                        tile_id,
                                        tileset_id: tileset.id.clone(),
                                        texture_id: tileset.texture_id.clone(),
                                        texture_coords: tileset.get_texture_coords(tile_id),
                                        attributes,
                                    };

                                    Some(tile)
                                },
                                _ => {
                                    None
                                }
                            };
                            assert!(tile.is_some(), "Unable to determine tileset from tile_id '{}'", tile_id);
                            tile
                        }).collect();

                    let objects = layer.objects.clone().unwrap_or(Vec::new());

                    let layer = MapLayer {
                        id: layer.id.clone(),
                        kind: layer.kind.clone(),
                        collision: layer.collision.clone(),
                        grid_size: def.grid_size,
                        tiles,
                        objects,
                        is_visible: layer.is_visible,
                        properties: layer.properties.clone(),
                    };

                    (layer.id.clone(), layer)
                }));

        Map {
            name: def.name,
            background_color: def.background_color,
            world_offset: def.world_offset,
            grid_size: def.grid_size,
            tile_size: def.tile_size,
            layers,
            tilesets,
            draw_order,
            properties: def.properties,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapLayerDef {
    pub id: String,
    #[serde(default, skip_serializing_if = "CollisionKind::is_none")]
    pub collision: CollisionKind,
    pub kind: MapLayerKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tiles: Option<Vec<u32>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub objects: Option<Vec<MapObject>>,
    #[serde(default)]
    pub is_visible: bool,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, MapProperty>,
}

impl Default for MapLayerDef {
    fn default() -> Self {
        MapLayerDef {
            id: "".to_string(),
            collision: CollisionKind::None,
            kind: MapLayerKind::TileLayer,
            tiles: Some(Vec::new()),
            objects: None,
            is_visible: true,
            properties: HashMap::new()
        }
    }
}
