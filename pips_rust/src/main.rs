use std::{fs, time::Instant};

use serde::Deserialize;

use crate::{
    game::{PipsGame, PipsRegion, Point, RegionType},
    solver::solve_game,
};

mod game;
mod solver;

#[derive(Deserialize, Debug)]
struct LoadedRegion {
    squares: Vec<(u32, u32)>,
    region_type: String,
    region_value: u32,
}

#[derive(Deserialize, Debug)]
struct LoadedGame {
    dominoes: Vec<(u32, u32)>,
    regions: Vec<LoadedRegion>,
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

impl Into<Option<PipsGame>> for LoadedGame {
    fn into(self) -> Option<PipsGame> {
        let mut region_vec: Vec<PipsRegion> = vec![];
        let mut width: usize = 0;
        let mut height: usize = 0;
        for region in self.regions {
            let mut temp_region = PipsRegion {
                region_type: (region.region_type, region.region_value).into(),
                squares: vec![],
            };
            for square in region.squares {
                let point: Point = square.into();
                temp_region.squares.push(point);
                width = std::cmp::max(width, point.x as usize + 1);
                height = std::cmp::max(height, point.y as usize + 1);
            }
            region_vec.push(temp_region);
        }
        if (width * height) > 128 {
            return None;
        }
        PipsGame::new(
            self.dominoes.into_iter().map(|d| d.into()).collect(),
            width,
            height,
            region_vec,
        )
        .ok()
    }
}

fn main() {
    let games_content = fs::read_to_string("games.json").unwrap();
    let loaded_games: Vec<LoadedGame> = serde_json::from_str(&games_content).unwrap();

    let mut pips_games: Vec<PipsGame> = loaded_games.into_iter().filter_map(|g| g.into()).collect();

    let puzzle_3 = pips_games.get_mut(1).unwrap();

    let (_, path) = solve_game(puzzle_3, 3);
    for domino_move in path {
        puzzle_3.make_move(&domino_move);
        println!("{}", domino_move);
    }
    println!("{}", puzzle_3.board);

    // println!(
    //     "{} {}",
    //     &today_hard_game.board.width, &today_hard_game.board.height
    // );

    // let now = Instant::now();

    // // for _ in 0..10 {
    // //     let (_score, _path) = solve_game(&mut today_hard_game, 3);
    // // }
    // //
    // let (score, path) = solve_game(&mut today_hard_game, 4);

    // for domino_move in path {
    //     today_hard_game.make_move(&domino_move);
    //     println!("{}", domino_move);
    // }
    // println!("{}", today_hard_game.board);
    // println!("{} (Time: {})", score, now.elapsed().as_millis());
}
