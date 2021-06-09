use std::fmt;
use std::ops;

/// A type that holds 2 values (x and y)
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl fmt::Display for Vector2 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "({}, {})", self.x, self.y)
    }
}

impl Copy for Vector2 {}

impl Clone for Vector2 {
    fn clone(&self) -> Vector2 {
        *self
    }
}

// Mathematical operator overloadings for Vector type on Vector type
impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Vector2) -> Vector2 {
        return Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}
impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Vector2) -> Vector2 {
        return Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}
impl ops::Mul<Vector2> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Vector2 {
        return Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        };
    }
}
impl ops::Div<Vector2> for Vector2 {
    type Output = Vector2;
    fn div(self, rhs: Vector2) -> Vector2 {
        return Vector2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        };
    }
}
impl ops::AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        *self = Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl ops::SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Vector2) {
        *self = Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl ops::MulAssign for Vector2 {
    fn mul_assign(&mut self, rhs: Vector2) {
        *self = Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl ops::DivAssign for Vector2 {
    fn div_assign(&mut self, rhs: Vector2) {
        *self = Vector2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
impl ops::Neg for Vector2 {
    type Output = Vector2;
    fn neg(self) -> Vector2 {
        return Vector2 {
            x: -self.x,
            y: -self.y,
        };
    }
}

// Mathematical operator overloadings for Vector type on f32 type
impl ops::Add<f32> for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: f32) -> Vector2 {
        return Vector2 {
            x: self.x + rhs,
            y: self.y + rhs,
        };
    }
}
impl ops::Sub<f32> for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: f32) -> Vector2 {
        return Vector2 {
            x: self.x - rhs,
            y: self.y - rhs,
        };
    }
}
impl ops::Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f32) -> Vector2 {
        return Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}
impl ops::Div<f32> for Vector2 {
    type Output = Vector2;
    fn div(self, rhs: f32) -> Vector2 {
        return Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        };
    }
}
impl ops::AddAssign<f32> for Vector2 {
    fn add_assign(&mut self, rhs: f32) {
        *self = Vector2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}
impl ops::SubAssign<f32> for Vector2 {
    fn sub_assign(&mut self, rhs: f32) {
        *self = Vector2 {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

// Logical operator overloadings for type Vector
impl PartialEq for Vector2 {
    fn eq(&self, other: &Vector2) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl Vector2 {
    /// Default constructor to initialize Vector2
    pub fn new() -> Self {
        return Self { x: 0.0, y: 0.0 };
    }

    // Unit vectors
    pub fn zero() -> Self {
        return Self { x: 0.0, y: 0.0 };
    }
    pub fn identity() -> Self {
        return Self { x: 1.0, y: 1.0 };
    }
    pub fn unit_x() -> Self {
        return Self { x: 1.0, y: 0.0 };
    }
    pub fn unit_y() -> Self {
        return Self { x: 0.0, y: 1.0 };
    }

    /// Return length of current vector
    pub fn length(&mut self) -> f32 {
        return (self.x.powi(2) + self.y.powi(2)).sqrt();
    }
    /// Normalize vector so that its length is 1
    pub fn normalized(&mut self) -> Vector2 {
        if self.x == 0.0 && self.y == 0.0 {
            return Vector2::zero();
        }
        return Vector2 {
            x: (1.0 / self.length()) * self.x,
            y: (1.0 / self.length()) * self.y,
        };
    }
    /// Return distance from current vector to another vector
    pub fn distance(&mut self, other: Vector2) -> f32 {
        return ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt();
    }
    /// Return dot product of current vector and another vector
    pub fn dot(&mut self, other: Vector2) -> f32 {
        return self.x * other.x + self.y * other.y;
    }
    /// Translate current vector with another vector
    pub fn translate(&mut self, difference: Vector2) {
        self.x += difference.x;
        self.y += difference.y;
    }
}
