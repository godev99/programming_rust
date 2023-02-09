struct Rectangle {
    width: u32,
    length: u32,
}

fn main() {
    let rec = Rectangle {
        width: 32,
        length: 24,
    };

    println!("For a {}x{} rectangle, area is {}",
             rec.width,
             rec.length,
             area(&rec));

    let x;
    x = 42;
    assert_eq!(x, 42);
}

fn area(r: &Rectangle) -> u32 {
    r.width * r.length
}