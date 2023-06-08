use std::io::{self, Read};

struct Player{
    name: String,
    health: i16,
    attack_damage: i8,
    armour_level: i8
}

fn main() {
    let mut test = create_player();

    println!("Hello {}!", test.name);
}

fn create_player() -> Player {
    let mut return_player = Player{
        name: "".to_string(),
        health: 0,
        attack_damage: 0,
        armour_level: 0
    };

    let mut player_name = String::new();
    println!("Enter the name of your charcter.");
    io::stdin().read_line(&mut player_name).expect("Failed to get input!");
    player_name = player_name.trim().to_string();
    return_player.name = player_name.clone();

    return return_player;
} 
