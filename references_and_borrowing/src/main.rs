fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of {} is {}.", s1, len);

    let mut s = String::from("hello");
    println!("{}", s);
    change(&mut s);
    println!("{}", s);

    non_race();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", word");
}

fn non_race() {
    // not considered race condition
    let mut s = String::from("not race");

    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);
}
