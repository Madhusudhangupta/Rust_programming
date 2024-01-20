fn main() {
    println!("======Functions=====");

    hello_fn();

    // passing single parameter
    number_fn(5);

    // passing multiple parameters
    some_fun(10, 'A');

    // epressions
    ex();

    // function returning some value
    let returned_value = return_value();
    println!("The returned value is {}", returned_value);
}

fn  hello_fn() {
    println!("This is hello function");
}

fn number_fn(x: i32) {
    println!("The value of x is: {}", x);
}

fn some_fun(x: i32, y: char) {
    println!("The number value of x is: {x} and the character value of y is {y}");
}

fn ex () {
    let y = {
        let x = 3;
        x+2
    };
    println!("The value of y is: {}", y);
}

// return value from function
fn return_value () -> i32 {
    println!("This function returns an integer");
    10 + 20
}