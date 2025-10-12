use blue_prince::morajai::{act, solve, PuzzleBox, Square};

const MAX_DEPTH: usize = 100;

fn main() {
    let mut corners: String = String::new();
    let mut box_lines: [String; 3] = [String::new(), String::new(), String::new()];
    println!("The following short codes are used:");
    println!("{}", generate_legend());
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

    let mut puzzle = PuzzleBox {
        target: line_to_corners(&corners),
        grid: lines_to_grid(box_lines),
    };
    println!("Solving puzzle:\n {}", puzzle);
    let solution = solve(&puzzle, MAX_DEPTH);

    println!("Solution: ");
    match solution {
        Some(solution) => {
            if solution.is_empty() {
                println!("(Already solved)");
            } else {
                for step in &solution {
                    println!("Press at row {}, column {}", step.0 + 1, step.1 + 1);
                    act(&mut puzzle.grid, step.0, step.1);
                    println!("{}\n", puzzle);
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

pub fn char_to_square(c: char) -> Square {
    match c.to_ascii_uppercase() {
        'Y' => Square::Yellow,
        'V' => Square::Violet,
        'B' => Square::Black,
        'R' => Square::Red,
        'P' => Square::Pink,
        'G' => Square::Green,
        'O' => Square::Orange,
        'U' => Square::Blue,
        'W' => Square::White,
        '.' => Square::Neutral,
        _ => panic!("Invalid character for square: {:x?}", c),
    }
}

pub fn generate_legend() -> String {
    let mut out = String::new();
    out.push_str("Y: Yellow\n");
    out.push_str("V: Violet\n");
    out.push_str("B: Black\n");
    out.push_str("R: Red\n");
    out.push_str("P: Pink\n");
    out.push_str("G: Green\n");
    out.push_str("O: Orange\n");
    out.push_str("U: Blue\n");
    out.push_str("W: White\n");
    out.push_str(".: Neutral (no color)\n");
    out
}