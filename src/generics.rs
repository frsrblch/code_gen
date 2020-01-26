use std::fmt::{Display, Formatter, Error};
use std::iter::FromIterator;

#[derive(Debug, Default, Clone)]
pub struct Generics(Vec<String>);

impl Generics {
    pub fn none() -> Self { Default::default() }

    pub fn one(t: &str) -> Self {
        Generics(vec![t.to_string()])
    }

    pub fn two(t: &str, u: &str) -> Self {
        Generics(vec![t.to_string(), u.to_string()])
    }
}

impl<'a> FromIterator<&'a str> for Generics {
    fn from_iter<T: IntoIterator<Item=&'a str>>(iter: T) -> Self {
        Generics(iter.into_iter().map(String::from).collect())
    }
}

impl Display for Generics {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if self.0.len() == 0 {
            return Ok(());
        }

        write!(f, "<").ok();

        for (i, g) in self.0.iter().enumerate() {
            match i {
                0 => write!(f, "{}", g).ok(),
                _ => write!(f, ", {}", g).ok(),
            };
        }

        write!(f, ">")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let g = Generics::none();

        assert_eq!("", g.to_string());
    }

    #[test]
    fn one() {
        let g = Generics::two("ID", "T");

        assert_eq!("<ID, T>", g.to_string());
    }
}