#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Square {
    GRAY, // do nothing
    YELLOW, // move up one (and swap)
    PURPLE, // move down one (and swap)
    BLACK, // rotate row right
    RED, // change all white to black, and all black to red (political!)
    PINK, // rotate clockwise, with wraparound, this as the center point
    GREEN, // swap over centerpoint
    ORANGE, // change to mode of surrounding (excluding grey). if no mode, no change.
    BLUE, // act as if on centerpoint
    WHITE, // "lights out" - toggle self and adjacent white to gray, adjacent grey to white.
}

type PuzzleGrid = [[Square;3];3];

#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub struct PuzzleBox {
    target: [Square;4], // ul, ur, ll, lr
    grid: PuzzleGrid, // row, col
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
        p[2][2]
    } else {
        p[r][c]
    };

    match act {
        Square::GRAY => {}, // no-op
        Square::BLUE => {}, // blue with blue in the centerpoint does nothing.
        Square::YELLOW => {
            // advance vertically by one if possible
            if r > 0 {
                let old = p[r-1][c]; // can't use std::mem::swap due to double mutable borrow
                p[r-1][c] = Square::YELLOW;
                p[r][c] = old;
            }
        },
        Square::PURPLE => {
            // advance vertically by one if possible
            if r < 2 {
                let old = p[r+1][c]; // can't use std::mem::swap due to double mutable borrow
                p[r+1][c] = Square::PURPLE;
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
            // TODO actually math this out, LLM is proving a bit of an issue here
        }
        Square::GREEN => {
            // swap over centerpoint
            let (r2, c2) = (2 - r, 2 - c);
            let old = p[r2][c2]; // can't use std::mem::swap due to double mutable borrow
            p[r2][c2] = p[r][c];
            p[r][c] = old;
        },
        _ => panic!("square not implemented {:?}", p[r][c]),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_solved() {
        let solved_box = PuzzleBox { target: [Square::RED; 4], grid: [[Square::RED;3];3],}; // trivially solved
        assert!(is_solved(&solved_box));
    }

    #[test]
    fn test_act_gray() {
        let mut test_grid = [[Square::GRAY;3];3];
        let target_grid = test_grid.clone();

        act(&mut test_grid, 1, 1);
        // Check that the grid has not changed
        assert_eq!(test_grid, target_grid, "Acting on gray square should leave all squares unchanged");
    }

    #[test]
    fn test_act_yellow() {
        let mut test_grid = [[Square::GRAY;3];3];
        let mut target_grid = test_grid.clone();

        test_grid[1][0] = Square::YELLOW; // allow to advance
        target_grid[0][0] = Square::YELLOW; // set as final


        act(&mut test_grid, 1, 0);
        // Check that the top-left square has changed to yellow and that none others have changed
        assert_eq!(test_grid, target_grid, "Acting on yellow square should advance vertically by one and leave all other squares unchanged");
    }

    #[test]
    fn test_act_purple() {
        let mut test_grid = [[Square::GRAY;3];3];
        let mut target_grid = test_grid.clone();

        test_grid[1][2] = Square::PURPLE; // allow to advance
        target_grid[2][2] = Square::PURPLE; // set as final

        act(&mut test_grid, 1, 2);
        // Check that the bottom-right square has changed to purple and that none others have changed
        assert_eq!(test_grid, target_grid, "Acting on purple square should advance vertically by one and leave all other squares unchanged");
    }

    #[test]
    fn test_act_black() {
        let mut test_grid = [[Square::GRAY;3];3];
        let mut target_grid = test_grid.clone();

        test_grid[1] = [Square::BLACK, Square::YELLOW, Square::PURPLE]; // set row to known state
        target_grid[1] = [Square::PURPLE, Square::BLACK, Square::YELLOW]; // set as final

        act(&mut test_grid, 1, 0);
        // Check that the middle row has been rotated right and that none others have changed
        assert_eq!(test_grid, target_grid, "Acting on black square should rotate row right and leave all other squares unchanged");
    }

    #[test]
    fn test_act_red() {
        let mut test_grid = [[Square::GRAY;3];3];
        test_grid[0][2] = Square::RED;
        test_grid[2][0] = Square::PURPLE;
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
        let mut test_grid = [[Square::GRAY;3];3];
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
        let mut test_grid = [[Square::GRAY;3];3];
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
        let mut test_grid = [[Square::GRAY;3];3];
        // test edge
        test_grid[0][0] = Square::YELLOW;
        test_grid[0][2] = Square::PURPLE;
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
        let mut test_grid = [[Square::GRAY;3];3];
        // test center
        test_grid[0][0] = Square::YELLOW;
        test_grid[0][1] = Square::PURPLE;
        test_grid[0][2] = Square::PURPLE;
        test_grid[1][0] = Square::YELLOW;
        test_grid[1][2] = Square::YELLOW;
        test_grid[2][0] = Square::PURPLE;
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
        let mut test_grid = [[Square::GRAY;3];3];
        // test no mode
        test_grid[0][0] = Square::ORANGE;
        test_grid[0][1] = Square::PURPLE;
        test_grid[1][1] = Square::YELLOW;
        let target_grid = test_grid.clone();
        // target grid is unchanged

        act(&mut test_grid, 1, 1);
        // Check that the grid has not changed
        assert_eq!(test_grid, target_grid, "Acting on orange square with no mode in surrounding squares should leave all squares unchanged");
    }

    #[test]
    fn test_orange_grey_unchanged() {
        let mut test_grid = [[Square::GRAY;3];3];
        // test grey unchanged
        test_grid[0][0] = Square::ORANGE;
        let target_grid = test_grid.clone();

        act(&mut test_grid, 0, 0);
        // Check that the color change has occurred and that none others have changed
        assert_eq!(test_grid, target_grid, "Grey should be excluded from mode calculations; cannot change to grey");
    }

    #[test]
    fn test_act_pink() {
        // TODO NOT IMPLEMENTED
    }

    #[test]
    fn test_act_white() {
        // TODO NOT IMPLEMENTED
    }
}