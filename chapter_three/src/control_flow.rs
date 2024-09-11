fn main() {
    println!("Main function \n");
    function_on_loop(5);
    let boolean_answer: bool = function(43);
    println!("The number  is {}", boolean_answer);
    while_function(4);
}

fn function(number: i32) -> bool {
    if number > 0 {
        true
    } else {
        false
    }
}

fn function_on_loop(mut num2: i64) {
    let x: i32 = loop {
        println!("Entering the loop");

        if num2 == 10 {
            break 10;
        }
        num2 += 1;
    };

    println!("The value of x : {}", x);
}

fn while_function(num2: i32) {
    let mut x: i32 = num2;

    while x > 0 {
        x -= 1;
        println!("{x}");
    }
    println!("End loop")
}

fn for_function(num1: i64) {
    let bottom: i64 = 0;
    let top: i64 = 10;
    for index in bottom..=top {
        println!("{}", index);
    }

    println!("End of for loop");
}
