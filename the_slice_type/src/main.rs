// main function
fn main() {
    // create a slice of i32
    let slice: &[i32] = &[1, 2, 3];
    // print the slice
    println!("slice: {:?}", slice);
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
