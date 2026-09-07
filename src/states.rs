use bevy::{
    app::Plugin,
    state::{app::AppExtStates, state::States},
};

pub struct StatesPlugin;

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    MapLoading,
    MapReady,
    MapPrinted,
}

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_state::<AppState>();
    }
}
