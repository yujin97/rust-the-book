fn main() {
    let both_integer = Point { x: 4, y: 10 };
    let integer_and_float = Point { x: 4, y: 12.2 };
    let mix = both_integer.mixup(integer_and_float);
    let string_string = Point {
        x: "hello".to_owned(),
        y: "world".to_owned(),
    };
    let integer_and_string = Point {
        x: 14,
        y: "bye".to_owned(),
    };
    let mix_up = string_string.mixup(integer_and_string);
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// not necessary to use same type for generic types
impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
