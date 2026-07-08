use std::{collections::HashMap, hash::Hash};

use crate::loader::LoadedGame;

#[derive(Debug)]
pub struct Domino {
    pub left: u32,
    pub right: u32,
}

impl Into<Domino> for (u32, u32) {
    fn into(self) -> Domino {
        Domino {
            left: self.0,
            right: self.1,
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub x: u32,
    pub y: u32,
    pub v: Option<u32>,
    pub left: Option<usize>,
    pub up: Option<usize>,
    pub right: Option<usize>,
    pub down: Option<usize>,
}

#[derive(Debug)]
pub enum RegionType {
    Same,
    GreaterThan(u32),
    LessThan(u32),
    SumsTo(u32),
    Blank,
}

impl Into<RegionType> for (String, u32) {
    fn into(self) -> RegionType {
        match self.0.as_str() {
            "=" => RegionType::Same,
            ">" => RegionType::GreaterThan(self.1),
            "<" => RegionType::LessThan(self.1),
            "+" => RegionType::SumsTo(self.1),
            _ => RegionType::Blank,
        }
    }
}

#[derive(Debug)]
pub enum RegionValidity {
    Possible,
    Invalid,
    Solved,
}

#[derive(Debug)]
pub struct Region {
    pub region_type: RegionType,
    pub squares: Vec<usize>,
}

#[derive(Debug)]
pub struct PipsGraph {
    pub nodes: Vec<Node>,
    pub regions: Vec<Region>,
    pub dominoes: Vec<Domino>,
}

impl PipsGraph {
    pub fn new(loaded_game: &LoadedGame) -> Self {
        fn get_neighbors(
            nodes_map: &HashMap<(u32, u32), usize>,
            x: u32,
            y: u32,
        ) -> [Option<usize>; 4] {
            const DIRS: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

            std::array::from_fn(|i| {
                let (dx, dy) = DIRS[i];
                let nx = x.checked_add_signed(dx)?;
                let ny = y.checked_add_signed(dy)?;
                nodes_map.get(&(nx, ny)).copied()
            })
        }
        let mut nodes = vec![];
        let mut nodes_map: HashMap<(u32, u32), usize> = HashMap::new();
        let mut regions = vec![];
        for region in loaded_game.regions.iter().clone() {
            let mut nodes_idxs = vec![];
            for square in region.squares.clone() {
                let (x, y) = square;
                let [left, up, right, down] = get_neighbors(&nodes_map, x, y);
                let new_node = Node {
                    x,
                    y,
                    v: None,
                    left,
                    up,
                    right,
                    down,
                };
                nodes.push(new_node);
                let idx = nodes.len() - 1;
                nodes_map.insert((x, y), idx);
                nodes_idxs.push(idx);
                if let Some(left_node) = left.and_then(|l| nodes.get_mut(l)) {
                    left_node.right = Some(idx);
                }
                if let Some(up_node) = up.and_then(|u| nodes.get_mut(u)) {
                    up_node.down = Some(idx);
                }
                if let Some(right_node) = right.and_then(|r| nodes.get_mut(r)) {
                    right_node.left = Some(idx);
                }
                if let Some(down_node) = down.and_then(|d| nodes.get_mut(d)) {
                    down_node.up = Some(idx);
                }
            }
            regions.push(Region {
                region_type: (region.region_type.clone(), region.region_value).into(),
                squares: nodes_idxs,
            })
        }
        PipsGraph {
            nodes,
            regions,
            dominoes: loaded_game
                .dominoes
                .clone()
                .into_iter()
                .map(|d| d.into())
                .collect(),
        }
    }
}
