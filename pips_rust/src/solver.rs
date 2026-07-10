use std::collections::{HashMap, HashSet};

use crate::game_v2::{Move, PipsGraph};

pub fn start_search(
    pips_game: &mut PipsGraph,
    depth: u32,
    seen: &mut HashSet<Move>,
    last_move: Option<&Move>,
) -> (u32, Vec<Move>) {
    let moves = pips_game.calculate_moves();

    if depth == 0 || moves.is_empty() {
        return (pips_game.score_all_regions(), vec![]);
    }

    let impossible = if let Some(last_move_val) = last_move {
        pips_game.regions_impossible(last_move_val)
    } else {
        pips_game.score_all_regions() == 0
    };

    if impossible {
        return (0, vec![]);
    }

    let mut max_score: u32 = 0;
    let mut best_path: Vec<Move> = vec![];
    let mut current_best_move: Option<Move> = None;

    for move_val in moves.iter() {
        if seen.contains(&move_val) {
            continue;
        }
        if !pips_game.make_move(move_val) {
            panic!()
        };

        let (score, child_path) = start_search(pips_game, depth - 1, seen, Some(move_val));
        if score >= max_score {
            max_score = score;
            best_path = child_path;
            current_best_move = Some(*move_val);
        }
        if !pips_game.undo_move(&move_val) {
            panic!()
        };
    }
    if let Some(winning_move) = current_best_move {
        best_path.insert(0, winning_move);
    }
    (max_score, best_path)
}

pub fn solve_game(pips_game: &mut PipsGraph, depth: u32) -> (bool, Vec<Move>) {
    let max_score = pips_game.max_score();
    let mut seen: HashMap<Vec<Move>, HashSet<Move>> = HashMap::new();

    let mut final_path: Vec<Move> = vec![];
    loop {
        let mut seen_set = seen.entry(final_path.clone()).or_insert(HashSet::new());
        let (best_score, best_path) =
            start_search(pips_game, depth, &mut seen_set, final_path.last());
        if best_score == max_score {
            for move_to_undo in final_path.iter() {
                if !pips_game.undo_move(move_to_undo) {
                    panic!()
                };
            }
            final_path.extend(best_path);
            return (true, final_path);
        }
        if let Some(best) = best_path.get(0) {
            if !pips_game.make_move(best) {
                panic!()
            };
            seen_set.insert(*best);
            final_path.push(*best);
        } else {
            if best_score == max_score {
                final_path.extend(best_path);
                return (true, final_path);
            }
            if let Some(last_move) = &final_path.pop() {
                seen_set.clear();
                if !pips_game.undo_move(last_move) {
                    panic!()
                };
            } else {
                return (false, final_path);
            }
        }
        if best_score == max_score {
            final_path.extend(best_path);
            return (true, final_path);
        }
    }
}
