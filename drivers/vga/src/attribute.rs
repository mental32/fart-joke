/// Various VGA colors that may be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    /// Black
    Black = 0,

    /// Blue
    Blue = 1,

    /// Green
    Green = 2,

    /// Cyan
    Cyan = 3,

    /// Red
    Red = 4,

    /// Magenta
    Magenta = 5,

    /// Browm
    Brown = 6,

    /// Light gray
    LightGray = 7,

    /// Dark gray
    DarkGray = 8,

    /// Light blue
    LightBlue = 9,

    /// Light green
    LightGreen = 10,

    /// Light cyan
    LightCyan = 11,

    /// Light red
    LightRed = 12,

    /// Pink
    Pink = 13,

    /// Yellow
    Yellow = 14,

    /// White
    White = 15,
}

const COLORS: &[Color; 8] = &[
    Color::Black,
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Magenta,
    Color::Cyan,
    Color::White,
];

impl Color {
    /// Get a color from an index.
    pub fn from_usize(other: usize) -> Option<Self> {
        if other >= COLORS.len() {
            None
        } else {
            Some(COLORS[other])
        }
    }
}

/// A character attribute describes the foreground and background colors to use
/// When rendering the character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Attribute(u8);

impl Attribute {
    /// Create a new Attribute.
    ///
    /// # Examples
    ///
    /// ```
    /// let attr = Attribute::new(Color::White, Color::Black);
    /// ```
    pub fn new(foreground: Color, background: Color) -> Self {
        Attribute((background as u8) << 4 | (foreground as u8))
    }

    /// Set the foreground color.
    pub fn set_foreground(&mut self, foreground: Color) {
        self.0 |= foreground as u8;
    }

    /// Set the background color.
    pub fn set_background(&mut self, background: Color) {
        self.0 = (background as u8) << 4 | ((self.0 & 0x0F) as u8)
    }

    /// Helper to create a white foreground on a black background.
    pub fn default() -> Self {
        Attribute((Color::Black as u8) << 4 | (Color::White as u8))
    }

    /// Helper to copy another attribute.
    pub fn same(color: Color) -> Self {
        Self::new(color, color)
    }
}
