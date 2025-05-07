fn counting_valleys(path: &str) -> i32 {
    let mut count: i32 = 0;
    let mut current: i32 = 0;

    for c in path.chars() {
        match c {
            'U' => current += 1,
            'D' => {
                match current - 1 {
                    0 => count += 1, // Вихід з долини
                    _ => {}
                }
                current -= 1;
            }
            _ => continue,
        }
    }

    count
}
