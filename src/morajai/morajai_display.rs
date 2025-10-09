use crate::morajai::*;
use std::fmt::{Display, Formatter};

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Square::Neutral => '.',
            Square::Yellow => 'Y',
            Square::Violet => 'V',
            Square::Black => 'B',
            Square::Red => 'R',
            Square::Pink => 'P',
            Square::Green => 'G',
            Square::Orange => 'O',
            Square::Blue => 'U',
            Square::White => 'W',
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