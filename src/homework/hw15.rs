use itertools::Itertools;

fn main() {
    let digits = (1..=8).collect::<Vec<_>>();
    let mut solutions = 0;

    for perm in digits.iter().permutations(7).unique() {
        let m = *perm[0];
        let u = *perm[1];
        let x = *perm[2];
        let a = *perm[3];
        let s = *perm[4];
        let l = *perm[5];
        let o = *perm[6];

        let muha = 1000 * m + 100 * u + 10 * x + a;
        let slon = 1000 * s + 100 * l + 10 * o + (*perm[3]); // остання 'a' = n = a

        if muha * a == slon {
            println!(
                "  {}{}{}{}\n×     {}\n--------\n  {}{}{}{}",
                m, u, x, a, a, s, l, o, a
            );
            solutions += 1;
        }
    }

    println!("\nЗагальна кількість розв’язків: {}", solutions);
}
