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
    fn giveaway(&self, user_preferences: Option<ShirtColor>) -> ShirtColor {
        user_preferences.unwrap_or_else(|| self.most_stocked())
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

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

fn main() {
    closure_examples();
    only_borrow_closure();
    mutable_borrow_closure();
    move_to_new_thread();

    sort_rectangles_by_width();
    sort_rectangles_and_count();

    do_store_shit();
}

fn sort_rectangles_by_width() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 6,
        },
        Rectangle {
            width: 7,
            height: 11,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("Sorted rectangle list: {:#?}", list);
}

fn sort_rectangles_and_count() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 6,
        },
        Rectangle {
            width: 7,
            height: 11,
        },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorting in {num_sort_operations} operations", list);
}

fn move_to_new_thread() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

fn closure_examples() {
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    println!("s from example closure is {s}");
    // Doesn't work because above call infers String type
    // let n = example_closure(5);
    let add_one = |x| x + 1;
    let two = add_one(1);
    println!("1 + 1 = {}", two);
}

fn only_borrow_closure() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

fn mutable_borrow_closure() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

fn do_store_shit() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The user with preference {:?} gets a {:?} shirt!",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!(
        "The user with preference {:?} gets a {:?} shirt!",
        user_pref2, giveaway2
    );
}
