use bevy::{
    app::{FixedUpdate, Plugin, PostUpdate},
    camera::{Camera, Projection},
    ecs::{
        resource::Resource,
        system::{Commands, Res, Single},
    },
    gizmos::gizmos::Gizmos,
    input::{ButtonInput, keyboard::KeyCode, mouse::MouseButton},
    transform::components::{GlobalTransform, Transform},
    window::Window,
};

use crate::sim::grid::{Grid, GridPos};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(PostUpdate, draw_cursor);
        app.add_systems(FixedUpdate, controls);
    }
}

#[derive(Resource)]
pub struct HighlightIdx(pub GridPos);

fn draw_cursor(
    mut commands: Commands,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
) {
    let (camera, camera_transform) = *camera_query;

    if let Some(cursor_position) = window.cursor_position()
        && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
    {
        let (normalized_x_pos, normalized_y_pos) =
            (((world_pos.x) / 64.).floor(), ((world_pos.y) / 64.).floor());
        commands.insert_resource(HighlightIdx(GridPos::new(
            normalized_x_pos as i32,
            normalized_y_pos as i32,
        )));
    }
}

fn controls(
    input: Res<ButtonInput<MouseButton>>,
    highlighted_tile: Option<Res<HighlightIdx>>,
    grid: Res<Grid>,
) {
    if input.just_pressed(MouseButton::Left)
        && let Some(highlighted_pos) = highlighted_tile
    {
        let HighlightIdx(highlight_pos) = highlighted_pos.as_ref();
        dbg!(grid.get(highlight_pos));
    }
}
