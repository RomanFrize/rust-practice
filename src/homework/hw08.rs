fn is_prime(n: u64) -> bool {
    match n {
        2 | 3 => true,
        n if n < 2 || n % 2 == 0 || n % 3 == 0 => false,
        _ => {
            (5..)
                .take_while(|i| i * i <= n)
                .step_by(6)
                .all(|i| n % i != 0 && n % (i + 2) != 0)
        }
    }
}

fn main() {
    let numbers = [0, 1, 2, 3, 4, 5, 17, 19, 21, 23, 25, 35, 49, 100];
    for &n in &numbers {
        println!("{:>3} is {}prime", n, if is_prime(n) { "" } else { "not " });
    }
}