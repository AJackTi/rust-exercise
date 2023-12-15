use draw::to_coord;
use game::Game;
use piston_window::types::Color;
use piston_window::*;

extern crate piston_window;
extern crate rand;

mod draw;
mod game;

const BLACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (30, 30);
    let mut window: PistonWindow = WindowSettings::new("snake", [to_coord(width), to_coord(height)])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(width, height);
    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g, _| {
            clear(BLACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|args| { game.update(args.dt) });
    }
}
