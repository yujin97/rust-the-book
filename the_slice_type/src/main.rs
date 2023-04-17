fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();

    println!("size of {}'s first word: {}", s, word);

    // string slices
    let s1 = String::from("hello world");

    let hello = &s1[0..5];
    let world = &s1[6..11];

    println!("{} {}", hello, world);

    let s2 = String::from("immutable string");

    let immutable_word = enhanced_first_word(&s2);

    println!("the {}", immutable_word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn enhanced_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
