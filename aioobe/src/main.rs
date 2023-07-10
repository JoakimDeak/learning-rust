use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("enter array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");

    let index: usize = index.trim().parse().expect("Index was not a number");

    let element = a[index];

    println!("value of element at index: {index} is: {element}");
}
