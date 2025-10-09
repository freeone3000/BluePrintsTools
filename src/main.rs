use crate::morajai::{PuzzleBox, Square};
use crate::morajai_display::char_to_square;

mod iddfs;
mod morajai;
mod morajai_display;
mod solve;

const MAX_DEPTH: usize = 100;

fn main() {
    let mut corners: String = String::new();
    let mut box_lines: [String; 3] = [String::new(), String::new(), String::new()];
    println!("The following short codes are used:");
    println!("{}", morajai_display::generate_legend());
    println!(
        "Counting from the TOP LEFT, going CLOCKWISE (entering one symbol only for symmetric boxes):"
    );
    println!(
        "Please enter your CORNERS on ONE LINE using short codes. No spaces between letters is required."
    );
    std::io::stdin()
        .read_line(&mut corners)
        .expect("Failed to read line");
    println!(
        "Please enter your box on THREE LINES using short codes. No spaces between letters is required."
    );
    for line in &mut box_lines {
        std::io::stdin()
            .read_line(line)
            .expect("Failed to read line");
    }

    let puzzle = PuzzleBox {
        target: line_to_corners(&corners),
        grid: lines_to_grid(box_lines),
    };
    println!("Solving puzzle:\n {}", puzzle);
    let solution = solve::solve(&puzzle, MAX_DEPTH);

    println!("Solution: ");
    match solution {
        Some(solution) => {
            if solution.is_empty() {
                println!("(Already solved)");
            } else {
                for step in &solution {
                    println!("Press at row {}, column {}", step.0 + 1, step.1 + 1);
                }
            }
        }
        None => println!("No solution found within {} steps", MAX_DEPTH),
    }
}

fn lines_to_grid(lines: [String; 3]) -> [[Square; 3]; 3] {
    lines.map(|line| {
        let mut iter = line.trim().chars();
        [
            char_to_square(iter.next().unwrap_or('.')),
            char_to_square(iter.next().unwrap_or('.')),
            char_to_square(iter.next().unwrap_or('.')),
        ]
    })
}

fn line_to_corners(line: &str) -> [Square; 4] {
    let mut iter = line.trim().chars();
    let first = iter.next().expect("has at least one character");
    let second = iter.next().unwrap_or(first);
    let third = iter.next().unwrap_or(first);
    let fourth = iter.next().unwrap_or(first);
    [
        char_to_square(first),
        char_to_square(second),
        char_to_square(third),
        char_to_square(fourth),
    ]
}
