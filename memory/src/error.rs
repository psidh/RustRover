// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }


// pub enum Option<T> {
//     Some(T),
//     None,
// }


fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("Cannot divide by zero"));
    }
    Ok(a / b)
}

fn find_element(array: &[i32], target: i32) -> Option<usize> {
    for (index, &element) in array.iter().enumerate() {
        if element == target {
            return Some(index);
        }
    }
    None
}

fn main() {
    let result = divide(10, 0);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }

    let array = [1, 2, 3, 4, 5];
    let target = 3;
    let result = find_element(&array, target);
    match result {
        Some(index) => println!("Element found at index: {}", index),
        None => println!("Element not found"),
    }
}
