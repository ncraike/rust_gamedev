use ggez::{Context, GameResult};
use ggez::graphics::{
    clear, draw, present, BlendMode, Drawable, DrawParam, Rect, Text, TextFragment};
use ggez::input::keyboard;
use ggez::nalgebra;

use crate::entities::{Character, CharacterAction, GameWorld};


pub struct UI {
    status_text: StatusText,
    action_prompt: Text,
    pending_action: Option<UserAction>,
    should_quit: bool,
}

impl UI {
    pub fn new(world: &GameWorld) -> GameResult<UI> {
        let mut ui = UI {
            status_text: StatusText::new(),
            action_prompt: Text::new(
                "Press E to eat food\nPress W to wait\nPress Q to quit"),
            pending_action: None,
            should_quit: false,
        };
        ui.status_text.update_status(&world.player);
        Ok(ui)
    }

    pub fn update(&mut self, _ctx: &mut Context, world: &mut GameWorld) -> GameResult<()> {
        match self.pending_action {
            Some(UserAction::PlayerDo(player_action)) => {
                world.player_do_action(player_action);
                self.pending_action = None;
                world.tick();
                self.status_text.update_status(&world.player);
            }
            Some(UserAction::QuitGame) => {
                self.should_quit = true;
            }
            _ => (),
        }
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        draw(
            ctx,
            &self.status_text,
            DrawParam::default().dest(nalgebra::Point2::new(0.0, 0.0)),
        )?;
        draw(
            ctx,
            &self.action_prompt,
            DrawParam::default().dest(nalgebra::Point2::new(0.0, 80.0)),
        )?;
        present(ctx)?;
        Ok(())
    }

    pub fn key_up_event(
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

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
}

pub struct StatusText {
    health_text: Text,
    nutrition_text: Text,
    oxygen_text: Text,
}

impl StatusText {
    pub fn new() -> StatusText {
        let mut health_text = Text::new("Health: ");
        health_text.add("00");
        let mut nutrition_text = Text::new("Nutrition: ");
        nutrition_text.add("00");
        let mut oxygen_text = Text::new("Oxygen: ");
        oxygen_text.add("00");
        StatusText {
            health_text,
            nutrition_text,
            oxygen_text,
        }
    }

    pub fn update_status(&mut self, player: &Character) {
        let health_fragments = self.health_text.fragments_mut();
        health_fragments[1] = TextFragment::new(player.health.to_string());
        let nutrition_fragments = self.nutrition_text.fragments_mut();
        nutrition_fragments[1] = TextFragment::new(player.nutrition.to_string());
        let oxygen_fragments = self.oxygen_text.fragments_mut();
        oxygen_fragments[1] = TextFragment::new(player.oxygen.to_string());
    }
}

impl Drawable for StatusText {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        self.health_text.draw(ctx, param)?;

        let health_rec: Rect =
            Drawable::dimensions(&self.health_text, ctx).unwrap();
        let nutrition_dest = nalgebra::Point2::new(param.dest.x, health_rec.bottom() + 5.0);
        self.nutrition_text.draw(ctx, param.clone().dest(nutrition_dest))?;

        let nutrition_rec: Rect =
            Drawable::dimensions(&self.nutrition_text, ctx).unwrap();
        let oxygen_dest = nalgebra::Point2::new(
            param.dest.x,
            health_rec.bottom() + nutrition_rec.bottom() + 10.0);

        self.oxygen_text.draw(ctx, param.clone().dest(oxygen_dest))?;

        Ok(())
    }

    fn dimensions(&self, ctx: &mut Context) -> Option<Rect> {
        let health_rec: Rect =
            Drawable::dimensions(&self.health_text, ctx)?;
        let nutrition_rec: Rect =
            Drawable::dimensions(&self.nutrition_text, ctx)?;
        let oxygen_rec: Rect =
            Drawable::dimensions(&self.oxygen_text, ctx)?;
        Some(
            health_rec
                .combine_with(nutrition_rec)
                .combine_with(oxygen_rec)
        )
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        self.health_text.set_blend_mode(mode);
        self.nutrition_text.set_blend_mode(mode);
        self.oxygen_text.set_blend_mode(mode);
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        self.health_text.blend_mode()
    }
}


#[derive(Copy,Clone)]
pub enum UserAction {
    PlayerDo(CharacterAction),
    QuitGame,
}
