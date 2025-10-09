use crate::search;
use crate::morajai::{PuzzleBox, act, is_solved, possible_actions, is_solvable};

fn act_adapt(puzzle: &PuzzleBox, action: &(usize, usize)) -> PuzzleBox {
    let mut new = *puzzle;
    act(&mut new.grid, action.0, action.1);
    new
}

/// returns the sequence of steps, or None if no solution was found to the supplied max depth
pub fn solve(puzzle_box: &PuzzleBox, _max_depth: usize) -> Option<Vec<(usize, usize)>> {
    search::bfs(
        puzzle_box,
        possible_actions,
        act_adapt,
        is_solved,
        is_solvable,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::morajai::Square;
    use Square::*;

    #[test]
    fn test_solve_trivial() {
        let already_solved = PuzzleBox {
            target: [Neutral; 4],
            grid: [[Neutral; 3]; 3],
        };
        assert_eq!(solve(&already_solved, 10), Some(vec![]));
    }

    #[test]
    fn test_solve_one_step() {
        let one_step = PuzzleBox {
            target: [Yellow; 4],
            grid: [
                [Neutral, Neutral, Yellow],
                [Yellow, Neutral, Neutral],
                [Yellow, Neutral, Yellow],
            ],
        };
        assert_eq!(solve(&one_step, 10), Some(vec![(1, 0)]));
    }

    #[test]
    fn test_solve_multi_step() {
        let multi_step = PuzzleBox {
            target: [Green; 4],
            grid: [
                [Black, Green, Green],
                [Blue, Black, Violet],
                [Black, Green, Green],
            ],
        };
        assert_eq!(solve(&multi_step, 3), Some(vec![(0, 0), (2, 0)]));
    }

    #[test]
    fn test_solve_case_1() {
        let case_1 = PuzzleBox {
            target: [Blue;4],
            grid: [
                [White, White, White],
                [Yellow, White, Black],
                [Blue, Blue, Blue]
            ]
        };
        assert!(solve(&case_1, 20).is_some());
    }

    #[test]
    fn test_solve_case_2() {
        let case_2 = PuzzleBox {
            target: [Green; 4],
            grid: [
                [Orange, Yellow, Orange],
                [Green, Neutral, Green],
                [Blue, Green, Blue]
            ]
        };
        let solution = vec![(2, 1), (0, 0), (0, 2), (2, 1), (2, 0), (2, 2)];
        assert_eq!(solve(&case_2, solution.len()), Some(solution));
    }

    #[test]
    fn test_solve_case_3() {
        let case_3 = PuzzleBox {
            target: [Orange;4],
            grid: [
                [Orange, Green, Orange],
                [Yellow, Orange, Orange],
                [Blue, Yellow, Orange],
            ]
        };
        assert!(solve(&case_3, 20).is_some());
    }

    #[test]
    fn test_solve_case_4() {
        let case_4 = PuzzleBox {
            target: [Red;4],
            grid: [
                [Violet, White, Violet],
                [Blue, Neutral, Blue],
                [Pink, Red, Pink]
            ]
        };
        assert!(solve(&case_4, 20).is_some());
    }
}
