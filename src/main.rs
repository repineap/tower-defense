#![forbid(unsafe_code)]

mod debug;
mod input;
mod render;
mod save;
mod sim;
mod states;
mod ui;

use bevy::{
    DefaultPlugins,
    app::{App, PluginGroup},
    camera::ClearColor,
    color::Color,
    utils::default,
    window::{Window, WindowPlugin, WindowResolution},
};

pub struct HelloPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Hello World Tower Defense Game".into(),
                resolution: WindowResolution::new(1280, 720),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.5, 0.1, 0.1)))
        .run();
}
