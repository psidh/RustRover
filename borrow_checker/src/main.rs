fn main() {
    println!("Welcome to Borrow Checker");
    borrow_checker();
    ownership();
    mutable_reference();
    let s : String = String::from("Sidharth");
    first_word(&s);
}

//Ownership
fn borrow_checker() {
    let mut s = String::from("Hello");
    s.push_str("string");
    print!("{s}");

    // transfer of ownership from an old owner to a new one

    let x: i32 = 5;
    let y: i32 = x;
    print!("{x}");
    print!("{y}");

    // the ownership did not transferred from x to y of the value 5 in the memory

    // let s1_string: String = String::from("one string");
    // let s2_string = s1_string;

    //the ownership got transferred due the fact that heap was used
    // print!("{s1_string} "); would result in error

    // instead we use
    // let one_string: String = String::from("Sidharth");
    // let two_string: String = one_string.clone();
}

fn ownership() {
    let s1_string: String = String::from("Sidharth");
    takes_ownership(s1_string); // Ownership of s1_string is moved to the function
                                // s1_string.push_str("string"); <- would result in an error
                                // cause the ownership got transferred to the function
                                // instead a good approach would be to use reference to of the string var

    let s2_string: String = String::from("Chandan");
    takes_ownership_by_reference(&s2_string);
    // this works
}

fn takes_ownership(str: String) {
    print!("{str}");
}

fn takes_ownership_by_reference(str: &String) {
    print!("{str}");
}

//___________________________________________________________
fn mutable_reference() {
    println!("Mutable Reference");

    let mut s1_string: String = String::from("Manas");

    // Create an immutable reference to `s1_string` and print it
    let s3_string: &String = &s1_string;
    println!("Before mutation: {s3_string}"); // prints "Manas"

    // Create a new scope where you have a mutable reference
    {
        let s2_string: &mut String = &mut s1_string;
        s2_string.push_str(" string"); // Modifying the string
        println!("After mutation using mutable reference: {s2_string}"); // prints "Manas string"
        print!("mutable changed : {s1_string}");
    } // Mutable reference `s2_string` goes out of scope here

    // Now you can create a new immutable reference and print the updated string
    let s4_string: &String = &s1_string;
    println!("Final modified string: {s4_string}"); // prints "Manas string"
}

fn first_word(s: &String)-> usize {
    let bytes = s.as_bytes();

    let mut s = String::from("Hello World");
    let _word = first_word(&s);

    s.clear();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}
