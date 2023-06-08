mod menu;
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

fn main() {
    let option: i8 = menu::menu();
    match option {
        1 => println!("New Game Selected!"),
        2 => println!("No Tutorial Implemented Yet!"),
        3 => return,
        _ => return
    }
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
    io::stdin().read_line(&mut player_name).expect("Failed to get input!");
    player_name = player_name.trim().to_string();
    return_player.name = player_name.clone();

    return return_player;
} 
