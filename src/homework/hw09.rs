/// Повертає новий рядок, у якому символи зсунуті на `n` позицій вправо.
fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s;
    }

    let shift = ((n % len) + len) % len;
    let split_index = (len - shift) as usize;

    let chars: Vec<char> = s.chars().collect();
    let rotated: String = chars[split_index..]
        .iter()
        .chain(chars[..split_index].iter())
        .collect();

    rotated
}

fn main() {
    let s = "abcdefgh".to_string();
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-3, "cdefghab"),
        (-10,"cdefghab"),
    ];

    for (n, expected) in shifts.iter() {
        let result = rotate(s.clone(), *n);
        let status = if result == *expected {
            "OK"
        } else {
            "False"
        };
        println!(
            "shift: {:>3} | result: {} | expected: {} | {}",
            n, result, expected, status
        );
    }
}
