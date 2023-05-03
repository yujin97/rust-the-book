fn main() {
    let v: Vec<i32> = Vec::new();

    let mut v1 = Vec::new();

    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    let v2 = vec![1, 2, 3, 4, 5];
    let third = &v2[2];
    println!("The third element is {third}");
    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }
    let v3 = vec![100, 32, 57];
    for i in &v3 {
        println!("{i}");
    }
    let mut v4 = vec![100, 32, 57];
    for i in &mut v4 {
        *i += 50;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
