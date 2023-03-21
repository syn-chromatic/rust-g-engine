use std::iter::Cloned;

use crate::color::RGBA;
use speedy2d::font::Font as FontSP2D;
use speedy2d::font::FormattedTextBlock;
use speedy2d::font::{TextLayout, TextOptions};
use speedy2d::{dimen::Vec2, Graphics2D};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Font {
    font: FontSP2D,
    font_type: String,
    font_size: u32,
    font_style: String,
    font_color: RGBA,
    line_height: f32,
    padding_percent: u32,
}

impl Font {
    pub fn new(
        font_type: String,
        font_size: u32,
        font_style: String,
        font_color: RGBA,
        line_height: f32,
        padding_percent: u32,
    ) -> Font {
        let bytes: &[u8; 367112] = include_bytes!("../fonts/arial.ttf");
        let font: FontSP2D = FontSP2D::new(bytes).unwrap();

        Font {
            font,
            font_type,
            font_size,
            font_style,
            font_color,
            line_height,
            padding_percent,
        }
    }

    pub fn get_font_tuple(&self) -> (String, u32, String) {
        let font_type: String = self.font_type.clone();
        let font_style: String = self.font_style.clone();
        let font_tuple: (String, u32, String) = (font_type, self.font_size, font_style);
        font_tuple
    }

    pub fn get_font_color(&self) -> RGBA {
        let font_color = self.font_color;
        font_color
    }
}

pub struct TextWriter {
    width: u32,
    height: u32,
    font: Font,
    tl_column: Vec<(String, Option<Font>)>,
}

impl TextWriter {
    pub fn new(width: u32, height: u32, font: Font) -> TextWriter {
        let tl_column: Vec<(String, Option<Font>)> = vec![];
        TextWriter {
            width,
            height,
            font,
            tl_column,
        }
    }

    pub fn add_text_top_left(&mut self, text: String, font: Option<Font>) {
        self.tl_column.push((text, font));
    }

    pub fn draw(&mut self, graphics: &mut Graphics2D) {
        for (idx, (text, font)) in self.tl_column.iter().enumerate() {
            let font = font.as_ref().unwrap_or(&self.font);
            let text_xy = self.get_text_xy(font, idx);
            let font_rgba = &font.font_color;
            let font_color = font_rgba.get_speedy2d_color();
            let text_position = Vec2::new(text_xy.0, text_xy.1);
            let text_block = self.get_text_block(text);
            graphics.draw_text(text_position, font_color, &text_block);
        }
        self.tl_column = vec![];
    }

    fn get_text_block(&self, text: &String) -> Rc<FormattedTextBlock> {
        let text_options: TextOptions = TextOptions::new();
        let text_scale: f32 = self.font.font_size as f32;
        let text_block: Rc<FormattedTextBlock> =
            self.font.font.layout_text(text, text_scale, text_options);
        text_block
    }

    fn get_padded_top_left_corner(&self, padding_x: f32, padding_y: f32) -> (f32, f32) {
        let width: f32 = self.width as f32;
        let height: f32 = self.height as f32;

        let top_left_x: f32 = (-width / 2.0) + padding_x;
        let top_left_y: f32 = (width / 2.0) - padding_y;
        let tl_tuple: (f32, f32) = (top_left_x, top_left_y);
        tl_tuple
    }

    fn get_text_xy(&self, font: &Font, idx: usize) -> (f32, f32) {
        let width: f32 = self.width as f32;
        let height: f32 = self.height as f32;

        let font_size = font.font_size;
        let line_height = font.line_height;
        let padding_percent = font.padding_percent;
        let padding_x = width as f32 * (padding_percent as f32 / 100.0);
        let padding_y = height as f32 * (padding_percent as f32 / 100.0);

        let text_x = padding_x;
        let text_y = (font_size as f32 * line_height * idx as f32) + (padding_y / line_height);

        let text_xy = (text_x, text_y);
        text_xy
    }
}
