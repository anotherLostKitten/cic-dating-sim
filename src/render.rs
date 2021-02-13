use std::path::Path;

use sdl2::render::*;
use sdl2::rect::*;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;
use sdl2::pixels::Color;
use sdl2::video::WindowContext;
use sdl2::ttf::Font;

pub struct Renderer<'a> {
    canvas: WindowCanvas,
    back_texture: Texture<'a>,
    char_texture: Texture<'a>,
    back_tr: Rect,
    char_tr: Rect,
    char_rect: Rect,
    text_rect: Rect,
    font: Font<'a, 'a>,
}

impl<'a> Renderer<'a> {
    pub fn init(canvas: WindowCanvas, texture_creator: &'a TextureCreator<WindowContext>, width: u32, font: Font<'a, 'a>) -> Renderer<'a> {
        Renderer {
            canvas: canvas,
            back_texture: texture_creator.create_texture_from_surface(Surface::load_bmp(Path::new("data/backgrounds.bmp")).unwrap()).unwrap(),
            back_tr: Rect::new(0, 0, 1920, 1080),
            char_texture: texture_creator.create_texture_from_surface(Surface::load_bmp(Path::new("data/characters.bmp")).unwrap()).unwrap(),
            char_tr: Rect::new(0, 0, 750, 1000),
            char_rect: Rect::new((1170*width/1920) as i32, (80*width/1920) as i32, 750*width/1920, 1000*width/1920),
            text_rect: Rect::new((210*width/1920) as i32, (850*width/1920) as i32, 1500*width/1920, 200*width/1920),
            font: font,
        }
    }
    pub fn render(&mut self, texture_creator: &'a TextureCreator<WindowContext>, back: i32, dude: i32, text: &str) {
        self.canvas.set_draw_color(Color::RGB(211, 209, 158));

        let texture_creator = self.canvas.texture_creator();
        let font_surf = self.font.render(text).blended_wrapped(Color::RGB(0, 0, 0), 1400*self.text_rect.width()/1500).unwrap();
        let font_texture = texture_creator.create_texture_from_surface(&font_surf).unwrap();
        self.char_tr.set_y(dude * 1000);
        self.char_tr.set_y(dude * 1000);

        self.canvas.fill_rect(self.text_rect); // for some reason without this line the background just renders white idk why

        self.canvas.clear();

        
        self.canvas.copy(&self.back_texture, Some(self.back_tr), None);
        self.canvas.copy(&self.char_texture, Some(self.char_tr), Some(self.char_rect));

        self.canvas.fill_rect(self.text_rect);        
        
        self.canvas.copy(&font_texture, None, Some(Rect::new(self.text_rect.x(), self.text_rect.y(), font_surf.width(), font_surf.height())));
        
        self.canvas.present();
    }
}
