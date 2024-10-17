#[test]

/*
fn main() {
    // The following two are padding with 5 spaces
    println!("Hello {:5}!", "x"); // =>  "Hello x    !"
    println!("Hello {:1$}!", "x", 5); // =>  "Hello x    !"

    /* Fill in the blanks */
    assert_eq!(format!("Hello __!", 5, "x"), "Hello x    !");
    assert_eq!(format!("Hello __!", "x", width = 5), "Hello x    !");

    println!("Success!");
}
*/

fn main() {
    // The following two are padding with 5 spaces
    println!("Hello {:5}!", "x"); // =>  "Hello x    !"
    println!("Hello {:1$}!", "x", 5); // =>  "Hello x    !"

    /* Fill in the blanks */
    assert_eq!(format!("Hello {:width$}!", "x", width = 5), "Hello x    !");
    assert_eq!(format!("Hello {:5}!", "x"), "Hello x    !");

    println!("Success!");
}


/*
Перший пропуск:

assert_eq!(format!("Hello {:width$}!", "x", width = 5), "Hello x !"); - тут використовується іменоване форматування з width, яке задає ширину поля для "x".

Другий пропуск:

assert_eq!(format!("Hello {:5}!", "x"), "Hello x !"); - тут просто використовується стандартне форматування з фіксованою шириною.
*/