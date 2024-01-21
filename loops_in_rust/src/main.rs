fn main() {
    println!("=====Loops in Rust======");

    // loop loop
    println!("\n-----loop loop-----");
    loop_loop ();

    // while loop
    println!("\n-----while loop-----");
    while_loop();

    // for loop
    println!("\n-----for loop-----");
    for_loop();

}

fn loop_loop () {
    let mut number = 0;
    loop {
        number += 1;
        println!("Loop line {}", number);

        if number == 10 {
            break;
        }
    }
}

fn while_loop () {
    let mut number = 1;
    while number != 11 {
        println!("Loop line {}", number);
        number += 1;
    }
}

fn for_loop () {
    for i in 1..11 {
        println!("Loop line {}", i);
    }
}