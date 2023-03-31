use chapter14_2::mix;
use chapter14_2::PrimaryColor;

fn main() {
    let blue = PrimaryColor::Blue;
    let yellow = PrimaryColor::Yellow;
    println!("Blue and yellow makes {:?}", mix(blue, yellow));
}
