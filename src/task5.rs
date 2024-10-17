#[test]

/*
fn main() {
    println!("Hello {:5}!", 5); // => Hello     5!
    println!("Hello {:+}!", 5); // =>  Hello +5!
    println!("Hello {:05}!", 5); // => Hello 00005!
    println!("Hello {:05}!", -5); // => Hello -0005!

    /* Fill in the blank */
    assert!(format!("{number:0>width$}", number=1, width=6) == __);

    println!("Success!")
;}
*/

fn main() {
    println!("Hello {:5}!", 5); // => Hello     5!
    println!("Hello {:+}!", 5); // => Hello +5!
    println!("Hello {:05}!", 5); // => Hello 00005!
    println!("Hello {:05}!", -5); // => Hello -0005!

    // Заповнення пробілу
    assert!(format!("{number:0>width$}", number=1, width=6) == "000001");

    println!("Success!");
}


/*
{:5} — вирівнює число в рядку з шириною 5 символів, додаючи пробіли зліва.
{:+} — виводить число з ознакою знака, тобто для позитивних чисел додається знак +.
{:05} — виводить число з фіксованою шириною 5 символів, заповнюючи відсутні місця нулями.
{:0>width$} — заповнює число нулями зліва, щоб загальна ширина рядка була рівною значенню width.
*/