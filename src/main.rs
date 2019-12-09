extern crate adversarial_search;

mod tictactoe;

use adversarial_search::prelude::minimax;
use tictactoe::Coordinate;
use tictactoe::Game;
use tictactoe::Player;
use tictactoe::GameOutcome;

use std::io::stdin;

fn is_maximizing_player(player: Player) -> bool {
    match player {
        Player::Cross => true,
        Player::Circle => false,
    }
}

fn to_search_terminality(outcome: Option<GameOutcome>) -> Option<f32> {
    if outcome.is_none() {
        return None;
    }
    match outcome.unwrap() {
        GameOutcome::CrossWin => Some(std::f32::INFINITY),
        GameOutcome::CircleWin => Some(-std::f32::INFINITY),
        GameOutcome::Draw => Some(0.0),
    }
}

fn main() {
    let mut game: Game = Game::new();
    println!("{}", game.to_string());

    for _ in 0..9 {
        let move_coordinate = prompt_move(&game);
        game.make_move(move_coordinate);
        println!("{}", game.to_string());
        let eval = minimax(
            &game,
            10,
            &|n| n.get_children(),
            &|n| to_search_terminality(n.get_outcome()),
            &|_| { 0 },
            is_maximizing_player(game.get_game_turn()),
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
            if available_moves.binary_search(&coord).is_ok() {
                return coord;
            } else {
                println!("That tile is occupied");
                continue;
            }
        } else {
            println!("Invalid string");
        }
    }
}
