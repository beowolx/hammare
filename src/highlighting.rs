use termion::color;

#[derive(PartialEq)]
pub enum Type {
    None,
    Number,
}

impl Type {
    pub fn to_color(&self) -> impl color::Color {
        match *self {
            Type::Number => color::Rgb(189, 147, 249),
            Type::None => color::Rgb(255, 255, 255),
        }
    }
}
