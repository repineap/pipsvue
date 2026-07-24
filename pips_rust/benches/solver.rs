use std::hint::black_box;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use pips_rust::{game_v2::PipsGraph, loader::load_games, solver::solve_game};

pub fn bench_solver(c: &mut Criterion) {
    let games = load_games("games.json");
    c.bench_function("solve puzzle3 depth3", |b| {
        b.iter_batched(
            || PipsGraph::new(&games[2]),
            |mut game| black_box(solve_game(&mut game, 3)),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("solve puzzle3 depth4", |b| {
        b.iter_batched(
            || PipsGraph::new(&games[2]),
            |mut game| black_box(solve_game(&mut game, 4)),
            BatchSize::SmallInput,
        )
    });
}
criterion_group!(benches, bench_solver);
criterion_main!(benches);
