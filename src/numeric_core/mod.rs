// teal is already covered

#[repr(u8)]
#[derive(Copy, Clone)]
enum Op {
    Subtract, // yellow
    Multiply, // mauve
    Divide, // purple
}

const ALL_OPS_PERMUTE: [[Op;3];6] = permute_all_ops();

const fn permute_all_ops() -> [[Op;3];6] {
    use Op::*;
    [
        [Subtract, Multiply, Divide],
        [Subtract, Divide, Multiply],
        [Multiply, Subtract, Divide],
        [Multiply, Divide, Subtract],
        [Divide, Subtract, Multiply],
        [Divide, Multiply, Subtract],
    ]
}

fn eval_op(a: f32, op: Op, b: f32) -> f32 {
    use Op::*;
    match op {
        Subtract => a - b,
        Multiply => a * b,
        Divide => a / b,
    }
}


pub fn calculate_numeric_core(parts: [u16;4]) -> Option<u16> {
    // convert into float
    let work_parts: [f32;4] = [parts[0] as f32, parts[1] as f32, parts[2] as f32, parts[3] as f32];
    // broadcast to output
    let mut out: [f32;6] = [work_parts[0]; 6];
    // iterate over ops in order, and then for each, execute the correct op at the broadcast step
    for op_numer in 0..3 {
        for broadcast in 0..6 {
            out[broadcast] = eval_op(out[broadcast], ALL_OPS_PERMUTE[broadcast][op_numer], work_parts[op_numer+1]);
        }
    }

    // filter out results that are negative or non-integer, convert to u16, then return the minimum (or none)
    out.into_iter()
        .filter(|x| x.fract() == 0.0 && *x >= 0.0)
        .map(|x| x as u16)
        .min()
}

#[cfg(test)]
mod test {
    use super::calculate_numeric_core;

    #[test]
    fn test_calculate_mary_book_1() {
        assert_eq!(calculate_numeric_core([8, 6, 45, 5]), Some(18), "Mary's book worked example");
    }

    #[test]
    fn test_calculate_mary_book_2() {
        // note: the textbook in-game says 3, but we believe it to be *wrong*?
        assert_eq!(calculate_numeric_core([3, 6, 1, 4]), Some(14), "Mary's book given example");
    }
    #[test]
    fn test_calculate_room_8() {
        assert_eq!(calculate_numeric_core([1000, 200, 11, 2]), Some(53), "Room 8 key test");
    }
}