use termion::color;

#[derive(PartialEq)]
pub enum Type {
    None,
    Number,
    Match,
    String,
}

impl Type {
    pub fn to_color(&self) -> impl color::Color {
        match *self {
            Type::Number => color::Rgb(189, 147, 249),
            Type::Match => color::Rgb(38, 139, 210),
            Type::String => color::Rgb(241, 250, 140),
            Type::None => color::Rgb(255, 255, 255),
        }
    }
}
