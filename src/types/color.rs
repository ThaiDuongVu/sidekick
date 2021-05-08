use std::fmt;
use std::ops;

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl fmt::Display for Color {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "({}, {}, {}, {})",
            self.r, self.g, self.b, self.a
        )
    }
}

impl Copy for Color {}

impl Clone for Color {
    fn clone(&self) -> Color {
        *self
    }
}

// Mathematical operator overloadings for Color type on Color type
impl ops::Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Color {
        return Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        };
    }
}
impl ops::Sub<Color> for Color {
    type Output = Color;
    fn sub(self, rhs: Color) -> Color {
        return Color {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
            a: self.a - rhs.a,
        };
    }
}
impl ops::Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        return Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a * rhs.a,
        };
    }
}
impl ops::Div<Color> for Color {
    type Output = Color;
    fn div(self, rhs: Color) -> Color {
        return Color {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
            a: self.a / rhs.a,
        };
    }
}

// Mathematical operator overloadings for Color type on f32 type
impl ops::Add<f32> for Color {
    type Output = Color;
    fn add(self, rhs: f32) -> Color {
        return Color {
            r: self.r + rhs,
            g: self.g + rhs,
            b: self.b + rhs,
            a: self.a + rhs,
        };
    }
}
impl ops::Sub<f32> for Color {
    type Output = Color;
    fn sub(self, rhs: f32) -> Color {
        return Color {
            r: self.r - rhs,
            g: self.g - rhs,
            b: self.b - rhs,
            a: self.a - rhs,
        };
    }
}
impl ops::Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Color {
        return Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a * rhs,
        };
    }
}
impl ops::Div<f32> for Color {
    type Output = Color;
    fn div(self, rhs: f32) -> Color {
        return Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
            a: self.a / rhs,
        };
    }
}

// Logical operator overloadings for type Color
impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        return self.r == other.r && self.g == other.g;
    }
}

impl Color {
    // Default constructor to initialize Color
    pub fn new() -> Self {
        return Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        };
    }

    // Unit colors
    pub fn red() -> Self {
        return Self {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        };
    }
    pub fn green() -> Self {
        return Self {
            r: 0.0,
            g: 1.0,
            b: 0.0,
            a: 1.0,
        };
    }
    pub fn blue() -> Self {
        return Self {
            r: 0.0,
            g: 0.0,
            b: 1.0,
            a: 1.0,
        };
    }
    pub fn yellow() -> Self {
        return Self {
            r: 1.0,
            g: 1.0,
            b: 0.0,
            a: 1.0,
        };
    }
    pub fn pink() -> Self {
        return Self {
            r: 1.0,
            g: 0.0,
            b: 1.0,
            a: 1.0,
        };
    }
    pub fn teal() -> Self {
        return Self {
            r: 0.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        };
    }
    pub fn black() -> Self {
        return Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        };
    }
    pub fn white() -> Self {
        return Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        };
    }

    // Color normalized so that its RGB values are between 0 and 1
    pub fn normalized(&mut self) -> Color {
        return Color {
            r: if self.r > 1.0 {
                1.0
            } else if self.r < 0.0 {
                0.0
            } else {
                self.r
            },
            g: if self.g > 1.0 {
                1.0
            } else if self.g < 0.0 {
                0.0
            } else {
                self.g
            },
            b: if self.b > 1.0 {
                1.0
            } else if self.b < 0.0 {
                0.0
            } else {
                self.b
            },
            a: if self.a > 1.0 {
                1.0
            } else if self.a < 0.0 {
                0.0
            } else {
                self.a
            },
        };
    }
}
