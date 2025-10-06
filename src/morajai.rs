#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
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

#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub struct PuzzleBox {
    target: [Square;4], // ul, ur, ll, lr
    grid: [[Square;3];3], // row, col
}

pub fn is_solved(p: &PuzzleBox) -> bool {
    p.target[0] == p.grid[0][0] &&
    p.target[1] == p.grid[0][2] &&
    p.target[2] == p.grid[2][0] &&
    p.target[3] == p.grid[2][2]
}

pub fn act(p: &PuzzleBox, r: usize, c: usize) -> PuzzleBox {
    panic!("Not implemented yet");
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
    fn test_act_yellow() {
        let mut initial_grid = [[Square::GRAY;3];3];
        let mut target_grid = initial_grid.clone();

        initial_grid[1][0] = Square::YELLOW; // allow to advance
        let initial_box = PuzzleBox { target: [Square::YELLOW; 4], grid: initial_grid,};

        target_grid[0][0] = Square::YELLOW; // set as final
        let expected_box = PuzzleBox { target: [Square::YELLOW; 4], grid: target_grid,};

        let new_box = act(&initial_box, 0, 0);
        // Check that the top-left square has changed to yellow and that none others have changed
        assert_eq!(new_box, expected_box, "Acting on yellow square should advance vertically by one and leave all other squares unchanged");
    }
}