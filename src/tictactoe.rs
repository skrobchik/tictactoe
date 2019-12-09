pub type Line = [TileState; 3];
pub type Board = [Line; 3];

pub type Coordinate = (usize, usize);

macro_rules! return_winnable {
    ($x:expr) => {
        let line = $x;
        let winning = Game::is_winning_line(line);
        if winning.is_some() {
            return winning;
        }
    };
}

macro_rules! player_to_game_outcome {
    ($x:expr) => {
        match $x {
            Player::CROSS => GameOutcome::CROSS_WIN,
            Player::CIRCLE => GameOutcome::CIRCLE_WIN,
        }
    };
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TileState {
    CROSS,
    CIRCLE,
    EMPTY,
}

#[derive(Copy, Clone)]
pub enum Player {
    CROSS,
    CIRCLE,
}

pub enum GameOutcome {
    CROSS_WIN,
    CIRCLE_WIN,
    DRAW,
}

pub struct Game {
    board: Board,
    game_turn: Player,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: [[TileState::EMPTY; 3]; 3],
            game_turn: Player::CROSS,
        }
    }

    pub fn from_board(board: Board, player: Player) -> Game {
        Game {
            board: board,
            game_turn: player,
        }
    }

    pub fn empty_tiles(&self) -> Vec<Coordinate> {
        let mut tiles = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                if self.board[i][j] == TileState::EMPTY {
                    tiles.push((i, j));
                }
            }
        }
        tiles
    }

    pub fn get_game_turn(&self) -> Player {
        self.game_turn
    }

    pub fn make_move(&mut self, coord: Coordinate) {
        self.board[coord.0][coord.1] = match self.game_turn {
            Player::CROSS => TileState::CROSS,
            Player::CIRCLE => TileState::CIRCLE,
        };
        self.game_turn = match self.game_turn {
            Player::CROSS => Player::CIRCLE,
            Player::CIRCLE => Player::CROSS,
        };
    }

    pub fn get_children(&self) -> Vec<Game> {
        let mut children: Vec<Game> = Vec::new();
        let empty_tiles = self.empty_tiles();
        for tile in empty_tiles {
            let mut child: Game = Game::from_board(self.board, self.game_turn);
            child.make_move(tile);
            children.push(child);
        }
        children
    }
    pub fn get_board(&self) -> Board {
        self.board
    }

    fn get_row(&self, i: usize) -> Line {
        [self.board[i][0], self.board[i][1], self.board[i][2]]
    }

    fn get_column(&self, j: usize) -> Line {
        [self.board[0][j], self.board[1][j], self.board[2][j]]
    }

    fn get_positive_diagonal(&self) -> Line {
        [self.board[2][0], self.board[1][1], self.board[0][2]]
    }

    fn get_negative_diagonal(&self) -> Line {
        [self.board[0][0], self.board[1][1], self.board[2][2]]
    }

    fn is_winning_line(line: Line) -> Option<Player> {
        if line[0] == line[1] && line[1] == line[2] && line[0] != TileState::EMPTY {
            return Some(match line[0] {
                TileState::CROSS => Player::CROSS,
                TileState::CIRCLE => Player::CIRCLE,
                TileState::EMPTY => unreachable!(),
            });
        }
        None
    }

    fn is_winning(&self) -> Option<Player> {
        for i in 0..3 {
            return_winnable!(self.get_row(i));
            return_winnable!(self.get_column(i));
        }
        return_winnable!(self.get_negative_diagonal());
        return_winnable!(self.get_positive_diagonal());
        None
    }

    fn count_empty_tiles(&self) -> u8 {
        let mut n = 0;
        for i in 0..3 {
            for j in 0..3 {
                if self.board[i][j] == TileState::EMPTY {
                    n = n + 1;
                }
            }
        }
        n
    }

    pub fn get_outcome(&self) -> Option<GameOutcome> {
        if self.count_empty_tiles() != 0 {
            return None;
        }
        let winning = self.is_winning();
        if winning.is_none() {
            return Some(GameOutcome::DRAW);
        }
        Some(player_to_game_outcome!(winning.unwrap()))
    }

    pub fn to_string(&self) -> String {
        let mut output: String = String::new();
        for i in 0..3 {
            output.push_str("| ");
            for j in 0..3 {
                let tile = self.board[i][j];
                output.push_str(match tile {
                    TileState::EMPTY => "   ",
                    TileState::CIRCLE => " o ",
                    TileState::CROSS => " x ",
                });
            }
            output.push_str(" |\n");
        }
        output
    }
}
