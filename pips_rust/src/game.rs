use std::{
    collections::HashSet,
    fmt::{self, Display},
    ops::Shl,
};

pub const EMPTY_SQUARE: u32 = u32::MAX;
const INVALID_SQUARE: u32 = u32::MAX - 1;
const MAX_DOMINO: u32 = 9;
const REGION_WIN_SCORE: u32 = 20;

#[derive(Debug)]
pub enum RegionType {
    Same,
    GreaterThan(u32),
    LessThan(u32),
    SumsTo(u32),
    Blank,
}

#[derive(Debug)]
pub enum RegionValidity {
    Possible,
    Invalid,
    Solved,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum DominoOrientation {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
pub struct Domino {
    pub left: u32,
    pub right: u32,
}

#[derive(Debug, Clone)]
pub struct Board {
    pub squares: Vec<u32>,
    pub open_squares: u128,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
pub struct PipsGame {
    pub dominoes: Vec<Domino>,
    pub board: Board,
    pub regions: Vec<PipsRegion>,
    pub valid_squares: Vec<Point>,
    pub valid_dominoes: Vec<usize>,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub left_coord: Point,
    pub right_coord: Point,
    pub left_val: u32,
    pub right_val: u32,
    pub domino_idx: usize,
    pub domino_orientation: DominoOrientation,
}

#[derive(Debug)]
pub struct PipsRegion {
    pub region_type: RegionType,
    pub squares: Vec<Point>,
}

impl PipsRegion {
    pub fn validate(&self, values: &[u32]) -> bool {
        if self.squares.len() != values.len() {
            return false;
        }
        match self.region_type {
            RegionType::Same => values.iter().all(|x| x == &values[0]),
            RegionType::GreaterThan(val) => values.iter().sum::<u32>() > val,
            RegionType::LessThan(val) => values.iter().sum::<u32>() < val,
            RegionType::SumsTo(val) => values.iter().sum::<u32>() == val,
            RegionType::Blank => true,
        }
    }

    pub fn impossible(&self, values: &[u32]) -> bool {
        match self.region_type {
            RegionType::Same => values.iter().any(|x| x != &values[0]),
            RegionType::GreaterThan(val) => {
                values.iter().sum::<u32>() as u32
                    + ((self.squares.len() - values.len()) as u32 * MAX_DOMINO)
                    <= val
            }
            RegionType::LessThan(val) => values.iter().sum::<u32>() >= val,
            RegionType::SumsTo(val) => values.iter().sum::<u32>() > val,
            RegionType::Blank => false,
        }
    }

    pub fn get_validity(&self, values: &[u32]) -> RegionValidity {
        let filled_region = values.len() == self.squares.len();
        let condition_met = match self.region_type {
            RegionType::Same => values.iter().any(|x| x != &values[0]),
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

impl Into<Domino> for Point {
    fn into(self) -> Domino {
        Domino {
            left: self.x,
            right: self.y,
        }
    }
}

impl Into<Domino> for (u32, u32) {
    fn into(self) -> Domino {
        Domino {
            left: self.0,
            right: self.1,
        }
    }
}

impl Board {
    pub fn get(&self, row: usize, col: usize) -> Option<&u32> {
        self.squares.get(col + row * self.width)
    }

    pub fn get_point(&self, point: Point) -> Option<&u32> {
        self.squares
            .get(point.x as usize + point.y as usize * self.width)
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut u32> {
        self.squares.get_mut(col + row * self.width)
    }

    pub fn get_point_mut(&mut self, point: Point) -> Option<&mut u32> {
        self.squares
            .get_mut(point.x as usize + point.y as usize * self.width)
    }

    pub fn set(&mut self, row: usize, col: usize, val: u32) -> Option<u32> {
        if let Some(elem) = self.get_mut(row, col) {
            let original = *elem;
            *elem = val;
            if val == EMPTY_SQUARE {
                self.empty(row, col)
            } else {
                self.fill(row, col);
            };
            return Some(original);
        }
        return None;
    }

    pub fn set_point(&mut self, point: Point, val: u32) -> Option<u32> {
        self.set(point.y as usize, point.x as usize, val)
    }

    pub fn fill(&mut self, row: usize, col: usize) {
        self.open_squares = !(1u128 << row * self.width + col) & self.open_squares;
    }

    pub fn fill_point(&mut self, point: Point) {
        self.fill(point.y as usize, point.x as usize)
    }

    pub fn empty(&mut self, row: usize, col: usize) {
        self.open_squares = 1u128 << row * self.width + col | self.open_squares;
    }

    pub fn empty_point(&mut self, point: Point) {
        self.empty(point.y as usize, point.x as usize)
    }

    pub fn is_open(&self, row: usize, col: usize) -> bool {
        (row * self.width + col < 128)
            && (self.open_squares & (1u128 << row * self.width + col) > 0)
    }

    pub fn is_open_point(&self, point: Point) -> bool {
        self.is_open(point.y as usize, point.x as usize)
    }

    // pub fn len(&self) -> usize {
    //     self.squares.len()
    // }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for col in 0..self.height {
            for row in 0..self.width {
                if let Some(val) = self.get(col, row) {
                    match *val {
                        EMPTY_SQUARE => write!(f, "[ ]")?,
                        INVALID_SQUARE => write!(f, "{:^3}", "   ")?,
                        _ => write!(f, "[{}]", val)?,
                    }
                };
            }
            writeln!(f, "")?
        }
        fmt::Result::Ok(())
    }
}

impl From<(u32, u32)> for Point {
    fn from(value: (u32, u32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "|{}|{}| {:?} at ({}, {})",
            self.left_val,
            self.right_val,
            self.domino_orientation,
            self.left_coord.x,
            self.left_coord.y
        )
    }
}

const POINT_ARGUMENTS: [(i32, i32, DominoOrientation); 4] = [
    (1, 0, DominoOrientation::Right),
    (0, 1, DominoOrientation::Down),
    (-1, 0, DominoOrientation::Left),
    (0, -1, DominoOrientation::Up),
];

impl PipsGame {
    pub fn new(
        dominoes: Vec<Domino>,
        width: usize,
        height: usize,
        regions: Vec<PipsRegion>,
    ) -> Result<Self, String> {
        let mut board = Board {
            squares: vec![EMPTY_SQUARE as u32; width * height],
            open_squares: 0u128,
            width,
            height,
        };
        let mut valid_squares: Vec<Point> = vec![];
        let mut seen_squares: HashSet<Point> = HashSet::new();
        for region in &regions {
            for square in &region.squares {
                if !seen_squares.insert(*square) {
                    return Err("Regions overlap".to_string());
                };
                board.empty_point(*square);
                valid_squares.push(*square);
            }
        }
        let valid_dominoes: Vec<usize> = (0..dominoes.len()).collect();
        Ok(Self {
            board,
            regions,
            valid_squares,
            dominoes,
            valid_dominoes,
        })
    }

    fn get_squares_for_region(&self, region: &PipsRegion) -> Vec<u32> {
        region
            .squares
            .iter()
            .map(|s| self.board.get(s.x as usize, s.y as usize))
            .flatten()
            .filter(|v| **v <= MAX_DOMINO)
            .copied()
            .collect::<Vec<u32>>()
    }

    pub fn validate_regions(&self) -> Vec<(usize, RegionValidity)> {
        let mut return_vec = vec![];
        for idx in 0..self.regions.len() {
            let region = &self.regions[idx];
            return_vec.push((
                idx,
                region.get_validity(&self.get_squares_for_region(region)),
            ))
        }
        return_vec
    }

    pub fn max_score(&self) -> u32 {
        1 + self.regions.len() as u32 * REGION_WIN_SCORE
    }

    pub fn any_regions_impossible(&self) -> bool {
        self.regions
            .iter()
            .map(|r| r.impossible(&self.get_squares_for_region(r)))
            .any(|b| b)
    }

    pub fn calculate_moves(&self, domino_indexes: &[usize]) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        for domino_idx in domino_indexes {
            let Domino { left, right } = &self.dominoes.get(*domino_idx).unwrap();
            for valid_square in &self.valid_squares {
                if !self.board.is_open_point(*valid_square) {
                    continue;
                };
                let mut any_added = false;
                for point_transform in POINT_ARGUMENTS {
                    let (other_x, other_y) = (
                        valid_square.x as i32 - point_transform.0,
                        valid_square.y as i32 - point_transform.1,
                    );
                    if other_x < 0 || other_y < 0 {
                        continue;
                    }
                    if valid_square.x == 3 && valid_square.y == 0 {}
                    if self.board.is_open(other_y as usize, other_x as usize) {
                        moves.push(Move {
                            left_coord: Point {
                                x: valid_square.x,
                                y: valid_square.y,
                            },
                            right_coord: Point {
                                x: other_x as u32,
                                y: other_y as u32,
                            },
                            left_val: *left,
                            right_val: *right,
                            domino_idx: *domino_idx,
                            domino_orientation: point_transform.2,
                        });
                        any_added = true;
                    };
                }
                if !any_added {
                    return vec![];
                }
            }
        }

        moves
    }

    pub fn calculate_first_n_domino_moves(&self, n: usize) -> Vec<Move> {
        // self.calculate_moves(&self.valid_dominoes.as_slice())
        self.calculate_moves(&self.valid_dominoes.get(0..n).unwrap_or(&[]))
    }

    pub fn make_move(&mut self, domino_move: &Move) -> Option<(u32, u32)> {
        if self.board.is_open_point(domino_move.left_coord)
            && self.board.is_open_point(domino_move.right_coord)
        {
            let left_start_val = self
                .board
                .set_point(domino_move.left_coord, domino_move.left_val)
                .expect("WERID STATE");
            let right_start_val = self
                .board
                .set_point(domino_move.right_coord, domino_move.right_val)
                .expect("WEIRD STATE");
            self.valid_dominoes
                .retain(|&idx| idx != domino_move.domino_idx);
            return Some((left_start_val, right_start_val));
        }
        None
    }

    pub fn score_game(&self) -> u32 {
        let mut score: u32 = 1;
        for region_validity in self.validate_regions() {
            match region_validity.1 {
                RegionValidity::Possible => score += 1,
                RegionValidity::Invalid => return 0,
                RegionValidity::Solved => score += REGION_WIN_SCORE,
            }
        }
        score
    }

    pub fn undo_move(&mut self, domino_move: &Move, original_vals: &(u32, u32)) {
        self.board
            .set_point(domino_move.left_coord, original_vals.0);
        self.board
            .set_point(domino_move.right_coord, original_vals.1);
        self.valid_dominoes.push(domino_move.domino_idx);
    }
}
