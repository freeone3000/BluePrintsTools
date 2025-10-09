#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
#[repr(u16)]
pub enum Square { // n, y, v, b, r, p, g, o, u, w
    NEUTRAL, // do nothing
    YELLOW, // move up one (and swap)
    VIOLET, // move down one (and swap)
    BLACK, // rotate row right
    RED, // change all white to black, and all black to red (political!)
    PINK, // rotate clockwise, with wraparound, this as the center point
    GREEN, // swap over centerpoint
    ORANGE, // change to mode of surrounding (excluding grey). if no mode, no change.
    BLUE, // act as if on centerpoint
    WHITE, // "lights out" - toggle self and adjacent white to gray, adjacent grey to white.
}

pub type PuzzleGrid = [[Square;3];3];

#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub struct PuzzleBox {
    pub target: [Square;4], // ul, ur, ll, lr
    pub grid: PuzzleGrid, // row, col
}

pub fn is_solved(p: &PuzzleBox) -> bool {
    p.target[0] == p.grid[0][0] &&
    p.target[1] == p.grid[0][2] &&
    p.target[2] == p.grid[2][0] &&
    p.target[3] == p.grid[2][2]
}

pub fn act(p: &mut PuzzleGrid, r: usize, c: usize) {
    // if blue, act with color in center, but with current position
    let act = if p[r][c] == Square::BLUE {
        p[1][1]
    } else {
        p[r][c]
    };

    match act {
        Square::NEUTRAL => {}, // no-op
        Square::BLUE => {}, // blue with blue in the centerpoint does nothing.
        Square::YELLOW => {
            // advance vertically by one if possible
            if r > 0 {
                let old = p[r-1][c]; // can't use std::mem::swap due to double mutable borrow
                p[r-1][c] = Square::YELLOW;
                p[r][c] = old;
            }
        },
        Square::VIOLET => {
            // advance vertically by one if possible
            if r < 2 {
                let old = p[r+1][c]; // can't use std::mem::swap due to double mutable borrow
                p[r+1][c] = Square::VIOLET;
                p[r][c] = old;
            }
        },
        Square::BLACK => {
            // rotate row right
            let old = p[r][2];
            p[r][2] = p[r][1];
            p[r][1] = p[r][0];
            p[r][0] = old;
        },
        Square::RED => {
            // change all white to black, and all black to red (political!)
            for row in 0..3 {
                for col in 0..3 {
                    p[row][col] = match p[row][col] {
                        Square::WHITE => Square::BLACK,
                        Square::BLACK => Square::RED,
                        other => other,
                    };
                }
            }
        }
        Square::PINK => {
            // rotate surrounding squares clockwise, with wraparound, this as the center point
            // Clockwise order: TL, T, TR, R, BR, B, BL, L
            let directions = [
                (-1, -1), // TL
                (-1,  0), // T
                (-1,  1), // TR
                ( 0,  1), // R
                ( 1,  1), // BR
                ( 1,  0), // B
                ( 1, -1), // BL
                ( 0, -1), // L
            ];
            // Collect valid positions
            let mut positions = vec![];
            for (dr, dc) in directions.iter() {
                let nr = r as isize + dr;
                let nc = c as isize + dc;
                if nr >= 0 && nr < 3 && nc >= 0 && nc < 3 {
                    positions.push((nr as usize, nc as usize));
                }
            }
            // Collect values
            let mut values: Vec<Square> = positions.iter().map(|&(nr, nc)| p[nr][nc]).collect();
            // Rotate clockwise: each position gets value from previous position (counterclockwise neighbor)
            if !values.is_empty() {
                values.rotate_right(1);
                for ((nr, nc), val) in positions.iter().zip(values.iter()) {
                    p[*nr][*nc] = *val;
                }
            }
        }
        Square::GREEN => {
            // swap over centerpoint
            let (r2, c2) = (2 - r, 2 - c);
            let old = p[r2][c2]; // can't use std::mem::swap due to double mutable borrow
            p[r2][c2] = p[r][c];
            p[r][c] = old;
        },
        Square::ORANGE => {
            // change to mode of surrounding (excluding grey). if no mode, no change.
            use std::collections::HashMap;
            let mut counts: HashMap<Square, usize> = HashMap::new();
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr == 0 && dc == 0 {
                        continue; // skip self
                    }
                    let (nr, nc) = (r as isize + dr, c as isize + dc);
                    if nr >= 0 && nr < 3 && nc >= 0 && nc < 3 {
                        let neighbor = p[nr as usize][nc as usize];
                        if neighbor != Square::NEUTRAL {
                            *counts.entry(neighbor).or_insert(0) += 1;
                        }
                    }
                }
            }
            // if there exists a largest mode,
            if let Some((&mode, max)) = counts.iter().max_by_key(|&(_, count)| count) {
                // and it is unique,
                if counts.iter().filter(|(_, count)| *count == max).count() == 1 {
                    p[r][c] = mode; // set to that mode
                }
            }
        }
        Square::WHITE => {
            // "lights out" - toggle self and adjacent white to gray, adjacent grey to white.
            let mut to_toggle = vec![(r, c)];
            for (dr, dc) in [(-1,0), (1,0), (0,-1), (0,1)] {
                let (nr, nc) = (r as isize + dr, c as isize + dc);
                if nr >= 0 && nr < 3 && nc >= 0 && nc < 3 {
                    to_toggle.push((nr as usize, nc as usize));
                }
            }
            for (tr, tc) in to_toggle {
                p[tr][tc] = match p[tr][tc] {
                    Square::WHITE => Square::NEUTRAL,
                    Square::NEUTRAL => Square::WHITE,
                    other => other,
                };
            }
        }
    }
}

