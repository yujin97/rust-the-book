use std::collections::HashMap;

fn main() {
    let numbers = vec![7, 8, 9, 1, 5, 5, 5, 7, 8, 9, 9];

    let median_and_mode = find_median_and_mode(&numbers);

    println!("{:?}", median_and_mode);
}

fn find_median_and_mode(numbers: &Vec<i32>) -> Vec<i32> {
    let mut owned_numbers = numbers.clone();
    owned_numbers.sort();
    let mut counts = HashMap::new();
    let median = match numbers.len() % 2 {
        0 => {
            let larger = numbers.len() / 2 - 1;
            let smaller = larger - 1;
            owned_numbers[(smaller + larger) / 2]
        }
        _ => owned_numbers[numbers.len() / 2],
    };

    for number in &owned_numbers {
        let count = counts.entry(number.to_owned()).or_insert(0);
        *count += 1;
    }

    let mut mode = owned_numbers.get(0).unwrap_or(&0).to_owned();
    let mut mode_count = 0;

    for (key, value) in counts {
        if value > mode_count {
            mode_count = value;
            mode = key;
        }
    }

    return vec![median, mode];
}
