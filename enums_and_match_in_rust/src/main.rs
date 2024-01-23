enum Subjects {
    Science,
    Math,
    English,
    History
}

fn main() {
    println!("=====Enum and match=====\n");

    let subject:Subjects = Subjects::English;

    match subject {
        Subjects::Science => {
            println!("You chose {} subject.", "Science");
        },
        Subjects::Math => {
            println!("You chose {} subject.", "Math");
        },
        Subjects::English => {
            println!("You chose {} subject.", "English");
        },
        Subjects::History => {
            println!("You chose {} subject.", "History");
        },
        _ => {
            println!("Wrong subject!");
        }
    }
}
