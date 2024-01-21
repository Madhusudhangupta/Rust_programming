fn main() {
    println!("====If Else Condition=====");

    let number = 33;
    if number % 2 == 0 {
        println!("Number is divisible by 2");
    }
    else if number % 3 == 0 {
        println!("Number is divisible by 3");
    }
    else if number % 5 == 0 {
        println!("Number is divisible by 5");
    }
    else {
        println!("Number is not divisible by 2, 3  or 5");
    }
}
