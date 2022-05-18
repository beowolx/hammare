use termion::color;

#[derive(PartialEq)]
pub enum Type {
    None,
    Number,
    Match,
}

impl Type {
    pub fn to_color(&self) -> impl color::Color {
        match *self {
            Type::Number => color::Rgb(189, 147, 249),
            Type::Match => color::Rgb(38, 139, 210),
            Type::None => color::Rgb(255, 255, 255),
        }
    }
}
