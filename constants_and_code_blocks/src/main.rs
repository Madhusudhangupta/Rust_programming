fn main() {
    println!("=====Constants and Code Blocks=====");

    println!("\n-----Constant-----\n");

    const PI:f64 = 3.14;
    println!("The value of PI is {} and it is constant", PI);

    //  if we change the value of PI, it will throw an error
    // PI = 10.0; // error - invalid left-hand side of assignment

    println!("\n-----Code Block-----\n");
    {
        let x = 5;
        {
            let y = 10;
            println!("x = {} and y = {}", x, y);
        }
        // if we print y here it wil throw an error
        println!("x = {}", x);
        // println!("y = {}", y); // cannot find value `y` in this scope
    }
}
