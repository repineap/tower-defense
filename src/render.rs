use bevy::{
    app::{Plugin, Update},
    asset::{AssetServer, Assets, RenderAssetUsages},
    camera::Camera2d,
    color::{
        Color,
        palettes::css::{ORANGE, RED},
    },
    ecs::system::{Commands, Res, ResMut, Single},
    gizmos::gizmos::Gizmos,
    image::{TextureAtlas, TextureAtlasLayout},
    math::{Isometry2d, Rot2, UVec2, Vec2, Vec3, primitives::Rectangle},
    mesh::{
        Mesh, Mesh2d,
        PrimitiveTopology::{LineStrip, PointList},
    },
    sprite::Sprite,
    sprite_render::{ColorMaterial, MeshMaterial2d},
    state::state::OnEnter,
    transform::components::Transform,
    utils::default,
    window::Window,
};

use crate::{
    input::HighlightIdx,
    sim::{
        grid::{Grid, GridPos, TileKind},
        path::Path,
    },
};
use crate::{sim::path::find_path, states::AppState};

pub struct RendererPlugin;

impl Plugin for RendererPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(OnEnter(AppState::MapReady), render_map);
        app.add_systems(Update, render_highlight);
    }
}

const TILE_SIZE: u32 = 64;

fn grid_pos_to_screen(grid_pos: &GridPos) -> (f32, f32) {
    (
        (grid_pos.x * TILE_SIZE as i32) as f32 + 32.,
        (grid_pos.y * TILE_SIZE as i32) as f32 + 32.,
    )
}

fn render_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    loaded_grid: Res<Grid>,
    loaded_path: Res<Path>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture = asset_server.load("textures/Tilesheet/towerDefense_tilesheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 23, 13, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn((
        Camera2d,
        Transform::from_xyz(
            (loaded_grid.width as u32 * TILE_SIZE) as f32 / 2.,
            (loaded_grid.height as u32 * TILE_SIZE) as f32 / 2.,
            0.,
        ),
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
                        (x * TILE_SIZE as i32) as f32 + 32.,
                        (y * TILE_SIZE as i32) as f32 + 32.,
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
                (x * TILE_SIZE as i32) as f32 + 32.,
                (y * TILE_SIZE as i32) as f32 + 32.,
                0f32,
            ),
        ));
    }

    let points: Vec<Vec3> = loaded_path
        .tiles
        .iter()
        .map(|gp| Vec2::from(grid_pos_to_screen(gp)).extend(0.))
        .collect();

    let mut mesh = Mesh::new(LineStrip, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points);

    commands.spawn((
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(ColorMaterial {
            color: Color::Srgba(RED),
            ..default()
        })),
        Transform::from_xyz(0., 0., 10.),
    ));
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
