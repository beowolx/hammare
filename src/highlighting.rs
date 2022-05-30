use termion::color;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Type {
    None,
    Number,
    Match,
    String,
    Character,
    Comment,
    PrimaryKeywords,
    SecondaryKeywords
}
impl Type {
    pub fn to_color(self) -> impl color::Color {
        match self {
            Type::Number => color::Rgb(189, 147, 249),
            Type::Match => color::Rgb(38, 139, 210),
            Type::String => color::Rgb(241, 250, 140),
            Type::Character => color::Rgb(108, 113, 196),
            Type::Comment => color::Rgb(98, 114, 164),
            Type::PrimaryKeywords => color::Rgb(255, 121, 198),
            Type::SecondaryKeywords => color::Rgb(139, 233, 253),
            Type::None => color::Rgb(255, 255, 255),
        }
    }
}
