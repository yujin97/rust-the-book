use std::{collections::HashMap, string};

fn main() {
    let numbers = vec![1, 8, 9, 8, 1, 4, 6, 5, 5, 6, 8, 9, 9, 4];

    let median_and_mode = find_median_and_mode(&numbers);

    println!("{:?}", median_and_mode);
    println!("pig latin of \"first\": {:?}", pig_latin("first"));
    println!("pig latin of \"apple\": {:?}", pig_latin("apple"));
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

fn pig_latin(input: &str) -> String {
    if input.len() == 0 {
        return "".to_owned();
    }
    let is_vowel = |letter: &char| -> bool {
        letter == &'a' || letter == &'e' || letter == &'i' || letter == &'o' || letter == &'u'
    };
    let mut first_consonent_index = 0;
    for c in input.chars() {
        if is_vowel(&c) {
            first_consonent_index += 1;
        } else {
            break;
        }
    }
    let is_first_char_vowel = is_vowel(&input.chars().nth(0).unwrap());
    let first_part = match is_first_char_vowel {
        true => input.to_owned(),
        false => match first_consonent_index <= input.len() - 1 {
            true => input[..first_consonent_index].to_owned() + &input[first_consonent_index + 1..],
            false => input.to_owned(),
        },
    };
    let result = match is_first_char_vowel {
        true => first_part + "-" + "hay",
        false => first_part + "-" + &input[first_consonent_index..first_consonent_index + 1] + "ay",
    };
    return result;
}
