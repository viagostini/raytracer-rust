use float_cmp::approx_eq;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        approx_eq!(f64, self.red, other.red)
            && approx_eq!(f64, self.green, other.green)
            && approx_eq!(f64, self.blue, other.blue)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Color) -> Self::Output {
        Self::Output {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

#[cfg(test)]
mod color_tests {
    use super::*;
    #[test]
    fn add_colors() {
        let c1 = Color {
            red: 3.0,
            green: -2.0,
            blue: 5.0,
        };

        let c2 = Color {
            red: -2.0,
            green: 3.0,
            blue: 1.0,
        };

        let expected = Color {
            red: 1.0,
            green: 1.0,
            blue: 6.0,
        };

        assert_eq!(c1 + c2, expected);
    }

    #[test]
    fn sub_colors() {
        let c1 = Color {
            red: 3.0,
            green: 2.0,
            blue: 1.0,
        };

        let c2 = Color {
            red: 5.0,
            green: 6.0,
            blue: 7.0,
        };

        let expected = Color {
            red: -2.0,
            green: -4.0,
            blue: -6.0,
        };

        assert_eq!(c1 - c2, expected);
    }

    #[test]
    fn multiply_by_f64() {
        let color = Color {
            red: 1.0,
            green: -2.0,
            blue: 3.0,
        };

        let expected = Color {
            red: 3.0,
            green: -6.0,
            blue: 9.0,
        };

        assert_eq!(color * 3.0, expected);
    }

    #[test]
    fn divide_by_f64() {
        let color = Color {
            red: 1.0,
            green: -2.0,
            blue: 3.0,
        };

        let expected = Color {
            red: (1.0 / 3.0),
            green: (-2.0 / 3.0),
            blue: 1.0,
        };

        assert_eq!(color / 3.0, expected);
    }
}
