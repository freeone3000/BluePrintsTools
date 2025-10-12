use criterion::{criterion_group, criterion_main, Criterion};
use blue_prince::morajai::{act, is_solvable, is_solved, possible_actions, PuzzleBox};
use blue_prince::search;

use blue_prince::morajai::Square::*;

// TODO look into if we actually need this method at all?
fn act_adapt(puzzle: &PuzzleBox, action: &(usize, usize)) -> PuzzleBox {
    let mut new = *puzzle;
    act(&mut new.grid, action.0, action.1);
    new
}

pub fn solve_iddfs(puzzle_box: &PuzzleBox, max_depth: usize) -> Option<Vec<(usize, usize)>> {
    for i in 0..=max_depth {
        let (result, remaining) = search::bounded_dfs(
            puzzle_box,
            possible_actions,
            act_adapt,
            is_solved,
            is_solvable,
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

fn solve_bfs(puzzle_box: &PuzzleBox) -> Option<Vec<(usize, usize)>> {
    search::bfs(
        puzzle_box,
        possible_actions,
        act_adapt,
        is_solved,
        is_solvable,
    )
}

fn bench_solve_iddfs(c: &mut Criterion) {
    let case_1 = PuzzleBox {
        target: [Blue;4],
        grid: [
            [White, White, White],
            [Yellow, White, Black],
            [Blue, Blue, Blue]
        ]
    };
    c.bench_function("solve_iddfs_case_1", |b| b.iter(|| {
        solve_iddfs(&case_1, 100)
    }));
}

fn bench_solve_bfs(c: &mut Criterion) {
    let case_1 = PuzzleBox {
        target: [Blue;4],
        grid: [
            [White, White, White],
            [Yellow, White, Black],
            [Blue, Blue, Blue]
        ]
    };
    c.bench_function("solve_bfs_case_1", |b| b.iter(|| {
        solve_bfs(&case_1)
    }));

}


criterion_group!(benches, bench_solve_bfs, bench_solve_iddfs);
criterion_main!(benches);