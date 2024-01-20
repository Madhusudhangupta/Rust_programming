fn main() {
    println!("======Variables in Rust=====");

    let num = 5; // immutable - isko change nhi kr skte
    println!("The value of num is: {} and it cannot be changed.", num);

    // num = 10;
    // println!("{}", num); // error - cannot assign twice to immutable variable

    let mut new_number = 12;
    println!("The value of new_number is: {}", new_number);
    new_number = 15; // new_number changes to 15 because of `mut` keyword
    println!("The value of new_number is changed to: {}", new_number); // prints 15 

    let some_string = "This is a string.";
    println!("{}", some_string);
}
