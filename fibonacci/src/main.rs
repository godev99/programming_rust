use std::io;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let mut number = String::new();
    println!("Fibonacci");

    loop {
        println!("Enter fibonacci number :");
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            _ => 0,
        };

        if number > 0 && number < 2_147_483_647 {
            break
        } else {
            println!("Please provide a number between 1 and 2_147_483_647");
        }
    }

    let number: i32 = match number.trim().parse() {
        Ok(num) => num,
        _ => 0,
    };

    for n in 1..=number {
        if n == 1 {
            v.push(1);
        } else {
            v.push((n-2) + (n-1))
        }
    }
    println!("v : {:?}", v);
}
