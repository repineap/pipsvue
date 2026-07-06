use std::collections::{HashMap, HashSet};

use crate::game::{EMPTY_SQUARE, Move, PipsGame};

pub fn start_search(
    pips_game: &mut PipsGame,
    depth: u32,
    seen: &mut HashSet<Move>,
) -> (u32, Vec<Move>) {
    let moves = pips_game.calculate_first_n_domino_moves(3);

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

pub fn solve_game(pips_game: &mut PipsGame, depth: u32) -> (bool, Vec<Move>) {
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
                // seen.remove(&final_path);
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
