use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::dimen::Vector2;
use speedy2d::font::FormattedTextBlock;
use speedy2d::Graphics2D;
use std::rc::Rc;

use crate::color::RGBA;
use crate::font::FontSettings;

pub struct TextWriter {
    resolution: (u32, u32),
    font_settings: FontSettings,
    tl_column: Vec<(String, Option<FontSettings>)>,
}

impl TextWriter {
    pub fn new(resolution: (u32, u32), font_settings: FontSettings) -> TextWriter {
        let tl_column: Vec<(String, Option<FontSettings>)> = vec![];
        TextWriter {
            resolution,
            font_settings,
            tl_column,
        }
    }

    pub fn add_text_top_left(&mut self, text: String, font_settings: Option<FontSettings>) {
        self.tl_column.push((text, font_settings));
    }

    pub fn draw(&mut self, graphics: &mut Graphics2D) {
        for (idx, (text, font_settings)) in self.tl_column.iter().enumerate() {
            let font_settings: Option<&FontSettings> = font_settings.as_ref();
            let font_settings: &FontSettings = font_settings.unwrap_or(&self.font_settings);
            let text_xy: (f32, f32) = self.get_text_xy(font_settings, idx);
            let font_rgba: &RGBA = &font_settings.font_color;
            let font_color: Color = font_rgba.to_sp2d_color();
            let text_position: Vector2<f32> = Vec2::new(text_xy.0, text_xy.1);
            let text_block: Rc<FormattedTextBlock> = self.font_settings.get_text_block(text);
            graphics.draw_text(text_position, font_color, &text_block);
        }
        self.tl_column = vec![];
    }

    fn get_text_xy(&self, font_settings: &FontSettings, idx: usize) -> (f32, f32) {
        let width: f32 = self.resolution.0 as f32;
        let height: f32 = self.resolution.1 as f32;

        let font_size: f32 = font_settings.font_size as f32;
        let line_height: f32 = font_settings.line_height;
        let padding_percent: f32 = font_settings.padding_percent as f32;
        let padding_x: f32 = width * (padding_percent / 100.0);
        let padding_y: f32 = height * (padding_percent / 100.0);

        let text_x: f32 = padding_x;
        let text_y: f32 = (font_size * line_height * idx as f32) + (padding_y / line_height);

        let text_xy: (f32, f32) = (text_x, text_y);
        text_xy
    }
}
