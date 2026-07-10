use crate::{
    game_v2::PipsGraph,
    loader::{LoadedGame, load_games},
    solver::solve_game,
};

mod game_v2;
mod loader;
mod solver;

fn main() {
    let loaded_games: Vec<LoadedGame> = load_games("games.json");

    let mut pips_games: Vec<PipsGraph> = loaded_games
        .iter()
        .map(|loaded| PipsGraph::new(loaded))
        .collect();

    // for graph in pips_games {
    //     println!("{:?}", graph);
    // }

    let puzzle_3 = pips_games.get_mut(2).unwrap();

    let (score, path) = solve_game(puzzle_3, 3);
    println!("{}", score);
    for domino_move in path {
        puzzle_3.make_move(&domino_move);
        println!("{}", domino_move);
    }
    // println!("{}", puzzle_3.board);

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
