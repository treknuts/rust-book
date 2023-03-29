fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    println!("Iterating over a simple array");
    for i in v1_iter {
        println!("{i}");
    }
}
