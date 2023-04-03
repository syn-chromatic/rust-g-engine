use crate::components::vectors::Vector3D;
use speedy2d::color::Color;

#[derive(Clone, Debug, PartialEq)]
pub struct RGBA {
    red: f64,
    green: f64,
    blue: f64,
    alpha: f64,
}

impl RGBA {
    pub fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        RGBA {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn rgb_tuple(&self) -> (f64, f64, f64) {
        (self.red, self.green, self.blue)
    }

    pub fn rgb_tuple_u8(&self) -> (u8, u8, u8) {
        let red: u8 = (self.red * 255.0) as u8;
        let green: u8 = (self.green * 255.0) as u8;
        let blue: u8 = (self.blue * 255.0) as u8;
        (red, green, blue)
    }

    pub fn rgba_tuple(&self) -> (f64, f64, f64, f64) {
        (self.red, self.green, self.blue, self.alpha)
    }

    pub fn from_rgb(red: f64, green: f64, blue: f64) -> Self {
        RGBA::new(red, green, blue, 1.0)
    }

    pub fn from_vector(vector: Vector3D) -> Self {
        RGBA::new(vector.x, vector.y, vector.z, 1.0)
    }

    pub fn from_rgba_tuple(rgba: (f64, f64, f64, f64)) -> Self {
        RGBA::new(rgba.0, rgba.1, rgba.2, rgba.3)
    }

    pub fn from_rgb_tuple(rgb: (f64, f64, f64)) -> Self {
        RGBA::new(rgb.0, rgb.1, rgb.2, 1.0)
    }

    pub fn multiply(&self, color: &RGBA) -> RGBA {
        let red: f64 = self.red * color.red;
        let green: f64 = self.green * color.green;
        let blue: f64 = self.blue * color.blue;
        let alpha: f64 = self.alpha * color.alpha;
        RGBA::new(red, green, blue, alpha)
    }

    pub fn average(&self, color: &RGBA) -> RGBA {
        let red: f64 = (self.red + color.red) / 2.0;
        let green: f64 = (self.green + color.green) / 2.0;
        let blue: f64 = (self.blue + color.blue) / 2.0;
        let alpha: f64 = 1.0;
        RGBA::new(red, green, blue, alpha)
    }

    pub fn interpolate(&self, color: &RGBA, weight: (f64, f64)) -> RGBA {
        let red = self.red * weight.0 + color.red * weight.1;
        let green = self.green * weight.0 + color.green * weight.1;
        let blue = self.blue * weight.0 + color.blue * weight.1;
        let alpha = 1.0;
        RGBA::new(red, green, blue, alpha)
    }

    pub fn to_vector_rgb(&self) -> Vector3D {
        let red: f64 = self.red;
        let green: f64 = self.green;
        let blue: f64 = self.blue;
        let vector_rgb: Vector3D = Vector3D::new(red, green, blue);
        vector_rgb
    }

    pub fn to_sp2d_color(&self) -> Color {
        let red: f32 = self.red as f32;
        let green: f32 = self.green as f32;
        let blue: f32 = self.blue as f32;
        let alpha: f32 = self.alpha as f32;

        let color: Color = Color::from_rgba(red, green, blue, alpha);
        color
    }
}
