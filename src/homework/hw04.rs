fn main() {
    const HEIGHT: usize = 11; // Висота ромба (непарне число)
    const WIDTH: usize = 11;  // Ширина ромба (непарне число)

    let mut output = String::new(); // Змінна для збереження всього ромба

    for y in 0..HEIGHT {
        let stars = if y <= HEIGHT / 2 {
            1 + y * 2  // Верхня половина ромба
        } else {
            1 + (HEIGHT - y - 1) * 2 // Нижня половина ромба
        };

        let spaces = (WIDTH - stars) / 2; // Кількість пробілів перед зірочками

        output.push_str(&" ".repeat(spaces)); // Додаємо пробіли
        output.push_str(&"*".repeat(stars)); // Додаємо зірочки
        output.push('\n'); // Новий рядок
    }

    print!("{}", output); // Виводимо весь ромб одним викликом print!
}