pub fn possible_actions(p: &PuzzleBox) -> Vec<(usize, usize)> {
    let mut out = vec![];
    for r in 0..3 {
        for c in 0..3 {
            let does_nothing =
                    (p.grid[r][c] == Square::NEUTRAL) || // always does nothing
                    (p.grid[r][c] == Square::BLUE && (p.grid[1][1] == Square::NEUTRAL || p.grid[1][1] == Square::BLUE)) || // blue with grey or blue in center does nothing
                    (r == 0 && p.grid[r][c] == Square::YELLOW) || // yellow at top does nothing
                    (r == 2 && p.grid[r][c] == Square::VIOLET) || // violet at bottom does nothing
                    (r == 1 && c == 1 && p.grid[r][c] == Square::GREEN); // green at center does nothing;

            if !does_nothing {
                out.push((r, c));
            }
        }
    }
    out
}

#[cfg(test)]
mod test_solved {
    use super::*;

    #[test]
    fn test_is_solved() {
        let solved_box = PuzzleBox { target: [Square::RED; 4], grid: [[Square::RED;3];3],}; // trivially solved
        assert!(is_solved(&solved_box));
    }

    #[test]
    fn test_is_not_solved() {
        let unsolved_box = PuzzleBox { target: [Square::BLUE; 4], grid: [[Square::RED;3];3],}; // trivially unsolved
        assert!(!is_solved(&unsolved_box));
    }

    #[test]
    fn test_is_solved_heterogeneous() {
        let solved_box = PuzzleBox { target: [Square::YELLOW, Square::VIOLET, Square::BLACK, Square::RED], grid: [
            [Square::YELLOW, Square::NEUTRAL, Square::VIOLET],
            [Square::NEUTRAL, Square::NEUTRAL, Square::NEUTRAL],
            [Square::BLACK, Square::NEUTRAL, Square::RED],
        ],};
        assert!(is_solved(&solved_box));
    }
}

#[cfg(test)]
mod test_act {
    use super::*;

    fn format_grid(grid: &PuzzleGrid) -> String {
        let mut out = String::new();
        for row in grid {
            out += "|";
            for square in row {
               out += format!("{}|", square).as_str();
            }
            out += "\n";
        }
        out
    }

    #[test]
    fn test_act_gray() {
        let mut test_grid = [[Square::NEUTRAL;3];3];
        let target_grid = test_grid.clone();

        act(&mut test_grid, 1, 1);
        // Check that the grid has not changed
        assert_eq!(test_grid, target_grid, "Acting on gray square should leave all squares unchanged");
    }

    #[test]
    fn test_act_yellow() {
        let mut test_grid = [[Square::NEUTRAL;3];3];
        let mut target_grid = test_grid.clone();

        test_grid[1][0] = Square::YELLOW; // allow to advance
        target_grid[0][0] = Square::YELLOW; // set as final


        act(&mut test_grid, 1, 0);
        // Check that the top-left square has changed to yellow and that none others have changed
        assert_eq!(test_grid, target_grid, "Acting on yellow square should advance vertically by one and leave all other squares unchanged");
    }

    #[test]
    fn test_act_purple() {
        let mut test_grid = [[Square::NEUTRAL;3];3];
        let mut target_grid = test_grid.clone();

        test_grid[1][2] = Square::VIOLET; // allow to advance
        target_grid[2][2] = Square::VIOLET; // set as final

        act(&mut test_grid, 1, 2);
        // Check that the bottom-right square has changed to purple and that none others have changed
        assert_eq!(test_grid, target_grid, "Acting on purple square should advance vertically by one and leave all other squares unchanged");
    }

