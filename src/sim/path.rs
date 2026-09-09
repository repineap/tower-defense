use crate::sim::grid::{Grid, GridPos, TileKind};
use std::collections::{HashMap, VecDeque};

use bevy::ecs::resource::Resource;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PathError {
    #[error("No path found between start point and end point")]
    PathNotFoundError,
}

#[derive(Resource, Debug, PartialEq, Eq)]
pub struct Path {
    pub tiles: Vec<GridPos>,
    pub total_mt: i32,
}

pub fn find_path(grid: &Grid) -> Result<Path, PathError> {
    for start_pos in &grid.spawns {
        let mut pos_queue = VecDeque::from(vec![*start_pos]);
        let mut pos_map = HashMap::new();
        loop {
            if let Some(current_pos) = pos_queue.pop_front() {
                if let Some(current_tile_kind) = grid.get(&current_pos) {
                    match current_tile_kind {
                        TileKind::Road | TileKind::Spawn => {
                            for neighbor in [
                                current_pos.up_pos(),
                                current_pos.right_pos(),
                                current_pos.down_pos(),
                                current_pos.left_pos(),
                            ] {
                                if pos_map.contains_key(&neighbor) {
                                    continue;
                                }
                                pos_map.insert(neighbor, current_pos);
                                pos_queue.push_back(neighbor);
                            }
                        }
                        TileKind::Base => {
                            let mut path = vec![current_pos];
                            let mut path_head = current_pos;
                            loop {
                                if let Some(parent_pos) = pos_map.get(&path_head) {
                                    if let Some(parent_type) = grid.get(parent_pos) {
                                        match parent_type {
                                            TileKind::Spawn => {
                                                path.push(*parent_pos);
                                                break;
                                            }
                                            TileKind::Road => path.push(*parent_pos),
                                            _ => return Err(PathError::PathNotFoundError),
                                        }
                                    }
                                    path_head = *parent_pos;
                                } else {
                                    return Err(PathError::PathNotFoundError);
                                }
                            }
                            let path_len = path.len();
                            path.reverse();
                            return Ok(Path {
                                tiles: path,
                                total_mt: path_len as i32 * 1000,
                            });
                        }
                        _ => {}
                    }
                }
            } else {
                return Err(PathError::PathNotFoundError);
            }
        }
    }
    Err(PathError::PathNotFoundError)
}

#[cfg(test)]
mod tests {
    use crate::sim::{
        grid::{Grid, GridPos},
        path::{Path, find_path},
    };

    #[test]
    fn simple_grid_finds_path() {
        let grid = Grid::new(
            5,
            5,
            &[
                ".....".to_string(),
                "###..".to_string(),
                "S.#.B".to_string(),
                "..###".to_string(),
                ".....".to_string(),
            ],
        );

        let path = find_path(&grid);
        assert!(path.is_ok());
        assert_eq!(
            path.unwrap(),
            Path {
                tiles: vec![
                    GridPos::new(0, 2),
                    GridPos::new(0, 3),
                    GridPos::new(1, 3),
                    GridPos::new(2, 3),
                    GridPos::new(2, 2),
                    GridPos::new(2, 1),
                    GridPos::new(3, 1),
                    GridPos::new(4, 1),
                    GridPos::new(4, 2),
                ],
                total_mt: 9000
            }
        )
    }
}
