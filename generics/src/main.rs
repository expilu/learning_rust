fn find_largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn find_largest_generic<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");
    println!("{}", &number_list[0]);

    let number_list = vec![34, 50, 25, 100, 65];

    let result = find_largest(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = find_largest(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![34, 50, 25, 100, 65];

    let result = find_largest_generic(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = find_largest_generic(&char_list);
    println!("The largest char is {result}");

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 5.0, y: 10.0 };
}