    #[test]
    fn test_act_black() {
        let mut test_grid = [[Square::NEUTRAL;3];3];
        let mut target_grid = test_grid.clone();

        test_grid[1] = [Square::BLACK, Square::YELLOW, Square::VIOLET]; // set row to known state
        target_grid[1] = [Square::VIOLET, Square::BLACK, Square::YELLOW]; // set as final

        act(&mut test_grid, 1, 0);
        // Check that the middle row has been rotated right and that none others have changed
        assert_eq!(test_grid, target_grid, "Acting on black square should rotate row right and leave all other squares unchanged");
    }

    #[test]
    fn test_act_red() {
        let mut test_grid = [[Square::NEUTRAL;3];3];
        test_grid[0][2] = Square::RED;
        test_grid[2][0] = Square::VIOLET;
        test_grid[2][1] = Square::YELLOW;
        let mut target_grid = test_grid.clone();

        test_grid[0][0] = Square::WHITE;
        test_grid[0][1] = Square::BLACK;

        target_grid[0][0] = Square::BLACK; // white to black
        target_grid[0][1] = Square::RED; // black to red
        // other squares unchanged

        act(&mut test_grid, 0, 2);
        // Check that the color changes have occurred and that none others have changed
        assert_eq!(test_grid, target_grid, "Acting on red square should change all white to black, all black to red, and leave all other squares unchanged");
    }

    #[test]
    fn test_act_green() {
        let mut test_grid = [[Square::NEUTRAL;3];3];
        let mut target_grid = test_grid.clone();
        test_grid[0][0] = Square::GREEN;
        test_grid[2][2] = Square::YELLOW;
        target_grid[0][0] = Square::YELLOW;
        target_grid[2][2] = Square::GREEN;

        act(&mut test_grid, 0, 0);
        // Check that the swap over centerpoint has occurred and that none others have changed
        assert_eq!(test_grid, target_grid, "Acting on green square should swap over centerpoint and leave all other squares unchanged");
    }

    #[test]
    fn test_act_orange_corner() {
        let mut test_grid = [[Square::NEUTRAL;3];3];
        // test corner
        test_grid[1][0] = Square::YELLOW;
        test_grid[0][1] = Square::YELLOW;
        let mut target_grid = test_grid.clone();
        test_grid[0][0] = Square::ORANGE;
        target_grid[0][0] = Square::YELLOW; // mode of surrounding is yellow

        act(&mut test_grid, 0, 0);
        // Check that the color change has occurred and that none others have changed
        assert_eq!(test_grid, target_grid, "Acting on orange square should change to mode of surrounding (excluding grey) and leave all other squares unchanged");
    }

    #[test]
    fn test_act_orange_edge() {
        let mut test_grid = [[Square::NEUTRAL;3];3];
        // test edge
        test_grid[0][0] = Square::YELLOW;
        test_grid[0][2] = Square::VIOLET;
        test_grid[1][1] = Square::YELLOW;
        let mut target_grid = test_grid.clone();
        test_grid[0][1] = Square::ORANGE;
        target_grid[0][1] = Square::YELLOW; // mode of surrounding is yellow

        act(&mut test_grid, 0, 1);
        // Check that the color change has occurred and that none others have changed
        assert_eq!(test_grid, target_grid, "Acting on orange square should change to mode of surrounding (excluding grey) and leave all other squares unchanged");
    }

    #[test]
    fn test_act_orange_center() {
        let mut test_grid = [[Square::NEUTRAL;3];3];
        // test center
        test_grid[0][0] = Square::YELLOW;
        test_grid[0][1] = Square::VIOLET;
        test_grid[0][2] = Square::VIOLET;
        test_grid[1][0] = Square::YELLOW;
        test_grid[1][2] = Square::YELLOW;
        test_grid[2][0] = Square::VIOLET;
        test_grid[2][1] = Square::YELLOW;
        let mut target_grid = test_grid.clone();
        test_grid[1][1] = Square::ORANGE;
        target_grid[1][1] = Square::YELLOW; // mode of surrounding is yellow

        act(&mut test_grid, 1, 1);
        // Check that the color change has occurred and that none others have changed
        assert_eq!(test_grid, target_grid, "Acting on orange square should change to mode of surrounding (excluding grey) and leave all other squares unchanged");
    }

    #[test]
    fn test_act_orange_no_change() {
        let mut test_grid = [[Square::NEUTRAL;3];3];
        // test no mode
        test_grid[0][0] = Square::ORANGE;
        test_grid[0][1] = Square::VIOLET;
        test_grid[1][1] = Square::YELLOW;
        let target_grid = test_grid.clone();
        // target grid is unchanged

        act(&mut test_grid, 0, 0);
        // Check that the grid has not changed
        assert_eq!(test_grid, target_grid, "Acting on orange square with no mode in surrounding squares should leave all squares unchanged");
    }

