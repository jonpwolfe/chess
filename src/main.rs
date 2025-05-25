fn main() {
    let mut game = Game::new();
    game.run();
}

struct Game {
    board : Board,
    players : Vec<Player>,
    pieces : Vec<Box<dyn Piece>>,
    notation: Notation,
    clock : Option<GameClock>,
    result : Option<GameResult>,
}

impl Game {
    fn new() -> Self {
        let board = Board::new();
        let mut players = Vec::new();
        for _i in 0..2 {
            players.push(Player::new());
        }
        let mut pieces : Vec<Box<dyn Piece>> = Vec::new();
        let pawn = Pawn::new(Position::new(PositionLetter::E,2));
        pieces.push(Box::new(pawn));
        let notation = Notation::new();
        let clock = None;
        let result = None;
        Game{board, players, pieces, notation, clock, result}
    }

    fn run(&mut self) {

    }
}

struct Player {

}

impl Player {
    fn new() -> Self {
        Player{}
    }
}

struct Board {

}

impl Board {
    fn new() -> Self {
        Board{}
    }
}
#[derive(Copy, Clone)]
struct Position {
    letter: PositionLetter,
    number: usize,
}
impl Position {
    fn new(letter: PositionLetter, number: usize) -> Self {
        Position{letter,number}
    }
}
#[derive(Copy, Clone)]
enum PositionLetter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}
trait Piece {
    fn get_name(&self) -> &str;
    fn get_position(&self) -> Position;
}

struct Pawn {
    position: Position,
}

impl Pawn {
    fn new(position: Position) -> Self {
        Pawn{position}
    }
}

impl Piece for Pawn {
    fn get_name(&self) -> &str {
        "Pawn"
    }
    fn get_position(&self) -> Position {
        self.position
    }
}
struct Notation {

}

impl Notation {
    fn new() -> Self {
        Notation{}
    }
}

struct GameClock {

}

impl GameClock {
    fn new() -> Self {
        GameClock{}
    }
}

struct GameResult {

}

impl GameResult {
    fn new() -> Self {
        GameResult{}
    }
}