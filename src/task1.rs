#[test]

/*
/* Fill in the blanks */
fn main() {
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob"); // => Alice, this is Bob. Bob, this is Alice
    assert_eq!(format!("{1}{0}", 1, 2), __);
    assert_eq!(format!(__, 1, 2), "2112");
    println!("Success!");
}
*/

fn main() {
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob"); // => Alice, this is Bob. Bob, this is Alice
    assert_eq!(format!("{1}{0}", 1, 2), "21");
    assert_eq!(format!("{0}{1}{0}", 1, 2), "121");
    println!("Success!");
}


/*
Перший пропуск:

assert_eq!(format!("{1}{0}", 1, 2), "21"); - тут вказується, що спочатку виводиться другий аргумент (2),
потім перший (1), тому результат буде "21".
Другий пропуск:

assert_eq!(format!("{0}{1}{0}", 1, 2), "121"); - тут порядок аргументів такий,
що спочатку виводиться перший аргумент (1), потім другий (2), а потім знову перший (1), отже, результат буде "121".
*/