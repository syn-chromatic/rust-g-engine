use crate::components::font::FontSettings;
use crate::components::graphics::Graphics;

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

    pub fn draw(&mut self, graphics: &mut Graphics) {
        for (idx, (text, font_settings)) in self.tl_column.iter().enumerate() {
            let font_settings: Option<FontSettings> = font_settings.clone();
            let font_settings: FontSettings = font_settings.unwrap_or(self.font_settings.clone());
            let point: (f64, f64) = self.get_text_xy(&font_settings, idx);
            graphics.draw_text(point, text.to_string(), font_settings);
        }
        self.tl_column = vec![];
    }

    fn get_text_xy(&self, font_settings: &FontSettings, idx: usize) -> (f64, f64) {
        let width: f64 = self.resolution.0 as f64;
        let height: f64 = self.resolution.1 as f64;

        let font_size: f64 = font_settings.font_size as f64;
        let line_height: f64 = font_settings.line_height as f64;
        let padding_percent: f64 = font_settings.padding_percent as f64;
        let padding_x: f64 = width * (padding_percent / 100.0);
        let padding_y: f64 = height * (padding_percent / 100.0);

        let text_x: f64 = padding_x as f64;
        let text_y: f64 = (font_size * line_height * idx as f64) + (padding_y / line_height);

        let text_xy: (f64, f64) = (text_x, text_y);
        text_xy
    }
}
