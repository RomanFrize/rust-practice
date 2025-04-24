fn draw_tree(levels: usize) {
    let max_width = levels * 2 + 1;

    (0..levels).for_each(|level| {
        let height = level + 2;
        (0..height).for_each(|line| {
            let stars = 1 + line * 2;
            let padding = (max_width - stars) / 2;
            let line_str = " ".repeat(padding) + &"*".repeat(stars);
            println!("{}", line_str);
        });
    });
}

fn main() {
    let levels = 4; // заміни це значення для іншої кількості трикутників
    draw_tree(levels);
}