use std::io::{self, Read};
struct Player{
    name: String,
    health: i16,
    armour_level: i8,
    weapon: Weapon
}

struct Weapon {
    name: String,
    damage: i8
}

pub fn new_game(){
    let mut running: bool = true;

    let mut player: Player = create_player();

    while running {
        let mut text_in: String = String::new();
        match io::stdin().read_line(&mut text_in) {
            Err(error) => panic!("Failed to get input: {}", error),
            Ok(text) => text
        };
        let text_in = text_in.trim().to_string();

        match text_in.as_str() {
            "name" => println!("Your name is {}.", player.name.as_str()),
            "help" => show_help(),
            "ping" => println!("pong!"),
            "exit" => running = false,
            _ => println!("Invalid Input!")
        }
    }
}

fn show_help() {
    println!("Here are all the commands you can run:");
    println!("help - shows you this.");
    println!("ping - sends pong back.");
    println!("exit - exits the game.");
}

fn create_player() -> Player {
    let mut return_player = Player{
        name: "".to_string(),
        health: 100,
        armour_level: 0,
        weapon: Weapon { name: "Fists".to_string(), damage: 2 }
    };
    
    let mut player_name = String::new();
    println!("Enter the name of your charcter.");
    match io::stdin().read_line(&mut player_name) {
        Err(reason) => panic!("Failed to get input: {}!", reason),
        Ok(text) => text,
    };
    player_name = player_name.trim().to_string();
    return_player.name = player_name.clone();

    return return_player;
} 