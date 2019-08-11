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

    while player.is_alive() {
        player.print_status();
        let action = prompt_for_action();
        player.do_action(action);
        player.tick();
    }
    println!("Player is dead");
}

impl Player {

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

    fn do_action(&mut self, action: Action) {
        match action {
            Action::Eat => self.nutrition += 10,
            Action::Wait => (),
            Action::Quit => panic!("Quitting game"),
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
