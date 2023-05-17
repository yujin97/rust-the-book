fn main() {
    // why does this work...?
    // because lifetime of all string literals is 'static'!!
    // let string2: &'static str = "xyz"
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = "xyz";
        result = longest(string1.as_str(), string2);
    }

    println!("The longest string is {}", result);
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    longest_with_an_announcement("hello", "world", "rust is the best");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
