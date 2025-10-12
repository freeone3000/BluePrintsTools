use criterion::{criterion_group, criterion_main, Criterion};
use blue_prince::morajai::{act, is_solved, possible_actions, PuzzleBox};
use blue_prince::search;

use blue_prince::morajai::Square::*;

fn act_adapt(puzzle: &PuzzleBox, action: &(usize, usize)) -> PuzzleBox {
    let mut new = *puzzle;
    act(&mut new.grid, action.0, action.1);
    new
}

fn solve_bfs(puzzle_box: &PuzzleBox) -> Option<Vec<(usize, usize)>> {
    search::bfs(
        puzzle_box,
        possible_actions,
        act_adapt,
        is_solved,
    )
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


criterion_group!(benches, bench_solve_bfs);
criterion_main!(benches);