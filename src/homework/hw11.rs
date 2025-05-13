use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..=99)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (usize, i32, i32) {
    let mut min_index = 0;
    let mut min_sum = data[0] + data[1];

    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    (min_index, data[min_index], data[min_index + 1])
}

fn print_result(data: &[i32], min_index: usize) {
    // Виводимо індекси
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:>3}.", i);
    }
    println!();

    // Виводимо дані
    print!("data:    ");
    for x in data {
        print!("{:>3},", x);
    }
    println!();

    // Виводимо стрілку
    print!("indexes: ");
    for i in 0..data.len() {
        if i == min_index {
            print!("\\__ ");
        } else if i == min_index + 1 {
            print!("__/ ");
        } else {
            print!("    ");
        }
    }
    println!();
}

fn main() {
    let data = gen_random_vector(20);
    let (min_index, a, b) = min_adjacent_sum(&data);

    print_result(&data, min_index);
    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        a,
        b,
        a + b,
        min_index,
        min_index + 1
    );
}