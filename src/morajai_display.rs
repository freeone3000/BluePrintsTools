use crate::morajai::*;
use std::fmt::{Display, Formatter};

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
        }
        .to_string();
        write!(f, "{}", s)
    }
}

pub fn format_grid(grid: &PuzzleGrid) -> String {
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

impl Display for PuzzleBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("\\{}\\      /{}/\n", self.target[0], self.target[1]).as_str())?;
        for line in format_grid(&self.grid).split("\n") {
            f.write_str(format!("   {}\n", line).as_str())?;
        }
        f.write_str(format!("/{}/      \\{}\\\n", self.target[2], self.target[3]).as_str())?;
        Ok(())
    }
}

/* helper methods */

pub fn char_to_square(c: char) -> Square {
    match c.to_ascii_uppercase() {
        'Y' => Square::YELLOW,
        'V' => Square::VIOLET,
        'B' => Square::BLACK,
        'R' => Square::RED,
        'P' => Square::PINK,
        'G' => Square::GREEN,
        'O' => Square::ORANGE,
        'U' => Square::BLUE,
        'W' => Square::WHITE,
        '.' => Square::NEUTRAL,
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
