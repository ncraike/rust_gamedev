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
            action_prompt: graphics::Text::new(
                "Press E to eat food\nPress W to wait\nPress Q to quit"),
            pending_action: None,
        };
        new_game_state.status_text.update_status(&new_game_state.player);
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
                self.status_text.update_status(&self.player);
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
            &self.status_text,
            graphics::DrawParam::default().dest(nalgebra::Point2::new(0.0, 0.0)),
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
    health_text: graphics::Text,
    nutrition_text: graphics::Text,
    oxygen_text: graphics::Text,
}

impl StatusText {
    fn new() -> StatusText {
        let mut health_text = graphics::Text::new("Health: ");
        health_text.add("00");
        let mut nutrition_text = graphics::Text::new("Nutrition: ");
        nutrition_text.add("00");
        let mut oxygen_text = graphics::Text::new("Oxygen: ");
        oxygen_text.add("00");
        StatusText {
            health_text,
            nutrition_text,
            oxygen_text,
        }
    }

    fn update_status(&mut self, player: &Character) {
        let health_fragments = self.health_text.fragments_mut();
        health_fragments[1] = graphics::TextFragment::new(player.health.to_string());
        let nutrition_fragments = self.nutrition_text.fragments_mut();
        nutrition_fragments[1] = graphics::TextFragment::new(player.nutrition.to_string());
        let oxygen_fragments = self.oxygen_text.fragments_mut();
        oxygen_fragments[1] = graphics::TextFragment::new(player.oxygen.to_string());
    }
}

impl graphics::Drawable for StatusText {
    fn draw(&self, ctx: &mut Context, param: graphics::DrawParam) -> GameResult<()> {
        self.health_text.draw(ctx, param)?;

        let health_rec: graphics::Rect =
            graphics::Drawable::dimensions(&self.health_text, ctx).unwrap();
        let nutrition_dest = nalgebra::Point2::new(param.dest.x, health_rec.bottom() + 5.0);
        self.nutrition_text.draw(ctx, param.clone().dest(nutrition_dest))?;

        let nutrition_rec: graphics::Rect =
            graphics::Drawable::dimensions(&self.nutrition_text, ctx).unwrap();
        let oxygen_dest = nalgebra::Point2::new(
            param.dest.x,
            health_rec.bottom() + nutrition_rec.bottom() + 10.0);

        self.oxygen_text.draw(ctx, param.clone().dest(oxygen_dest))?;

        Ok(())
    }

    fn dimensions(&self, ctx: &mut Context) -> Option<graphics::Rect> {
        let health_rec: graphics::Rect =
            graphics::Drawable::dimensions(&self.health_text, ctx)?;
        let nutrition_rec: graphics::Rect =
            graphics::Drawable::dimensions(&self.nutrition_text, ctx)?;
        let oxygen_rec: graphics::Rect =
            graphics::Drawable::dimensions(&self.oxygen_text, ctx)?;
        Some(
            health_rec
                .combine_with(nutrition_rec)
                .combine_with(oxygen_rec)
        )
    }

    fn set_blend_mode(&mut self, mode: Option<graphics::BlendMode>) {
        self.health_text.set_blend_mode(mode);
        self.nutrition_text.set_blend_mode(mode);
        self.oxygen_text.set_blend_mode(mode);
    }

    fn blend_mode(&self) -> Option<graphics::BlendMode> {
        self.health_text.blend_mode()
    }
}
