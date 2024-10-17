#[test]

/*
fn get_person() -> String {
    String::from("sunface")
}

fn get_format() -> (usize, usize) {
    (4, 1)
}


fn main() {
    let person = get_person();
    println!("Hello, {person}!");

    let (width, precision) = get_format();
    let scores = [("sunface", 99.12), ("jack", 60.34)];
    /* Make it print:
    sunface: 99.1
    jack: 60.3
    */
    for (name, score) in scores {
        println!("{name}: __");
    }
}
*/

fn main() {
    let person = get_person();
    println!("Hello, {person}!");

    let (_width, _precision) = get_format();
    let scores = [("sunface", 99.12), ("jack", 60.34)];

    /* Виведемо:
    sunface: 99.1
    jack: 60.3
    */
    for (name, score) in scores {
        println!("{name}: {score:.1}");
    }
}

fn get_person() -> String {
    String::from("sunface")
}

fn get_format() -> (usize, usize) {
    (4, 1)
}




/*
У рядку println!("{name}: __"); замість двох підкреслень (__) потрібно вказати формат для score. Вставивши "{score:.1}",
ми отримаємо число з однією цифрою після коми.
*/