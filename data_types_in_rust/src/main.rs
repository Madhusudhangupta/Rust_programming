fn main() {
    println!("=====Data Types in Rust=====");

    // Scalar Type
    // Integer, Strings, Boolean, Floating, Char

    // length (8bit, 16, 32, 64, 128, arch) - signed(i) or unsigned(u) - 132, 164, u32, u128

    let number = 2;
    println!("Integer value: {}", number);

    let true_or_false = true;
    println!("Boolean value: {}", true_or_false);

    let char_type = "abc";
    println!("Character or String value: {}", char_type);

    let decimal_value = "12.45";
    println!("Decimal value: {}", decimal_value);

    println!("");

    // Compound types - store multiple data at a time
    // Arrays, Tuples, Dictionary

    // Arrays
    println!("----Array----");

    let array = [10,20,30,40];
    println!("Array elements: {:?}", array);
    println!("First element of array is: {}", array[0]); //use array[0]

    println!("");
    
    // Tuples
    println!("----Tuple----");

    let tuple = (1,2,3,4,5);
    println!("Tuple value: {:?}", tuple);
    println!("Third element of tuple is: {}", tuple.2); // use tuple.2

    let tuple2: (i32, u16, f64) = (10, 1, 23.7);
    println!("Tuple value with different data types: {:?}", tuple2);

}
