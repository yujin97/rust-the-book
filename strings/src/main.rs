fn main() {
    let data = "initial contents";

    let s = data.to_string();

    let mut s1 = String::from("initial commit");

    s1.push_str(" is this commit");
    s1.push('!');

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let _s3 = format!("{tic}-{tac}-{toe}");
    let hello = "Здравствуйте";
    let _hello_sub = &hello[0..4];

    // lterating over strings
    for c in hello.chars() {
        println!("{c}");
    }

    for b in hello.bytes() {
        println!("{b}");
    }
}
