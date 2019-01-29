extern crate ggez;

use ggez::*;
use ggez::graphics::{Color, DrawMode, DrawParam};
use ggez::event::{KeyCode, KeyMods};

#[derive(Copy, Clone)]
enum Tile {
    Empty,
    Bomb,
    HiddenEmpty,
    FlagBomb,
    FlagEmpty,
}

struct State {
    dt: std::time::Duration,
    font: graphics::Font,
    board: [[Tile; 10]; 10],
    px: usize,
    py: usize,
    needs_redraw: bool,
}

impl State {
    pub fn new(ctx: &mut Context) -> GameResult<State> {
        Ok(State { 
            dt: std::time::Duration::new(0, 0),
            font: graphics::Font::new(ctx, "/Oxygen-Sans.ttf")?,
            board: State::random_board(),
            px: 0,
            py: 0,
            needs_redraw: true,
        })
    }

    fn random_board() -> [[Tile; 10]; 10] {
        let mut board = [[Tile::HiddenEmpty; 10]; 10];
        for i in 0..10 {
            for j in 0..10 {
                if rand::random() && rand::random() {
                    board[i][j] = Tile::Bomb;
                }
            }
        }
        board
    }

    fn inbounds(x: usize, y: usize) -> bool {
        x > 0 && y > 0 && x < 11 && y < 11
    }

    fn bomb_count_around(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for i in 0 .. 3 {
            for j in 0 .. 3 {
                if State::inbounds(x + i, y + j) {
                    println!("Checking {}, {}", x+i, y+j);
                    count += match self.board[x+i-1][y+j-1] {
                        Tile::Bomb => 1,
                        _ => 0,
                    }
                }else{
                    println!("(will be) Out of bounds: {}, {}", x+i, y+j);
                }
            }
        }
        println!("Bombs around ({}, {}) = {}", x, y, count);
        count
    }

    fn draw_tile(&self, ctx: &mut Context, x: usize, y: usize) -> GameResult<()> {
        let mut color = DARKBLUE;
        if x == self.px && y == self.py {
            color = BLUE;
        }

        match self.board[x][y] {
            Tile::Empty =>  {
                let r = graphics::Rect::new(rect_pos(x), rect_pos(y), 30.0, 30.0);
                let rm = graphics::Mesh::new_rectangle(ctx, DrawMode::Fill, r, color)?;
                graphics::draw(ctx, &rm, DrawParam::default())?;
                let num = self.bomb_count_around(x, y).to_string();
                let text = graphics::Text::new((num, self.font, 30.0));
                graphics::draw(ctx, &text, ([rect_pos(x)+7.0, rect_pos(y)+2.0], 0.0, graphics::WHITE))
            }
            Tile::Bomb => {
                let r = graphics::Rect::new(rect_pos(x), rect_pos(y), 30.0, 30.0);
                let rm = graphics::Mesh::new_rectangle(ctx, DrawMode::Fill, r, color)?;  //bomb is still hidden
                graphics::draw(ctx, &rm, DrawParam::default())
            }
            Tile::HiddenEmpty => {
                let r = graphics::Rect::new(rect_pos(x), rect_pos(y), 30.0, 30.0);
                let rm = graphics::Mesh::new_rectangle(ctx, DrawMode::Fill, r, color)?;
                graphics::draw(ctx, &rm, DrawParam::default())
            }
        }
    }

    fn update_pos(&mut self, ctx: &mut Context, new_x: usize, new_y: usize) -> GameResult<()> {
        self.px += 1;
        self.draw_tile(ctx, self.px - 1, self.py);
        self.px = new_x;
        self.py = new_y;
        self.draw_tile(ctx, self.px, self.py);
        self.needs_redraw = true;
        Ok(())
    }
}

const BLUE: Color = graphics::Color {r: 0.1, g: 0.2, b: 0.3, a: 1.0};
const DARKBLUE: Color = graphics::Color {r: 0.025, g: 0.05, b: 0.075, a: 1.0};

fn rect_pos(k: usize) -> f32 {
    20.0 + 35.0 * k as f32
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool){
        if keycode == KeyCode::W {
            let new_y = if self.py == 0 { 0 }else{ self.py - 1 };
            self.update_pos(ctx, self.px, new_y).unwrap()
        } else if keycode == KeyCode::S {
            let new_y = if self.py == 9 { 9 }else{ self.py + 1 };
            self.update_pos(ctx, self.px, new_y).unwrap()
        } else if keycode == KeyCode::A {
            let new_x = if self.px == 0 { 0 }else{ self.px - 1 };
            self.update_pos(ctx, new_x, self.py).unwrap()
        } else if keycode == KeyCode::D {
            let new_x = if self.px == 9 { 9 }else{ self.px + 1 };
            self.update_pos(ctx, new_x, self.py).unwrap()

        } else if keycode == KeyCode::R {  // reveal bomb
            match self.board[self.px][self.py] {
                Tile::HiddenEmpty => {
                    self.board[self.px][self.py] = Tile::Empty;
                    self.update_pos(ctx, self.px, self.py).unwrap()
                },
                Tile::Bomb => {
                    let r = graphics::Rect::new(rect_pos(self.px), rect_pos(self.py), 30.0, 30.0);
                    let rm = graphics::Mesh::new_rectangle(ctx, DrawMode::Fill, r, graphics::Color::new(1.0, 0.0, 0.0, 1.0)).unwrap();
                    graphics::draw(ctx, &rm, DrawParam::default());
                    graphics::present(ctx).unwrap();
                },
                Tile::Empty => {},
            }

        } else if keycode == KeyCode::F { // flag bomb
            let r = graphics::Rect::new(rect_pos(self.px), rect_pos(self.py), 30.0, 30.0);
            let rm = graphics::Mesh::new_rectangle(ctx, DrawMode::Fill, r, graphics::Color::new(0.3, 0.0, 0.0, 1.0)).unwrap();
            graphics::draw(ctx, &rm, DrawParam::default());
            graphics::present(ctx).unwrap();
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.needs_redraw {
            graphics::clear(ctx, graphics::WHITE);

            for i in 0..10 {
                for j in 0..10 {
                    self.draw_tile(ctx, i, j)?;
                }
            }

            graphics::present(ctx)?;
            self.needs_redraw = false;
            ggez::timer::yield_now();
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

    let mut state = State::new(ctx)?;
    event::run(ctx, event_loop, &mut state).unwrap();
    Ok(())
}
