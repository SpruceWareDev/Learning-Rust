use std::io::{self, Read};

fn print_menu() {
    println!("--- Main Menu ---");
    println!("[1] New Game");
    println!("[2] Tutorial");
    println!("[3] Quit");
    println!("-----------------");
}

fn handle_menu() -> i8{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to get input!");
    input = input.trim().to_string();

    match input.as_str() {
        "1" => return 1,
        "2" => return 2,
        "3" => return 3,
        _ => return -1
    }
}

pub fn menu() -> i8{
    print_menu();
    let mut option: i8 = handle_menu();
    while option == -1 {
        println!("Invalid Option! Enter 1, 2 or 3.");
        option = handle_menu();
    }
    return option;
}