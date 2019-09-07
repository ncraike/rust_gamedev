use ggez::{Context, ContextBuilder, GameResult};
use ggez::conf::Conf;
use ggez::event::{EventHandler, run, quit};
use ggez::input::keyboard::{KeyCode, KeyMods};

mod ui;
mod entities;

pub struct GameState {
    world: entities::GameWorld,
    ui: ui::UI,
}

impl GameState {
    pub fn new() -> GameState {
        let world = entities::GameWorld::new();
        let ui = ui::UI::new(&world).unwrap();
        return GameState {world, ui};
    }

    pub fn run(&mut self) {
        let (ref mut ctx, ref mut event_loop) =
            ContextBuilder::new("hello_ggez", "Nathan")
            .conf(Conf::new())
            .build()
            .unwrap();
        run(ctx, event_loop, self).unwrap()
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.ui.update(ctx, &mut self.world).unwrap();
        if self.ui.should_quit() || self.world.is_finished() {
            quit(ctx);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.ui.draw(ctx)
    }

    fn key_up_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
    ) {
        self.ui.key_up_event(ctx, keycode, keymods);
    }
}
