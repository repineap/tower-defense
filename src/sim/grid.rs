use bevy::ecs::resource::Resource;

#[derive(Debug)]
pub struct GridPos {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug)]
pub enum TileKind {
    Grass,
    Road,
    Spawn,
    Base,
    Rock,
}

impl From<char> for TileKind {
    fn from(value: char) -> Self {
        match value {
            '.' => TileKind::Grass,
            'r' => TileKind::Rock,
            '#' => TileKind::Road,
            'S' => TileKind::Spawn,
            'B' => TileKind::Base,
            _ => TileKind::Grass,
        }
    }
}

#[derive(Resource, Debug)]
pub struct Grid {
    width: i32,
    height: i32,
    tiles: Vec<TileKind>,
    spawns: Vec<GridPos>,
    bases: Vec<GridPos>,
    occupied: Vec<bool>,
}

impl Grid {
    pub fn new(width: i32, height: i32, tile_rows: &[String]) -> Self {
        let row_vecs: Vec<Vec<TileKind>> = tile_rows
            .iter()
            .map(|row_string| {
                row_string
                    .chars()
                    .map(|tile_char| tile_char.into())
                    .collect()
            })
            .collect();
        let mut tiles = vec![];
        for row in row_vecs.iter().rev() {
            tiles.extend(row);
        }
        let mut spawns = vec![];
        let mut bases = vec![];
        for (idx, tile) in tiles.iter().enumerate() {
            match tile {
                TileKind::Base => bases.push(Self::index_to_pos(width, idx)),
                TileKind::Spawn => spawns.push(Self::index_to_pos(width, idx)),
                _ => {}
            }
        }
        let mut occupied = vec![];
        occupied.resize((width * height) as usize, false);
        Self {
            width,
            height,
            tiles,
            spawns,
            bases,
            occupied,
        }
    }

    fn index(&self, pos: &GridPos) -> usize {
        (pos.x + pos.y * self.width) as usize
    }

    fn index_to_pos(width: i32, idx: usize) -> GridPos {
        GridPos {
            x: idx as i32 % width,
            y: idx as i32 / width,
        }
    }

    pub fn get(&self, pos: &GridPos) -> Option<TileKind> {
        self.tiles.get(self.index(pos)).cloned()
    }

    fn is_occupied(&self, pos: &GridPos) -> bool {
        if let Some(occupied) = self.occupied.get(self.index(pos)) {
            *occupied
        } else {
            true
        }
    }

    pub fn is_buildable(&self, pos: &GridPos) -> bool {
        if let Some(tile_kind) = self.get(pos) {
            match tile_kind {
                TileKind::Grass => self.is_occupied(pos),
                _ => false,
            }
        } else {
            false
        }
    }
}
