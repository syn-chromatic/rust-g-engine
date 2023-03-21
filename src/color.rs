use speedy2d::color::Color;

#[derive(Clone, Debug, Copy)]
pub struct RGBA {
    red: f64,
    green: f64,
    blue: f64,
    alpha: f64,
}

impl RGBA {
    pub fn new(red: f64, green: f64, blue: f64, alpha: f64) -> RGBA {
        RGBA {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn from_rgba_tuple(rgba: (f64, f64, f64, f64)) -> RGBA {
        let red: f64 = rgba.0;
        let green: f64 = rgba.1;
        let blue: f64 = rgba.2;
        let alpha: f64 = rgba.3;

        RGBA {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn from_rgb_tuple(rgb: (f64, f64, f64)) -> RGBA {
        let red: f64 = rgb.0;
        let green: f64 = rgb.1;
        let blue: f64 = rgb.2;
        let alpha: f64 = 1.0;

        RGBA {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn get_rgb_tuple(&self) -> (f64, f64, f64) {
        let rgb_tuple: (f64, f64, f64) = (self.red, self.green, self.blue);
        rgb_tuple
    }

    pub fn get_rgba_tuple(&self) -> (f64, f64, f64, f64) {
        let rgba_tuple: (f64, f64, f64, f64) = (self.red, self.green, self.blue, self.alpha);
        rgba_tuple
    }

    pub fn get_speedy2d_color(&self) -> Color {
        let red: f32 = self.red as f32;
        let green: f32 = self.green as f32;
        let blue: f32 = self.blue as f32;
        let alpha: f32 = self.alpha as f32;

        let color: Color = Color::from_rgba(red, green, blue, alpha);
        color
    }
}
