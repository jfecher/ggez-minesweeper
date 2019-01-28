extern crate ggez;

use ggez::*;


struct State {
    dt: std::time::Duration,
    font: graphics::Font,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt += timer::delta(ctx);

        let m = self.dt.subsec_millis();
        if m > 1000 / 144 {
            self.dt = std::time::Duration::new(0, 0);
            graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

            let offset = 30.0;
            let point = [offset, offset];

            let fps = ggez::timer::fps(ctx);
            let msg = format!("FPS: {}", fps);
            let text = graphics::Text::new((msg, self.font, 48.0));

            graphics::draw(ctx, &text, (point,))?;
            graphics::present(ctx)?;
        }

        Ok(())
    }
}

fn main() -> GameResult<()> {
    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("hello_ggez", "author")
        .conf(c)
        .build()
        .unwrap();

    let state = &mut State{
        dt: std::time::Duration::new(0, 0),
        font: graphics::Font::new(ctx, "/Oxygen-Sans.ttf")?
    };

    event::run(ctx, event_loop, state).unwrap();
    Ok(())
}
