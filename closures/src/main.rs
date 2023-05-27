use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let list = vec![1, 2, 3];
    println!("Before defining closure {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    let mut mut_list = vec![4, 5, 6];
    println!("Before defining closure: {:?}", mut_list);

    let mut borrows_mutably = || mut_list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", mut_list);

    let thread_list = vec![7, 8, 9];
    println!("Before defining closure {:?}", thread_list);

    thread::spawn(move || println!("From thread: {:?}", thread_list))
        .join()
        .unwrap();

    let mut key_list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 9,
        },
        Rectangle {
            width: 4,
            height: 3,
        },
    ];

    let mut num_sort_operations = 0;
    key_list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}
