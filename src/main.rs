use std::io;

enum GameResult {
    PlayerDeath,
    UserQuits,
}

struct Character {
    health: i64,
    nutrition: i64,
    oxygen: i64,
}

enum UserAction {
    PlayerDo(CharacterAction),
    QuitGame,
}

enum CharacterAction {
    Eat,
    Wait,
}

fn main() {
    match game_loop() {
        GameResult::PlayerDeath => println!("You have died!"),
        GameResult::UserQuits => println!("Quitting game..."),
    }
}

fn game_loop() -> GameResult {
    let mut player = Character {
        health: 10,
        nutrition: 10,
        oxygen: 10,
    };

    while player.is_alive() {
        player.print_status();
        let action = prompt_for_action();
        match action {
            UserAction::PlayerDo(player_action) => player.do_action(player_action),
            UserAction::QuitGame => return GameResult::UserQuits,
        }
        player.tick();
    }
    return GameResult::PlayerDeath;
}

impl Character {

    fn is_alive(&self) -> bool {
        self.health > 0
    }

    fn print_status(&self) {
        println!("
Health: {}
Nutrition: {}
Oxygen: {}
", self.health, self.nutrition, self.oxygen);
    }

    fn do_action(&mut self, action: CharacterAction) {
        match action {
            CharacterAction::Eat => self.nutrition += 10,
            CharacterAction::Wait => (),
        }
    }

    fn tick(&mut self) {
        if self.nutrition < 0 {
            self.health -= 1;
        }
        if self.oxygen < 0 {
            self.health -= 10;
        }
        self.nutrition -= 1;
    }
}

fn prompt_for_action() -> UserAction {
    loop {
        println!("What do you do?
E)at
W)ait
Q)uit the game
");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)
            .expect("Failed to read line");

        match user_input.chars().next() {
            Some('e') => return UserAction::PlayerDo(CharacterAction::Eat),
            Some('w') => return UserAction::PlayerDo(CharacterAction::Wait),
            Some('q') => return UserAction::QuitGame,
            _ => (),
        };
    }
}
