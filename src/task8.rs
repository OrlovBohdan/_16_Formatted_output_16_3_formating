#[test]

/*
fn main() {
    assert_eq!(format!("__", 27), "0b11011");
    assert_eq!(format!("__", 27), "0o33");
    assert_eq!(format!("__", 27), "0x1b");
    assert_eq!(format!("__", 27), "0x1B");

    println!("{:x}!", 27); // Hex with no prefix => 1b

    println!("{:#010b}", 27); // Pad binary with 0, width = 10,  => 0b00011011

    println!("Success!");
}
*/

fn main() {
    // Перевіряємо, чи правильні рядки в форматі двійкового, восьмкового та шістнадцяткового форматів
    assert_eq!(format!("{:b}", 27), "11011"); // Двійковий формат, без префікса
    assert_eq!(format!("{:o}", 27), "33");    // Восьмковий формат
    assert_eq!(format!("{:x}", 27), "1b");    // Шістнадцятковий формат (малими літерами)
    assert_eq!(format!("{:X}", 27), "1B");    // Шістнадцятковий формат (великими літерами)

    // Виводимо шістнадцяткове представлення числа 27 без префікса
    println!("{:x}!", 27); // Виведе: 1b!

    // Виводимо двійкове представлення числа 27 з префіксом і заповненням нулями до ширини 10
    println!("{:#010b}", 27); // Виведе: 0b00011011

    println!("Success!");
}


/*
Форматування в assert_eq!:

Замість "__" у форматі потрібно використовувати форматні специфікатори для перетворення числа 27 у відповідний формат. Я виправив "__" на "{:b}", "{:o}" та "{:x}" для двійкового, восьмкового та шістнадцяткового форматів відповідно.

Вивід шістнадцяткових чисел:

У форматах "{:x}" та "{:X}" використовуються маленькі та великі літери для шістнадцяткових цифр.
*/