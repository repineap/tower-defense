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
    image::ImagePlugin,
    utils::default,
    window::{Window, WindowPlugin, WindowResolution},
};

use crate::{input::InputPlugin, render::RendererPlugin, sim::SimPlugin, states::StatesPlugin};

pub struct HelloPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Hello World Tower Defense Game".into(),
                        resolution: WindowResolution::new(1280, 768),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(ClearColor(Color::srgb(0.5, 0.1, 0.1)))
        .add_plugins(StatesPlugin)
        .add_plugins(SimPlugin)
        .add_plugins(RendererPlugin)
        .add_plugins(InputPlugin)
        .run();
}
