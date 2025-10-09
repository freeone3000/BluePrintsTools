use crate::iddfs;
use crate::morajai::{PuzzleBox, act, is_solved, possible_actions};

fn act_adapt(puzzle: &PuzzleBox, action: &(usize, usize)) -> PuzzleBox {
    let mut new = *puzzle;
    act(&mut new.grid, action.0, action.1);
    new
}

/// returns the sequence of steps, or None if no solution was found to the supplied max depth
pub fn solve(puzzle_box: &PuzzleBox, max_depth: usize) -> Option<Vec<(usize, usize)>> {
    for i in std::cmp::max(10, max_depth)..=max_depth {
        let (result, remaining) = iddfs::bounded_dfs(
            puzzle_box,
            possible_actions,
            act_adapt,
            is_solved,
            vec![],
            i,
        );
        if let Some(solution) = result {
            return Some(solution);
        }
        if !remaining {
            return None;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::morajai::Square;

    #[test]
    fn test_solve_trivial() {
        let already_solved = PuzzleBox {
            target: [Square::Neutral; 4],
            grid: [[Square::Neutral; 3]; 3],
        };
        assert_eq!(solve(&already_solved, 10), Some(vec![]));
    }

    #[test]
    fn test_solve_one_step() {
        let one_step = PuzzleBox {
            target: [Square::Yellow; 4],
            grid: [
                [Square::Neutral, Square::Neutral, Square::Yellow],
                [Square::Yellow, Square::Neutral, Square::Neutral],
                [Square::Yellow, Square::Neutral, Square::Yellow],
            ],
        };
        assert_eq!(solve(&one_step, 10), Some(vec![(1, 0)]));
    }

    #[test]
    fn test_solve_multi_step() {
        let multi_step = PuzzleBox {
            target: [Square::Green; 4],
            grid: [
                [Square::Black, Square::Green, Square::Green],
                [Square::Blue, Square::Black, Square::Violet],
                [Square::Black, Square::Green, Square::Green],
            ],
        };
        assert_eq!(solve(&multi_step, 10), Some(vec![(0, 0), (2, 0)]));
    }
}
