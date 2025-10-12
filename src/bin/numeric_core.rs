use blue_prince::numeric_core::calculate_numeric_core;

fn transpose_to_numbers(s: &str) -> [u16; 4] {
    let mut out = [0u16; 4];
    for (i, c) in s.to_ascii_uppercase().chars().enumerate().take(4) {
        out[i] = (c as u8 - b'A' + 1) as u16;
    }
    out
}

fn transpose_to_letters(s: u16) -> char {
    ((s as u8) + ('A' as u8) - 1) as char
}

fn main() {
    println!("Please enter a sequence of letters. Separate each word with spaces. Two newlines ends input.");

    let mut input: String = String::new();
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("Failed to read line");
        if line.trim().is_empty() {
            break;
        }
        input.push_str(&line);
    }
    input.split_ascii_whitespace()
        .map(|x| transpose_to_numbers(x))
        .map(|x| calculate_numeric_core(x))
        .map(|x| x.map_or("%!".to_string(), |y| transpose_to_letters(y).to_string()))
        .for_each(|x| print!("{} ", x));
}

#[cfg(test)]
mod test {
    use super::transpose_to_numbers;
    use super::transpose_to_letters;

    #[test]
    fn test_transpose_to_numbers() {
        assert_eq!(transpose_to_numbers("ABCD"), [1, 2, 3, 4]);
        assert_eq!(transpose_to_numbers("WXYZ"), [23, 24, 25, 26]);
        assert_eq!(transpose_to_numbers("abcd"), [1, 2, 3, 4]);
        assert_eq!(transpose_to_numbers("wxyz"), [23, 24, 25, 26]);
    }

    #[test]
    fn test_transpose_to_letters() {
        assert_eq!(transpose_to_letters(1), 'A');
        assert_eq!(transpose_to_letters(26), 'Z');
        assert_eq!(transpose_to_letters(13), 'M');
    }
}