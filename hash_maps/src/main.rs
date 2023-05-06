fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name).copied().unwrap_or(0);

    scores.insert(String::from("Blue"), 20);

    scores.entry(String::from("Yellow")).or_insert(44);
    scores.entry(String::from("Yellow")).or_insert(99);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
