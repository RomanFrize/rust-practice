use rand::Rng;

/// Рахує мінімальну кількість переміщень вантажу між кораблями.
/// Якщо неможливо вирівняти вантаж — повертає -1.
fn count_permutation(shipments: &Vec<u32>) -> isize {
    let n = shipments.len();
    let total: u32 = shipments.iter().sum();

    if total as usize % n != 0 {
        return -1;
    }

    let avg = total / n as u32;
    let mut balance = 0;
    let mut moves = 0;

    for &x in shipments {
        let diff = x as isize - avg as isize;
        balance += diff;
        moves += diff.abs();
    }

    moves / 2
}

/// Генерує вектор довжиною `n` зі значеннями, які в сумі дають середнє.
fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let avg = rng.gen_range(10..=50);
    let mut vec = vec![avg; n];

    for _ in 0..(n / 2) {
        let i = rng.gen_range(0..n);
        let j = rng.gen_range(0..n);
        let shift = rng.gen_range(0..=avg.min(10));
        if vec[i] >= shift {
            vec[i] -= shift;
            vec[j] += shift;
        }
    }

    vec
}

/// Приклади тестів
fn main() {
    let examples = vec![
        vec![8, 2, 2, 4, 4],     // min moves = 4
        vec![9, 3, 7, 2, 9],     // min moves = 7
        vec![1, 1, 1, 1],        // min moves = 0
        vec![5, 5, 5, 5, 5],     // min moves = 0
        vec![10, 10, 20],        // min moves = 5
        vec![1, 2, 3],
        vec![1, 2, 10, 8, 1, 2],
    ];

    for (i, v) in examples.iter().enumerate() {
        println!("Example {}: {:?} => {}", i + 1, v, count_permutation(v));
    }

    // Генерація випадкових валідних даних
    let random = gen_shipments(10);
    println!("\nGenerated valid shipments: {:?}", random);
    println!("Moves needed: {}", count_permutation(&random));
}
