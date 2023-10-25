#![allow(dead_code)]

use std::f64::consts::PI;

use num_traits::Float;

#[derive(Debug, PartialEq, Eq)]
enum AngleType {
    Degrees,
    Radians,
}

struct Angle<T: Float> {
    angle: T,
    angle_type: AngleType,
}

impl<T: Float> Angle<T> {
    fn square(at: AngleType) -> Self {
        Self {
            angle: match at {
                AngleType::Degrees => T::from(90.0).unwrap(),
                AngleType::Radians => T::from(PI / 2.0).unwrap(),
            },
            angle_type: at,
        }
    }

    fn to_degrees(&self) -> T {
        match self.angle_type {
            AngleType::Degrees => self.angle,
            AngleType::Radians => self.angle.to_degrees(),
        }
    }

    fn to_radians(&self) -> T {
        match self.angle_type {
            AngleType::Degrees => self.angle.to_radians(),
            AngleType::Radians => self.angle,
        }
    }
}

impl<T> From<(T, AngleType)> for Angle<T>
where
    T: Float,
{
    fn from((angle, angle_type): (T, AngleType)) -> Self {
        Self {
            angle: angle.into(),
            angle_type,
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn test_from() {
        let angle = Angle::<f32>::from((90.0, AngleType::Degrees));
        assert_eq!(angle.angle, 90.0);
        assert_eq!(angle.angle_type, AngleType::Degrees);

        let angle: Angle<f64> = (PI, AngleType::Radians).into();
        assert_eq!(angle.angle, PI);
        assert_eq!(angle.angle_type, AngleType::Radians);
    }

    #[test]
    fn test_to_degrees() {
        let angle = Angle::<f32>::from((PI as f32, AngleType::Radians));
        assert_eq!(angle.to_degrees(), 180.0);

        let angle = Angle::<f64>::from((PI, AngleType::Radians));
        assert_eq!(angle.to_degrees(), 180.0);
    }

    #[test]
    fn test_to_radians() {
        let angle = Angle::<f32>::from((180.0, AngleType::Degrees));
        assert_eq!(angle.to_radians(), PI as f32);

        let angle = Angle::<f64>::from((180.0, AngleType::Degrees));
        assert_eq!(angle.to_radians(), PI);
    }

    #[test]
    fn test_square() {
        let angle = Angle::<f32>::square(AngleType::Degrees);
        assert_eq!(angle.angle, 90.0);
        assert_eq!(angle.angle_type, AngleType::Degrees);

        let angle = Angle::<f64>::square(AngleType::Radians);
        assert_eq!(angle.angle, PI / 2.0);
        assert_eq!(angle.angle_type, AngleType::Radians);
    }
}
