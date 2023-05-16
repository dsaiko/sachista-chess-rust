use std::fmt;

/// Chess piece color.
#[derive(Debug, Copy, Clone, Eq, PartialEq, strum_macros::EnumCount, strum_macros::EnumIter)]
pub enum Color {
    White = 0,
    Black,
}

impl Color {
    /// Creates piece form string.
    pub fn from_string(str: &str) -> Option<Color> {
        match str.to_ascii_lowercase().as_str() {
            "w" => Some(Color::White),
            "b" => Some(Color::Black),
            _ => None,
        }
    }

    /// Returns color of opponent pieces.
    pub fn opponent(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::White => write!(f, "w"),
            Color::Black => write!(f, "b"),
        }
    }
}
