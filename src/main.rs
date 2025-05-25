fn main() {
    let mut game = Game::new();
    game.run();
}

struct Game {
    players : Vec<Player>,
    board : Board,
    notation: Notation,
    clock : Option<GameClock>,
    result : Option<GameResult>,
}

impl Game {
    fn new() -> Self {
        let mut players = Vec::new();
        for _i in 0..2 {
            let player = Player::new();
            players.push(player);
        }
        let board = Board::new();
        let notation = Notation::new();
        let clock = None;
        let result = None;
        Game{players, board, notation, clock, result}
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