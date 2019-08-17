use ggez::{Context, ContextBuilder, GameResult};
use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::input::keyboard;
use ggez::nalgebra;


fn main() {
    let state = &mut GameState::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("hello_ggez", "Nathan")
        .conf(conf::Conf::new())
        .build()
        .unwrap();
    event::run(ctx, event_loop, state).unwrap();
}

struct GameState {
    player: Character,
    status_text: StatusText,
    action_prompt: graphics::Text,
    pending_action: Option<UserAction>,
}

impl GameState {
    fn new() -> GameState {
        let mut new_game_state = GameState {
            player: Character::new(),
            status_text: StatusText::new(),
            action_prompt: graphics::Text::new("Press E to eat food, W to wait, Q to quit"),
            pending_action: None,
        };
        new_game_state.status_text.update_text(&new_game_state.player);
        return new_game_state;
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.pending_action {
            Some(UserAction::PlayerDo(player_action)) => {
                self.player.do_action(player_action);
                self.pending_action = None;
                self.player.tick();
                self.status_text.update_text(&self.player);
            }
            Some(UserAction::QuitGame) => {
                event::quit(ctx);
            }
            _ => (),
        }
        if !self.player.is_alive() {
            event::quit(ctx);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        graphics::draw(
            ctx,
            &self.status_text.health,
            graphics::DrawParam::default().dest(nalgebra::Point2::new(0.0, 0.0)),
        )?;
        graphics::draw(
            ctx,
            &self.status_text.nutrition,
            graphics::DrawParam::default().dest(nalgebra::Point2::new(0.0, 20.0)),
        )?;
        graphics::draw(
            ctx,
            &self.status_text.oxygen,
            graphics::DrawParam::default().dest(nalgebra::Point2::new(0.0, 40.0)),
        )?;
        graphics::draw(
            ctx,
            &self.action_prompt,
            graphics::DrawParam::default().dest(nalgebra::Point2::new(0.0, 80.0)),
        )?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: keyboard::KeyCode,
        _keymods: keyboard::KeyMods,
    ) {
        match keycode {
            keyboard::KeyCode::E => {
                self.pending_action = Some(UserAction::PlayerDo(CharacterAction::Eat));
            }
            keyboard::KeyCode::W => {
                self.pending_action = Some(UserAction::PlayerDo(CharacterAction::Wait));
            }
            keyboard::KeyCode::Q | keyboard::KeyCode::Escape => {
                self.pending_action = Some(UserAction::QuitGame);
            }
            _ => (),
        };
    }
}

struct Character {
    health: i64,
    nutrition: i64,
    oxygen: i64,
}

impl Character {
    fn new() -> Character {
        Character {
            health: 5,
            nutrition: 5,
            oxygen: 5,
        }
    }
}

impl Character {

    fn is_alive(&self) -> bool {
        self.health > 0
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

#[derive(Copy,Clone)]
enum UserAction {
    PlayerDo(CharacterAction),
    QuitGame,
}

#[derive(Copy,Clone)]
enum CharacterAction {
    Eat,
    Wait,
}

struct StatusText {
    health: graphics::Text,
    nutrition: graphics::Text,
    oxygen: graphics::Text,
}

impl StatusText {
    fn new() -> StatusText {
        StatusText {
            health: graphics::Text::new(""),
            nutrition: graphics::Text::new(""),
            oxygen: graphics::Text::new(""),
        }
    }

    fn update_health(&mut self, health_value: i64) {
        self.health = graphics::Text::new("Health: ");
        self.health.add(health_value.to_string());
    }

    fn update_nutrition(&mut self, nutrition_value: i64) {
        self.nutrition = graphics::Text::new("Nutrition: ");
        self.nutrition.add(nutrition_value.to_string());
    }

    fn update_oxygen(&mut self, oxygen_value: i64) {
        self.oxygen = graphics::Text::new("Oxygen: ");
        self.oxygen.add(oxygen_value.to_string());
    }

    fn update_text(&mut self, player: &Character) {
        self.update_health(player.health);
        self.update_nutrition(player.nutrition);
        self.update_oxygen(player.oxygen);
    }
}
