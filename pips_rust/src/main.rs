use pips_rust::{game_v2::PipsGraph, loader::{LoadedGame, load_games}, solver::solve_game};


fn main() {
    let loaded_games: Vec<LoadedGame> = load_games("games.json");

    for _ in 0..150 {
        let mut g = PipsGraph::new(&loaded_games[2]);
        std::hint::black_box(solve_game(&mut g, 3));
    }
}
