use std::{
    collections::HashMap,
    fmt::{self, Display},
    hash::Hash,
};

use crate::loader::LoadedGame;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct MoveHalf {
    value: u32,
    node_index: usize,
    region: usize,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Move {
    pub left: MoveHalf,
    pub right: MoveHalf,
    pub domino_idx: usize,
}

impl Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "|{}|{}| at ({}, {})",
            self.left.value, self.right.value, self.left.node_index, self.right.node_index
        )
    }
}

#[derive(Debug)]
pub struct Domino {
    pub left: u32,
    pub right: u32,
    pub valid: bool,
}

impl Into<Domino> for (u32, u32) {
    fn into(self) -> Domino {
        Domino {
            left: self.0,
            right: self.1,
            valid: true,
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub x: u32,
    pub y: u32,
    pub value: Option<u32>,
    pub region_idx: usize,
    pub left: Option<usize>,
    pub up: Option<usize>,
    pub right: Option<usize>,
    pub down: Option<usize>,
}

#[derive(Debug)]
pub enum RegionType {
    Same,
    AllDifferent,
    GreaterThan(u32),
    LessThan(u32),
    SumsTo(u32),
    Blank,
}

impl Into<RegionType> for (String, u32) {
    fn into(self) -> RegionType {
        match self.0.as_str() {
            "=" => RegionType::Same,
            "!=" => RegionType::AllDifferent,
            ">" => RegionType::GreaterThan(self.1),
            "<" => RegionType::LessThan(self.1),
            "+" => RegionType::SumsTo(self.1),
            _ => RegionType::Blank,
        }
    }
}

const REGION_WIN_SCORE: u32 = 20;

#[derive(Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum RegionValidity {
    Possible = 1,
    Invalid = 0,
    Solved = REGION_WIN_SCORE,
}

#[derive(Debug)]
pub struct Region {
    pub region_type: RegionType,
    pub squares: Vec<usize>,
}

impl Region {
    pub fn get_validity(&self, graph_nodes: &[Node]) -> RegionValidity {
        let values: Vec<u32> = self
            .squares
            .iter()
            .map(|square| graph_nodes.get(*square).and_then(|node| node.value))
            .flatten()
            .collect();
        let filled_region = values.len() == self.squares.len();
        let condition_met = match self.region_type {
            RegionType::Same => values.iter().all(|x| x == &values[0]),
            RegionType::AllDifferent => {
                values.iter().all(|x| values.iter().filter(|y| *y != x).count() == 1)
            },
            RegionType::GreaterThan(v) => values.iter().sum::<u32>() > v,
            RegionType::LessThan(v) => values.iter().sum::<u32>() < v,
            RegionType::SumsTo(v) => values.iter().sum::<u32>() == v,
            RegionType::Blank => true,
        };
        if !condition_met && filled_region {
            RegionValidity::Invalid
        } else {
            if filled_region {
                RegionValidity::Solved
            } else {
                RegionValidity::Possible
            }
        }
    }
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
        for (region_idx, region) in loaded_game.regions.iter().clone().enumerate() {
            let mut nodes_idxs = vec![];
            for square in region.squares.clone() {
                let (x, y) = square;
                let [left, up, right, down] = get_neighbors(&nodes_map, x, y);
                let new_node = Node {
                    x,
                    y,
                    value: None,
                    region_idx,
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

    pub fn make_move(&mut self, domino_move: &Move) -> bool {
        let Move {
            left:
                MoveHalf {
                    value: left_value,
                    node_index: left_node_index,
                    region: _region,
                },
            right:
                MoveHalf {
                    value: right_value,
                    node_index: right_node_index,
                    region: __region,
                },
            domino_idx,
        } = domino_move;
        let (left, right) = if *left_node_index < *right_node_index {
            let (left_side, right_side) = self.nodes.split_at_mut(*right_node_index);
            (&mut left_side[*left_node_index], &mut right_side[0])
        } else {
            let (left_side, right_side) = self.nodes.split_at_mut(*left_node_index);
            (&mut right_side[0], &mut left_side[*right_node_index])
        };

        if left.value.is_none() && right.value.is_none() {
            left.value = Some(*left_value);
            right.value = Some(*right_value);
            if let Some(domino) = self.dominoes.get_mut(*domino_idx) {
                domino.valid = false;
            }
            true
        } else {
            false
        }
    }

    pub fn undo_move(&mut self, domino_move: &Move) -> bool {
        let Move {
            left:
                MoveHalf {
                    value: _left_value,
                    node_index: left_node_index,
                    region: _region,
                },
            right:
                MoveHalf {
                    value: _right_value,
                    node_index: right_node_index,
                    region: __region,
                },
            domino_idx,
        } = domino_move;
        let (left, right) = if *left_node_index < *right_node_index {
            let (left_side, right_side) = self.nodes.split_at_mut(*right_node_index);
            (&mut left_side[*left_node_index], &mut right_side[0])
        } else {
            let (left_side, right_side) = self.nodes.split_at_mut(*left_node_index);
            (&mut right_side[0], &mut left_side[*right_node_index])
        };

        if left.value.is_some() && right.value.is_some() {
            left.value = None;
            right.value = None;
            if let Some(domino) = self.dominoes.get_mut(*domino_idx) {
                domino.valid = true;
            }
            true
        } else {
            false
        }
    }

    pub fn max_score(&self) -> u32 {
        1 + self.regions.len() as u32 * REGION_WIN_SCORE
    }

    pub fn score_all_regions(&self) -> u32 {
        let mut score = 1;
        for region in self.regions.iter() {
            let region_score = region.get_validity(&self.nodes) as u32;

            if region_score == 0 {
                return 0;
            }

            score += region_score;
        }
        score
    }

    pub fn node_empty(&self, idx: &Option<usize>) -> Option<(&Node, usize)> {
        if let Some(node) = idx.and_then(|idx| self.nodes.get(idx))
            && node.value.is_none()
        {
            Some((node, idx.unwrap()))
        } else {
            None
        }
    }

    pub fn build_other_half(&self, idx: &Option<usize>, domino_value: u32) -> Option<MoveHalf> {
        if let Some((node, node_idx)) = self.node_empty(idx) {
            Some(MoveHalf {
                value: domino_value,
                node_index: node_idx,
                region: node.region_idx,
            })
        } else {
            None
        }
    }

    pub fn calculate_moves(&self) -> Vec<Move> {
        let mut moves = vec![];
        for (domino_idx, domino) in self.dominoes.iter().enumerate() {
            if !domino.valid {
                continue;
            }
            for (node_idx, node) in self.nodes.iter().enumerate() {
                let mut is_isolated = true;
                let Node {
                    left,
                    up,
                    right,
                    down,
                    x: _x,
                    y: _y,
                    value,
                    region_idx,
                } = node;
                if value.is_some() {
                    continue;
                }
                let left_half = MoveHalf {
                    value: domino.left,
                    node_index: node_idx,
                    region: *region_idx,
                };
                if let Some(move_half) = self.build_other_half(left, domino.right) {
                    moves.push(Move {
                        left: left_half,
                        right: move_half,
                        domino_idx,
                    });
                    is_isolated = false;
                }
                if let Some(move_half) = self.build_other_half(up, domino.right) {
                    moves.push(Move {
                        left: left_half,
                        right: move_half,
                        domino_idx,
                    });
                    is_isolated = false;
                }
                if let Some(move_half) = self.build_other_half(right, domino.right) {
                    moves.push(Move {
                        left: left_half,
                        right: move_half,
                        domino_idx,
                    });
                    is_isolated = false;
                }
                if let Some(move_half) = self.build_other_half(down, domino.right) {
                    moves.push(Move {
                        left: left_half,
                        right: move_half,
                        domino_idx,
                    });
                    is_isolated = false;
                }
                if is_isolated {
                    return vec![];
                }
            }
        }

        moves
    }

    pub fn regions_impossible(&self, best: &Move) -> bool {
        let Move {
            left:
                MoveHalf {
                    value: _left_value,
                    node_index: _left_node_index,
                    region: left_region,
                },
            right:
                MoveHalf {
                    value: _right_value,
                    node_index: _right_node_index,
                    region: right_region,
                },
            domino_idx: _domino_idx,
        } = best;
        if let (Some(left), Some(right)) = (
            self.regions.get(*left_region),
            self.regions.get(*right_region),
        ) {
            left.get_validity(&self.nodes) == RegionValidity::Invalid
                && right.get_validity(&self.nodes) == RegionValidity::Invalid
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_node(v: Option<u32>) -> Node {
        Node { x: 0, y: 0, value: v, region_idx: 0, left: None, up: None, right: None, down: None }
    }

    fn create_region(region_type: RegionType, values: &[Option<u32>]) -> (Region, Vec<Node>) {
        (Region { region_type, squares: (0..values.len()).collect::<Vec<_>>() }, values.iter().map(|v| {create_node(*v) }).collect::<Vec<_>>())
    }

    fn create_empty_region(region_type: RegionType) -> (Region, Vec<Node>) {
        create_region(region_type, &[None, None])
    }

    fn create_partial_region(region_type: RegionType) -> (Region, Vec<Node>) {
        create_region(region_type, &[Some(1), None, None])
    }

    fn create_full_region(region_type: RegionType) -> (Region, Vec<Node>) {
        create_region(region_type, &[Some(1), Some(2), Some(3)])
    }

    #[test]
    fn empty_region_is_possible() {
        let (region, nodes) = create_empty_region(RegionType::Blank);
        assert!(region.get_validity(&nodes) == RegionValidity::Possible)
    }

    #[test]
    fn partial_region_is_possible() {
        let (region, nodes) = create_partial_region(RegionType::Blank);
        assert!(region.get_validity(&nodes) == RegionValidity::Possible)
    }

    #[test]
    fn full_region_is_solved() {
        let (region, nodes) = create_full_region(RegionType::Blank);
        assert!(region.get_validity(&nodes) == RegionValidity::Solved)
    }
}
