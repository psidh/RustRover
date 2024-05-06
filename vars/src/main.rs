fn main() {
    // variables in Rust
    let _x: i32 = 1;
    /*

       number of bits required to represent the bit
    "i" represents signed or unsigned integer
    we have i8, i16, i32, i64, i128 ranges for memmory optimisation
    default is 32 bits

    */

    let _y: u16 = 54;
    let _z: f32 = 5.67;

    /*

     By the way, all variables in Rust are immutable

     To make a mutable variable make it `mut` to mutate the values


    */

    let _is_male: bool = true;

    let _greeting: String = String::from("Sidharth");

    let _new_string: &str = "sexy";

    //print!("{}", greeting.chars().nth(1000));
    //
    //
    //
    //conditionals

    //if(condition){
    //}

    //loop

    //  for i in 0..10 {
    //      println!("{}", i);
    //  }

    // iteration over ds

    let sentence: String = String::from("my name is singh");

    let first_word = get_word(sentence);
    print!("{}", first_word);

    let mut _number_one: i32 = 42;
    let mut _number_two: i32 = 58;

    let _sum_number: i32 = do_sum(_number_one, _number_two);
    print!("{}", _sum_number);

    //  println!("x : {x}, y: {y}, z :{z}");
}

// ownership
fn get_word(sentence: String) -> String {
    let mut ans: String = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }

    return ans;
}

fn do_sum(a: i32, b: i32) -> i32 {
    return a + b;
}