    #[test]
    fn test_orange_grey_unchanged() {
        let mut test_grid = [[Square::NEUTRAL;3];3];
        // test grey unchanged
        test_grid[0][0] = Square::ORANGE;
        let target_grid = test_grid.clone();

        act(&mut test_grid, 0, 0);
        // Check that the color change has occurred and that none others have changed
        assert_eq!(test_grid, target_grid, "Grey should be excluded from mode calculations; cannot change to grey");
    }

    #[test]
    fn test_act_pink_corner() {
        let mut test_grid = [[Square::NEUTRAL;3];3];
        let mut target_grid = test_grid.clone();

        test_grid[0][0] = Square::PINK;
        test_grid[0][1] = Square::YELLOW;
        test_grid[1][1] = Square::BLACK;
        test_grid[1][0] = Square::VIOLET;

        target_grid[0][0] = Square::PINK;
        target_grid[0][1] = Square::VIOLET;
        target_grid[1][1] = Square::YELLOW;
        target_grid[1][0] = Square::BLACK;

        act(&mut test_grid, 0, 0);
        // Check that the color changes have occurred and that none others have changed
        assert_eq!(test_grid, target_grid, "Acting on pink square should rotate surrounding squares clockwise and leave all other squares unchanged");
    }

    #[test]
    fn test_act_pink_edge() {
        let mut test_grid = [[Square::NEUTRAL;3];3];
        let mut target_grid = test_grid.clone();

        test_grid[0][1] = Square::PINK;
        test_grid[0][0] = Square::YELLOW;
        test_grid[0][2] = Square::VIOLET;
        test_grid[1][2] = Square::BLACK;
        test_grid[1][1] = Square::RED;
        test_grid[1][0] = Square::GREEN;

        target_grid[0][1] = Square::PINK;
        target_grid[0][0] = Square::GREEN;
        target_grid[0][2] = Square::YELLOW;
        target_grid[1][2] = Square::VIOLET;
        target_grid[1][1] = Square::BLACK;
        target_grid[1][0] = Square::RED;

        println!("Test:\n{}", format_grid(&test_grid));
        println!("Target:\n{}", format_grid(&target_grid));

        act(&mut test_grid, 0, 1);
        println!("Result:\n{}", format_grid(&test_grid));

        // Check that the color changes have occurred and that none others have changed
        assert_eq!(test_grid, target_grid, "Acting on pink square should rotate surrounding squares clockwise and leave all other squares unchanged");
    }

    #[test]
    fn test_act_pink_center() {
        let mut test_grid = [
            [Square::NEUTRAL, Square::YELLOW, Square::BLACK],
            [Square::RED, Square::PINK, Square::BLUE],
            [Square::GREEN, Square::NEUTRAL, Square::WHITE]
        ];
        let target_grid = [
            [Square::RED, Square::NEUTRAL, Square::YELLOW],
            [Square::GREEN, Square::PINK, Square::BLACK],
            [Square::NEUTRAL, Square::WHITE, Square::BLUE]
        ];

        act(&mut test_grid, 1, 1);
        assert_eq!(test_grid, target_grid, "Acting on pink square should rotate surrounding squares clockwise and leave all other squares unchanged");
    }

    #[test]
    fn test_act_white() {
        let mut test_grid = [
            [Square::NEUTRAL, Square::WHITE, Square::PINK],
            [Square::NEUTRAL, Square::WHITE, Square::WHITE],
            [Square::NEUTRAL, Square::PINK, Square::WHITE],
        ];
        let target_grid = [
            [Square::NEUTRAL, Square::NEUTRAL, Square::PINK],
            [Square::WHITE, Square::NEUTRAL, Square::NEUTRAL],
            [Square::NEUTRAL, Square::PINK, Square::WHITE],
        ];

        act(&mut test_grid, 1, 1);
        // Check that the color changes have occurred and that none others have changed
        assert_eq!(test_grid, target_grid, "Acting on white square should toggle self and adjacent white to gray, adjacent grey to white, and leave all other squares unchanged");
    }
}

#[cfg(test)]
mod test_enumerate {
    use super::*;

    #[test]
    fn test_possible_actions() {
        let puzzle = PuzzleBox {
            target: [Square::NEUTRAL;4],
            grid: [
                [Square::NEUTRAL, Square::YELLOW, Square::NEUTRAL],
                [Square::NEUTRAL, Square::NEUTRAL, Square::BLUE],
                [Square::NEUTRAL, Square::NEUTRAL, Square::GREEN],
            ],
        };

        let actions = possible_actions(&puzzle);
        let expected_actions = vec![(2, 2)];
        assert_eq!(actions, expected_actions, "Only possible no-op is green here");
    }
}