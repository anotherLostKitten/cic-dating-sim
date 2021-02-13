extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::{Instant,Duration};
use std::path::Path;
use sdl2::render::*;
use sdl2::rect::*;
use sdl2::ttf::Font;

mod render;

static WIDTH: u32 = 1600;
static FRAMERATE: u32 = 30;

fn main() {
    let ctx = sdl2::init().unwrap();
    let vss = ctx.video().unwrap();
    let ttfctx = sdl2::ttf::init().unwrap();
    let window = vss.window("CIC dating", WIDTH, WIDTH / 16 * 9).position_centered().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut events = ctx.event_pump().unwrap();
    let texture_creator = canvas.texture_creator();
    let font = ttfctx.load_font("data/montserrat.otf", 50).unwrap();
    let width = WIDTH; //cfg file eventually, maybe
    
    let mut renderer = render::Renderer::init(canvas, &texture_creator, width, font);
    let mut i = 0;
    'running: loop {
        let begin = Instant::now();
        i = i%200 + 1;
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        renderer.render(&texture_creator, 0, 0, &"i have a bendyback, this is so sad"[0..i/6 + 1]);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000 / FRAMERATE - begin.elapsed().as_millis() as u32));
    }
}
