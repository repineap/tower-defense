use bevy::{
    app::{Plugin, Startup},
    asset::{AssetServer, Assets},
    camera::Camera2d,
    ecs::system::{Commands, Res, ResMut},
    image::{TextureAtlas, TextureAtlasLayout},
    math::UVec2,
    sprite::Sprite,
    state::state::OnEnter,
    transform::components::Transform,
};

use crate::sim::grid::{Grid, TileKind};
use crate::states::AppState;

pub struct RendererPlugin;

impl Plugin for RendererPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(OnEnter(AppState::MapReady), render_map);
    }
}

fn render_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    loaded_grid: Res<Grid>,
) {
    let tile_size = 64;
    let texture = asset_server.load("textures/Tilesheet/towerDefense_tilesheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(tile_size), 23, 13, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn(Camera2d);

    for (idx, square) in loaded_grid.tiles.iter().enumerate() {
        let x = idx as i32 % loaded_grid.width - loaded_grid.width / 2;
        let y = idx as i32 / loaded_grid.width - loaded_grid.height / 2;

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
                        (x * tile_size as i32) as f32 + 32f32,
                        (y * tile_size as i32) as f32 + 32f32,
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
                (x * tile_size as i32) as f32 + 32f32,
                (y * tile_size as i32) as f32 + 32f32,
                0f32,
            ),
        ));
    }
}
