use std::fmt::Display;
use crate::morajai::{Square, PuzzleGrid};

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Square::NEUTRAL => '.',
            Square::YELLOW => 'Y',
            Square::VIOLET => 'V',
            Square::BLACK => 'B',
            Square::RED => 'R',
            Square::PINK => 'P',
            Square::GREEN => 'G',
            Square::ORANGE => 'O',
            Square::BLUE => 'U',
            Square::WHITE => 'W',
        }.to_string();
        write!(f, "{}", s)
    }
}