extern crate adversarial_search;

mod tictactoe;

use tictactoe::Game;
use adversarial_search::prelude::minimax;
use tictactoe::Coordinate;

use std::io::stdin;

macro_rules! to_search_player {
    ($x:expr) => {
        match $x{
            tictactoe::Player::CROSS => adversarial_search::prelude::Player::MAX,
            tictactoe::Player::CIRCLE => adversarial_search::prelude::Player::MIN,
        }
    };
}

fn main() {
    let mut game: Game = Game::new();
    println!("{}", game.to_string());
    
    for _ in 0..9{
        let move_coordinate = prompt_move(&game);
        game.make_move(move_coordinate);
        println!("{}", game.to_string());
        let eval = minimax(
            &game,
            10,
            &|n| n.get_children(),
            &|n| {
                let winning = n.is_winning();
                if winning.is_none(){
                    return None;
                }
                Some(to_search_player!(winning.unwrap()))
            },
            &|n| {return 0;},
            to_search_player!(game.get_game_turn())
        );
        println!("eval: {}", eval);
    }
}

fn prompt_move(game: &Game) -> Coordinate {
    let available_moves = game.empty_tiles();
    let mut input_buffer = String::new();
    loop {
        input_buffer.clear();
        let result = stdin().read_line(&mut input_buffer);
        if result.is_ok() {
            let s = input_buffer.trim();
            let controls = "uiohjkbnm";
            let index = controls.find(s);

            if index.is_none() {
                print!("Enter a tile to play the next move:\n|u|i|o|\n|h|j|k|\n|b|n|m|\n");
                continue;
            }
            let index = index.unwrap();
            let coord = (index / 3, index % 3);
            if available_moves.binary_search(&coord).is_ok(){
                return coord;
            }
            else {
                println!("That tile is occupied");
                continue;
            }
        }
        else {
            println!("Invalid string");
        }
    }
}