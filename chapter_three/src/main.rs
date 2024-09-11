// Functions in Rust

fn some_function() {
    println!("non returning s function");
}

fn return_function() -> i32 {
    println!("returning function");
    32
}

fn parameterized_function(x: i64, y: i64) {
    println!("accepts parameters but no return");
    println!("x : {x}");
    println!("y: {y}");
}

fn function(num1: i32, num2: i32) -> i32 {
    let answer: i32 = num1 + num2;
    println!("Returns");
    answer
}

fn main() {
    println!("Main function");
    let mut answer1: i32 = function(32, 64);
    println!("the sum of two number: {answer1}")
}
