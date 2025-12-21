fn main() {
    let s = "Hello";
    println!("{s}");

    let s = String::from("hi");
    println!("{s}");

    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{s}");

    let x = 5;
    let y = x;
    println!("{x} and {y}");

    let s1 = String::from("hello");
    // the following fails
    // let s2 = s1;
    // println!("{s1} and {s2}");

    let s = String::from("hello");
    takes_ownership(s);
    // the following fails
    // println!("{s}");

    let x = 5;
    makes_copy(x);
    println!("{x}");

    let s = String::from("how long?");
    let (s, len) = calculate_length(s);
    println!("The length of '{s}' is {len}.");

    let s = String::from("how long?");
    let len = calculate_length_by_reference(&s);
    println!("The length of '{s}' is {len}.");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calculate_length_by_reference(s: &String) -> usize {
    s.len()
}
