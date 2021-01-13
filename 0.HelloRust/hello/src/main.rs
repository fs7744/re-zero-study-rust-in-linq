const fn add() -> i32 {
    6 * 7 * 6
}

const ADD_D: i32 = add();

fn main() {
    println!("Hello, {}!", ADD_D);
}
