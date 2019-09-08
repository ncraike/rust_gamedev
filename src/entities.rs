
use ggez::nalgebra as na;

type Point2 = na::Point2<f32>;

pub struct GameWorld {
    pub player: Character,
}

impl GameWorld {
    pub fn new() -> GameWorld {
        GameWorld {
            player: Character::new(),
        }
    }

    pub fn player_do_action(&mut self, action: CharacterAction) {
        self.player.do_action(action);
    }

    pub fn is_finished(&self) -> bool {
        !(self.player.is_alive())
    }

    pub fn tick(&mut self) {
        self.player.tick();
    }
}

pub struct Character {
    pub pos: Point2,
    pub health: i64,
    pub nutrition: i64,
    pub oxygen: i64,
}

impl Character {
    fn new() -> Character {
        Character {
            pos: Point2::origin(),
            health: 5,
            nutrition: 5,
            oxygen: 5,
        }
    }
}

impl Character {

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn do_action(&mut self, action: CharacterAction) {
        match action {
            CharacterAction::Eat => self.nutrition += 10,
            CharacterAction::Wait => (),
        }
    }

    pub fn tick(&mut self) {
        if self.nutrition < 0 {
            self.health -= 1;
        }
        if self.oxygen < 0 {
            self.health -= 10;
        }
        self.nutrition -= 1;
    }
}

#[derive(Copy,Clone)]
pub enum CharacterAction {
    Eat,
    Wait,
}
