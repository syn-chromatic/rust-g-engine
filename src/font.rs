use speedy2d::font::Font;
use speedy2d::font::FormattedTextBlock;
use speedy2d::font::TextLayout;
use speedy2d::font::TextOptions;
use std::rc::Rc;

use crate::color::RGBA;

pub trait FontTrait {
    fn get_sp2d_font(&self) -> &Font;
}

#[derive(Clone, Debug)]
pub enum FontType {
    ArialFont(ArialFont),
}

impl FontTrait for FontType {
    fn get_sp2d_font(&self) -> &Font {
        match self {
            FontType::ArialFont(f) => f.get_sp2d_font(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ArialFont {
    font: Font,
}

impl FontTrait for ArialFont {
    fn get_sp2d_font(&self) -> &Font {
        &self.font
    }
}

impl ArialFont {
    pub fn new() -> ArialFont {
        let bytes: &[u8; 367112] = include_bytes!("../fonts/arial.ttf");
        let font: Font = Font::new(bytes).unwrap();

        ArialFont { font }
    }
}

#[derive(Debug)]
pub struct FontSettings {
    pub font_type: FontType,
    pub font_size: u32,
    pub font_color: RGBA,
    pub line_height: f32,
    pub padding_percent: u32,
}

impl FontSettings {
    pub fn new(
        font_type: FontType,
        font_size: u32,
        font_color: RGBA,
        line_height: f32,
        padding_percent: u32,
    ) -> FontSettings {
        FontSettings {
            font_type,
            font_size,
            font_color,
            line_height,
            padding_percent,
        }
    }

    pub fn get_text_block(&self, text: &String) -> Rc<FormattedTextBlock> {
        let text_options: TextOptions = TextOptions::new();
        let text_scale: f32 = self.font_size as f32;
        let sp2d_font: &Font = self.font_type.get_sp2d_font();
        let text_block: Rc<FormattedTextBlock> =
            sp2d_font.layout_text(text, text_scale, text_options);
        text_block
    }
}
