use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display},
    hash::Hash,
    time::Instant,
};

// use rand::Rng;

pub enum RegionType {
    Same,
    GreaterThan(u32),
    LessThan(u32),
    SumsTo(u32),
    Blank,
}

pub enum RegionValidity {
    Possible,
    Invalid,
    Solved,
}

const EMPTY_SQUARE: u32 = u32::MAX;
const INVALID_SQUARE: u32 = u32::MAX - 1;
const MAX_DOMINO: u32 = 9;
const REGION_WIN_SCORE: u32 = 20;

pub struct PipsRegion {
    region_type: RegionType,
    squares: Vec<Point>,
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

//https://crates.io/crates/rusqlite

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum DominoOrientation {
    Up,
    Right,
    Down,
    Left,
}

pub struct Domino {
    left: u32,
    right: u32,
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

#[derive(Clone)]
pub struct Board {
    squares: Vec<u32>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn get(&self, row: usize, col: usize) -> Option<&u32> {
        self.squares.get(row + col * self.width)
    }

    pub fn get_point(&self, point: Point) -> Option<&u32> {
        self.squares
            .get(point.x as usize + point.y as usize * self.width)
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut u32> {
        self.squares.get_mut(row + col * self.width)
    }

    pub fn get_point_mut(&mut self, point: Point) -> Option<&mut u32> {
        self.squares
            .get_mut(point.x as usize + point.y as usize * self.width)
    }

    pub fn set(&mut self, row: usize, col: usize, val: u32) -> Option<u32> {
        if let Some(elem) = self.get_mut(row, col) {
            let original = *elem;
            *elem = val;
            return Some(original);
        }
        return None;
    }

    pub fn set_point(&mut self, point: Point, val: u32) -> Option<u32> {
        if let Some(elem) = self.get_point_mut(point) {
            let original = *elem;
            *elem = val;
            return Some(original);
        }
        return None;
    }

    pub fn is_valid(&self, row: usize, col: usize) -> bool {
        if let Some(val) = self.get(row, col) {
            return *val == EMPTY_SQUARE;
        }
        false
    }

    pub fn is_valid_point(&self, point: Point) -> bool {
        if let Some(val) = self.get(point.x as usize, point.y as usize) {
            return *val == EMPTY_SQUARE;
        }
        false
    }

    pub fn len(&self) -> usize {
        self.squares.len()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for col in 0..self.height {
            for row in 0..self.width {
                if let Some(val) = self.get(row, col) {
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

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    x: u32,
    y: u32,
}

impl From<(u32, u32)> for Point {
    fn from(value: (u32, u32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

pub struct PipsGame {
    dominoes: Vec<Domino>,
    board: Board,
    regions: Vec<PipsRegion>,
    valid_squares: Vec<Point>,
    valid_dominoes: Vec<usize>,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    left_coord: Point,
    right_coord: Point,
    left_val: u32,
    right_val: u32,
    domino_idx: usize,
    domino_orientation: DominoOrientation,
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
            squares: vec![INVALID_SQUARE as u32; width * height],
            width,
            height,
        };
        let mut valid_squares: Vec<Point> = vec![];
        for region in &regions {
            for square in &region.squares {
                let Point { x, y } = square;
                let (x_idx, y_idx) = (*x as usize, *y as usize);
                if let Some(val) = board.get(x_idx, y_idx) {
                    if *val != INVALID_SQUARE {
                        return Err("Regions overlap".to_string());
                    }
                }
                if board.set(x_idx, y_idx, EMPTY_SQUARE).is_none() {
                    return Err("Regions do not fit".to_string());
                } else {
                    valid_squares.push(*square);
                }
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

    pub fn calculate_all_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        for domino_idx in &self.valid_dominoes {
            let Domino { left, right } = &self.dominoes.get(*domino_idx).unwrap();
            for valid_square in &self.valid_squares {
                if !self.board.is_valid_point(*valid_square) {
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
                    if self.board.is_valid(other_x as usize, other_y as usize) {
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

    pub fn calculate_last_domino_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        if self.valid_dominoes.len() > 0 {
            let domino_idx = self.valid_dominoes.len() - 1;
            let Domino { left, right } = &self.dominoes.get(domino_idx).unwrap();
            for valid_square in &self.valid_squares {
                if !self.board.is_valid_point(*valid_square) {
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
                    if self.board.is_valid(other_x as usize, other_y as usize) {
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
                            domino_idx: domino_idx,
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

    pub fn make_move(&mut self, domino_move: &Move) -> Option<(u32, u32)> {
        if let Some(left_start_val) = self
            .board
            .set_point(domino_move.left_coord, domino_move.left_val)
        {
            if let Some(right_start_val) = self
                .board
                .set_point(domino_move.right_coord, domino_move.right_val)
            {
                self.valid_dominoes
                    .retain(|&idx| idx != domino_move.domino_idx);
                return Some((left_start_val, right_start_val));
            } else {
                self.board.set_point(domino_move.left_coord, left_start_val)
            };
        };
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

pub fn create_pips_game(
    dominoes: Vec<(u32, u32)>,
    regions: Vec<(Vec<(u32, u32)>, RegionType)>,
) -> Result<PipsGame, String> {
    let domino_vec: Vec<Domino> = dominoes.into_iter().map(|d| d.into()).collect();
    let mut region_vec: Vec<PipsRegion> = vec![];
    let mut width: usize = 0;
    let mut height: usize = 0;
    for region in regions {
        let mut temp_region = PipsRegion {
            region_type: region.1,
            squares: vec![],
        };
        for square in region.0 {
            let point: Point = square.into();
            temp_region.squares.push(point);
            width = std::cmp::max(width, point.x as usize + 1);
            height = std::cmp::max(height, point.y as usize + 1);
        }
        region_vec.push(temp_region);
    }
    PipsGame::new(domino_vec, width, height, region_vec)
}

fn main() {
    let mut pips_game = create_pips_game(
        vec![(0, 5), (6, 3), (5, 1), (3, 2), (2, 2), (2, 1)],
        vec![
            (vec![(1, 0), (1, 1), (2, 0)], RegionType::GreaterThan(13)),
            (vec![(3, 0)], RegionType::SumsTo(3)),
            (vec![(1, 2)], RegionType::GreaterThan(0)),
            (vec![(0, 0), (1, 4)], RegionType::Blank),
            (vec![(2, 4), (3, 2), (3, 3), (3, 4)], RegionType::Same),
            (vec![(4, 4)], RegionType::LessThan(2)),
        ],
    )
    .unwrap();

    fn start_search(
        pips_game: &mut PipsGame,
        depth: u32,
        seen: &mut HashSet<Move>,
    ) -> (u32, Vec<Move>) {
        let moves = pips_game.calculate_last_domino_moves();

        if depth == 0 || moves.is_empty() {
            return (pips_game.score_game(), vec![]);
        }
        if pips_game.any_regions_impossible() {
            return (0, vec![]);
        }

        let mut max_score: u32 = 0;
        let mut best_path: Vec<Move> = vec![];
        let mut current_best_move: Option<Move> = None;

        for move_val in moves {
            if seen.contains(&move_val) {
                continue;
            }
            let original_vals = pips_game.make_move(&move_val).unwrap();

            let (score, child_path) = start_search(pips_game, depth - 1, seen);
            if score >= max_score {
                max_score = score;
                best_path = child_path;
                current_best_move = Some(move_val);
            }
            pips_game.undo_move(&move_val, &original_vals);
        }
        if let Some(winning_move) = current_best_move {
            best_path.insert(0, winning_move);
        }
        (max_score, best_path)
    }

    fn solve_game(pips_game: &mut PipsGame, depth: u32) -> (bool, Vec<Move>) {
        let max_score = pips_game.max_score();
        let mut seen: HashMap<Vec<Move>, HashSet<Move>> = HashMap::new();

        let mut final_path: Vec<Move> = vec![];
        loop {
            let mut seen_set = seen.entry(final_path.clone()).or_insert(HashSet::new());
            let (best_score, best_path) = start_search(pips_game, depth, &mut seen_set);
            if best_score == max_score {
                for move_to_undo in final_path.iter() {
                    pips_game.undo_move(move_to_undo, &(EMPTY_SQUARE, EMPTY_SQUARE));
                }
                final_path.extend(best_path);
                return (true, final_path);
            }
            if let Some(best) = best_path.get(0) {
                pips_game.make_move(best);
                final_path.push(*best);
                seen_set.insert(*best);
            } else {
                if best_score == max_score {
                    final_path.extend(best_path);
                    return (true, final_path);
                }
                if let Some(last_move) = &final_path.pop() {
                    seen_set.clear();
                    seen.remove(&final_path);
                    pips_game.undo_move(last_move, &(EMPTY_SQUARE, EMPTY_SQUARE));
                } else {
                    return (false, final_path);
                }
            }
            if best_score == max_score {
                final_path.extend(best_path);
                return (true, final_path);
            }
        }
        // (false, final_path)
    }
    // let (solved, path) = solve_game(&mut pips_game, 99);
    // // for domino_move in path {
    // //     pips_game.make_move(domino_move);
    // //     println!("{}", domino_move);
    // // }
    // println!("{} {}\n{:?}", pips_game.board, solved, path);

    let mut today_hard_game = create_pips_game(
        vec![
            (2, 0),
            (6, 4),
            (0, 0),
            (2, 2),
            (5, 5),
            (0, 6),
            (4, 2),
            (6, 5),
            (0, 4),
            (1, 5),
            (6, 6),
            (4, 1),
            (0, 3),
            (1, 2),
            (3, 4),
        ],
        vec![
            (vec![(0, 1), (6, 5), (6, 6)], RegionType::Blank),
            (vec![(6, 0)], RegionType::GreaterThan(4)),
            (vec![(6, 1), (7, 1), (6, 2)], RegionType::Same),
            (vec![(8, 1), (8, 2)], RegionType::SumsTo(0)),
            (vec![(7, 2)], RegionType::LessThan(4)),
            (vec![(0, 2), (1, 2)], RegionType::SumsTo(11)),
            (vec![(0, 3), (1, 3), (2, 3), (0, 4)], RegionType::SumsTo(0)),
            (vec![(3, 3), (4, 3)], RegionType::Same),
            (vec![(5, 3), (4, 4), (5, 4)], RegionType::Same),
            (vec![(3, 4)], RegionType::GreaterThan(4)),
            (vec![(6, 3), (6, 4)], RegionType::SumsTo(4)),
            (vec![(1, 4), (2, 4)], RegionType::SumsTo(5)),
            (vec![(1, 5), (1, 6), (2, 6)], RegionType::SumsTo(13)),
            (vec![(7, 6)], RegionType::SumsTo(4)),
        ],
    )
    .unwrap();

    println!(
        "{} {}",
        &today_hard_game.board.width, &today_hard_game.board.height
    );

    let now = Instant::now();

    // for _ in 0..10 {
    //     let (_score, _path) = solve_game(&mut today_hard_game, 3);
    // }
    //
    let (score, path) = solve_game(&mut today_hard_game, 4);

    for domino_move in path {
        today_hard_game.make_move(&domino_move);
        println!("{}", domino_move);
    }
    println!("{}", today_hard_game.board);
    println!("{} (Time: {})", score, now.elapsed().as_millis());
}
