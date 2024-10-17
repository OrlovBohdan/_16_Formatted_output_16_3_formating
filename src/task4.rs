#[test]

/*
fn main() {
    // Left align
    println!("Hello {:<5}!", "x"); // => Hello x    !
    // Right align
    assert_eq!(format!("Hello __!", "x"), "Hello     x!");
    // Center align
    assert_eq!(format!("Hello __!", "x"), "Hello   x  !");

    // Left align, pad with '&'
    assert_eq!(format!("Hello {:&<5}!", "x"), __);

    println!("Success!");
}
*/

fn main() {
    // Ліве вирівнювання
    println!("Hello {:<5}!", "x"); // => Hello x    !

    // Праве вирівнювання
    assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!");

    // Центрування
    assert_eq!(format!("Hello {:^5}!", "x"), "Hello   x  !");

    // Ліве вирівнювання, заповнення символом '&'
    assert_eq!(format!("Hello {:&<5}!", "x"), "Hello x&&&&!");

    println!("Success!");
}



/*
{:>5}: Праве вирівнювання рядка у ширину 5 символів.
{:^5}: Центрування рядка у ширину 5 символів.
{:&<5}: Ліве вирівнювання рядка з заповненням символом & до ширини 5 символів.
*/