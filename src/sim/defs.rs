use bevy::{
    asset::{Asset, Handle},
    ecs::resource::Resource,
    reflect::TypePath,
};

#[derive(serde::Deserialize, Asset, TypePath, Debug)]
pub struct MapDef {
    pub width: i32,
    pub height: i32,
    pub rows: Vec<String>,
}

#[derive(Resource)]
pub struct MapLoadingHandle(pub Handle<MapDef>);
