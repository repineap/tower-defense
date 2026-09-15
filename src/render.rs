use bevy::{
    app::{Plugin, Startup, Update},
    asset::{AssetServer, Assets},
    camera::Camera2d,
    color::palettes::css::RED,
    ecs::system::{Commands, Res, ResMut, Single},
    gizmos::gizmos::Gizmos,
    image::{TextureAtlas, TextureAtlasLayout},
    math::{Isometry2d, Rot2, UVec2, Vec2},
    sprite::Sprite,
    state::state::OnEnter,
    transform::components::Transform,
    window::Window,
};

use crate::states::AppState;
use crate::{
    input::HighlightIdx,
    sim::grid::{Grid, GridPos, TileKind},
};

pub struct RendererPlugin;

impl Plugin for RendererPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(OnEnter(AppState::MapReady), render_map);
        app.add_systems(Update, render_highlight);
    }
}

fn render_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    loaded_grid: Res<Grid>,
    window: Single<&Window>,
) {
    let tile_size = 64;
    let texture = asset_server.load("textures/Tilesheet/towerDefense_tilesheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(tile_size), 23, 13, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn((
        Camera2d,
        Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
    ));

    for (idx, square) in loaded_grid.tiles.iter().enumerate() {
        let x = idx as i32 % loaded_grid.width;
        let y = idx as i32 / loaded_grid.width;

        let sprite_index = match square {
            TileKind::Grass => 5 * 23 + 4, // 0
            TileKind::Road => 11 * 23 + 4,
            TileKind::Spawn => 5 * 23 + 3,
            TileKind::Base => 5 * 23 + 8,
            TileKind::Rock => {
                commands.spawn((
                    Sprite::from_atlas_image(
                        texture.clone(),
                        TextureAtlas {
                            layout: texture_atlas_layout.clone(),
                            index: 23 * 5 + 22,
                        },
                    ),
                    Transform::from_xyz(
                        (x * tile_size as i32) as f32 + 32.,
                        (y * tile_size as i32) as f32 + 32.,
                        1f32,
                    ),
                ));
                5 * 23 + 4
            } // 4
        };

        commands.spawn((
            Sprite::from_atlas_image(
                texture.clone(),
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: sprite_index,
                },
            ),
            Transform::from_xyz(
                (x * tile_size as i32) as f32 + 32.,
                (y * tile_size as i32) as f32 + 32.,
                0f32,
            ),
        ));
    }
}

fn render_highlight(highlighted_tile: Option<Res<HighlightIdx>>, mut gizmos: Gizmos) {
    if let Some(highlighted_pos) = highlighted_tile.as_ref()
        && let HighlightIdx(highlight_pos) = highlighted_pos.as_ref()
    {
        gizmos.rect_2d(
            Isometry2d::new(
                Vec2::new(
                    (highlight_pos.x * 64) as f32 + 32.,
                    (highlight_pos.y * 64) as f32 + 32.,
                ),
                Rot2::degrees(0.),
            ),
            Vec2::splat(64.),
            RED,
        );
    }
}
