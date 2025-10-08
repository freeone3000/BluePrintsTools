#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Square {
    YELLOW,
    GRAY,
    BLACK,
    RED,
    PINK,
    GREEN,
    ORANGE,
    PURPLE,
    BLUE,
    WHITE,
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
    match p[r][c] {
        Square::GRAY => {}, // no-op
        Square::YELLOW => {
            // advance vertically by one if possible
            if r > 0 {
                let old = p[r-1][c]; // can't use std::mem::swap due to double mutable borrow
                p[r-1][c] = Square::YELLOW;
                p[r][c] = old;
            }
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
}