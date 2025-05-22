fn gray(n: u8) -> Vec<String> {
    if n == 0 {
        return vec![String::new()];
    }

    let prev = gray(n - 1);
    let mut result = Vec::new();

    // Префіксуємо "0" до попереднього списку
    for code in &prev {
        result.push(format!("0{}", code));
    }

    // Префіксуємо "1" до розгорнутого попереднього списку
    for code in prev.iter().rev() {
        result.push(format!("1{}", code));
    }

    result
}

fn main() {
    let test_data = [
        (0, vec![""]),
        (1, vec!["0", "1"]),
        (2, vec!["00", "01", "11", "10"]),
        (3, vec![
            "000", "001", "011", "010",
            "110", "111", "101", "100"
        ]),
        (4, vec![
            "0000", "0001", "0011", "0010",
            "0110", "01011", "0101", "0100",
            "1100", "1101", "1111", "1110",
            "1010", "1011", "1001", "1000"
        ]),
    ];

    for (n, expected) in test_data.iter() {
        let actual = gray(*n);
        if actual == *expected {
            println!("Тест пройдено в блоці = {} ", n);
        } else {
            println!("Тест провалено в блоці= {} ", n);
            println!("Дійсний : {:?}", expected);
            println!("Очікуваний : {:?}", actual);
        }
    }
}