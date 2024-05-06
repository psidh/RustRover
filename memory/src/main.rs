fn main() {
    // no garbage collector
    // Mutability: constant

    let _x: i32 = 1;
    // not possible x = 2

    // mutate it
    let mut _y: i64 = 560;

    // ensures thread safety, no sync needed for immutability, now compiler can optimize the condition as no race condition

    /*
    Stack v/s Heap

    Rust uses Stack:Fast for primitive data-types - run time static
    int, float, booleans, arrays

    Rust uses Heap:Slow for vector string

    Stack stores `temp` which points to Heap;

    Every function has a frame - called frame and there we store the variables

    */

    /*
    OwnerShip
    let s1: String = String::from("Hello");

    let s2 = s1;

    println!("{}", s1); <= X can't happen - owner is s2 now!

    */

    /*
    Borrowing and References

    let s1: String = String::from("Hello");

    let s2 = &s1 ; <= borrower, pass by reference

    println!("{}", s1); <= can happen , cause s2 was a borrower


    update

    */
    initial();
}

fn initial() {
    let mut str: String = String::from("Hello");
    let _answer = &mut str;
    update_str(_answer);
    print!("{}", str);
}

fn update_str(s: &mut String) {
    s.push_str(" World");
}
