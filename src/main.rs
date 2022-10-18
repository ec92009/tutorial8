fn main() {

    let number = {
        let x = 3;
        x + 1
    };

    println!("Hello, world!, number = {}", number);

    let x = 3i32;
    let y = 43i32;
    println!("Hello, world!, x = {}, y = {}, x+y = {}", x, y, add(x, y));
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

