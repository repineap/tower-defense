use bevy::prelude::*;
use bevy::{
    app::{Plugin, Startup, Update},
    asset::{AssetServer, Assets},
    ecs::system::{Commands, Res},
};
use bevy_common_assets::ron::RonAssetPlugin;

use crate::sim::defs::{MapDef, MapLoadingHandle};
use crate::sim::grid::Grid;
use crate::states::AppState::{self, MapLoading};

pub struct SimPlugin;

mod defs;
mod grid;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(RonAssetPlugin::<MapDef>::new(&["ron"]));
        app.add_systems(OnEnter(AppState::MapLoading), load_ron_map);
        app.add_systems(
            Update,
            generate_grid_from_loaded_ron_map.run_if(in_state(AppState::MapLoading)),
        );
        app.add_systems(
            Update,
            print_level_when_ready.run_if(in_state(AppState::MapReady)),
        );
    }
}

fn load_ron_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("data/map.ron");

    commands.insert_resource(MapLoadingHandle(handle));
}

fn generate_grid_from_loaded_ron_map(
    mut commands: Commands,
    level_handle: Option<Res<MapLoadingHandle>>,
    levels: Res<Assets<MapDef>>,
    app_state: Res<State<AppState>>,
    mut next_map_state: ResMut<NextState<AppState>>,
) {
    if *app_state.as_ref() == AppState::MapLoading {
        if let Some(handle_res) = level_handle {
            if let Some(level_data) = levels.get(&handle_res.0) {
                commands.insert_resource(Grid::new(
                    level_data.width,
                    level_data.height,
                    &level_data.rows,
                ));
                next_map_state.set(AppState::MapReady)
            }
        }
    }
}

fn print_level_when_ready(
    mut commands: Commands,
    grid: Res<Grid>,
    mut next_map_state: ResMut<NextState<AppState>>,
) {
    println!("{:?}", grid.as_ref());
    next_map_state.set(AppState::MapPrinted)
}
