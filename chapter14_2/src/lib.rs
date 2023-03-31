//! # Art
//!
//! A library for mixing colors

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    #[derive(PartialEq)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    #[derive(PartialEq, Debug)]
    pub enum SecondaryColor {
        Green,
        Orange,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        if c1 == PrimaryColor::Red && c2 == PrimaryColor::Blue
            || c1 == PrimaryColor::Blue && c2 == PrimaryColor::Red
        {
            SecondaryColor::Purple
        } else if c1 == PrimaryColor::Red && c2 == PrimaryColor::Yellow
            || c1 == PrimaryColor::Yellow && c2 == PrimaryColor::Red
        {
            SecondaryColor::Orange
        } else {
            SecondaryColor::Green
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        kinds::{PrimaryColor, SecondaryColor},
        utils::mix,
    };

    #[test]
    fn blue_and_red() {
        let c1 = PrimaryColor::Red;
        let c2 = PrimaryColor::Blue;

        let c3 = mix(c1, c2);
        assert_eq!(c3, SecondaryColor::Purple);
    }

    #[test]
    fn yellow_and_red() {
        let c1 = PrimaryColor::Red;
        let c2 = PrimaryColor::Yellow;

        let c3 = mix(c1, c2);
        assert_eq!(c3, SecondaryColor::Orange);
    }

    #[test]
    fn blue_and_yellow() {
        let c1 = PrimaryColor::Yellow;
        let c2 = PrimaryColor::Blue;

        let c3 = mix(c1, c2);
        assert_eq!(c3, SecondaryColor::Green);
    }
}
