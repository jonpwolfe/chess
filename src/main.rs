fn main() {
    let mut game = Game::new();
    game.run();
}

struct Game {
    game_type : GameType,
    players : Vec<Player>,
    board : Board,
    notation: Notation,
    clock : Option<GameClock>,
    result : Option<GameResult>,
}

impl Game {
    fn new() -> Self {
        let game_type = GameType::Checkers;
        let mut players = Vec::new();
        for _i in 0..2 {
            let player = Player::new();
            players.push(player);
        }
        let board = Board::new();
        let notation = Notation::new();
        let clock = None;
        let result = None;
        Game{game_type, players, board, notation, clock, result}
    }

    fn run(&mut self) {

    }
}

enum GameType {
    Checkers,
    Chess,
    BugHouse,
}

impl GameType {

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