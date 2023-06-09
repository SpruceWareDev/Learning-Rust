mod menu;
mod game;

fn main() {
    let option: i8 = menu::menu();
    match option {
        1 => game::new_game(),
        2 => println!("No Tutorial Implemented Yet!"),
        3 => return,
        _ => main()
    }
}
