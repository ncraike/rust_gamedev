use std::io;

struct Player {
    health: i64,
    nutrition: i64,
    oxygen: i64,
}

enum Action {
    Eat,
    Wait,
    Quit,
}

fn main() {
    let mut player = Player {
        health: 10,
        nutrition: 10,
        oxygen: 10,
    };

    while player_is_alive(&player) {
        player_status(&player);
        let action = prompt_for_action();
        player_action(&mut player, action);
        player.tick();
    }
    player_death();
}

impl Player {
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

fn prompt_for_action() -> Action {
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
            Some('e') => return Action::Eat,
            Some('w') => return Action::Wait,
            Some('q') => return Action::Quit,
            _ => (),
        };
    }
}

fn player_action(player: &mut Player, action: Action) {
    match action {
        Action::Eat => player.nutrition += 10,
        Action::Wait => (),
        Action::Quit => panic!("Quitting game"),
    }
}

fn player_is_alive(player: &Player) -> bool {
    player.health > 0
}

fn player_status(player: &Player) {
    println!("
Health: {}
Nutrition: {}
Oxygen: {}
", player.health, player.nutrition, player.oxygen);
}

fn player_death() {
    println!("Player is dead");
}
