use std::collections::HashMap;

fn main() {
    let numbers = vec![1, 8, 9, 8, 1, 4, 6, 5, 5, 6, 8, 9, 9, 4];

    let median_and_mode = find_median_and_mode(&numbers);

    println!("{:?}", median_and_mode);
}

fn find_median_and_mode(numbers: &Vec<i32>) -> Vec<f32> {
    let mut ans = Vec::new();
    if numbers.len() == 0 {
        return ans;
    }
    let mut owned_numbers = numbers.clone();
    owned_numbers.sort();
    let mut counts = HashMap::new();
    let median = match numbers.len() % 2 {
        0 => {
            let larger = numbers.len() / 2 - 1;
            let smaller = larger - 1;
            let median = (owned_numbers[larger] + owned_numbers[smaller]) as f32 / 2.0;
            median
        }
        _ => owned_numbers[numbers.len() / 2] as f32,
    };

    ans.push(median);

    for number in &owned_numbers {
        let count = counts.entry(number.to_owned()).or_insert(0);
        *count += 1;
    }

    let mut mode_count = 0;

    for (_, value) in &counts {
        if value > &mode_count {
            mode_count = value.to_owned();
        }
    }

    for (key, value) in counts {
        if value == mode_count {
            ans.push(key.clone() as f32);
        }
    }

    return ans;
}
