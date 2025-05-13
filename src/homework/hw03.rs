#[test]

use std::env;
/*fdf*/
fn main() {
    const W: u32 = 25;
    const H: u32 = 10;

    for y in 0..H {
        for x in 0..W {
            let is_hor: bool = y == 0 || y == H - 1;  // Верхня та нижня межі
            let is_ver: bool = x == 0 || x == W - 1;  // Ліва та права межі

            // Діагоналі з відступом на 3 одиниці по горизонталі
            let is_diag1: bool = x + 1  == y * 3; // Перша діагональ
            let is_diag2: bool = if y * 3 <= W { x == W - y * 3 } else { false }; // Друга діагональ


            let show: bool = is_hor || is_ver || is_diag1 || is_diag2;

            let sym: &str = if show { "*" } else { " " };
            print!("{}", sym);
        }
        println!();
    }
}
