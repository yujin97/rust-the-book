type Kilometer = i32;

type Thunk = Box<dyn Fn() + Send + 'static>;

fn main() {
    let x: i32 = 5;
    let y: Kilometer = 5;

    let f: Thunk = Box::new(|| println!("hi"));

    println!("x + y = {}", x + y);
    f();
}
