// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = String::from("hello");
    let y: Option<&String> = Some(&a);

    match y {
        Some(ref p) => println!("string is: {}", p),
        _ => panic!("no match!"),
    }
    //    y; // Fix without deleting this line.
}
