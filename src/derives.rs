use std::collections::HashSet;
use std::fmt::{Display, Formatter, Error};
use std::iter::FromIterator;

#[derive(Debug, Default, Clone)]
pub struct Derives(HashSet<Derive>);

impl Derives {
    pub fn debug() -> Self {
        let mut derives = Self::default();
        derives.insert(Derive::Debug);
        derives
    }

    pub fn debug_default() -> Self {
        let mut derives = Self::default();
        derives.insert(Derive::Debug);
        derives.insert(Derive::Default);
        derives
    }

    pub fn debug_default_clone() -> Self {
        let mut derives = Self::default();
        derives.insert(Derive::Debug);
        derives.insert(Derive::Default);
        derives.insert(Derive::Clone);
        derives
    }

    pub fn insert(&mut self, derive: Derive) {
        match derive {
            Derive::Ord => {
                self.insert(Derive::PartialOrd);
            },
            Derive::Eq => {
                self.insert(Derive::PartialEq);
            },
            Derive::Hash => {
                self.insert(Derive::Eq);
            },
            Derive::Copy => {
                self.insert(Derive::Clone);
            },
            _ => {},
        }

        self.0.insert(derive);
    }
}

impl FromIterator<Derive> for Derives {
    fn from_iter<T: IntoIterator<Item=Derive>>(iter: T) -> Self {
        let mut derives = Self::default();

        for d in iter {
            derives.insert(d);
        }

        derives
    }
}

impl Display for Derives {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "#[derive(").ok();

        let mut derives: Vec<Derive> = self.0.iter().copied().collect();
        derives.sort();

        for (i, d) in derives.iter().enumerate() {
            match i {
                0 => write!(f, "{}", d),
                _ => write!(f, ", {}", d),
            }.ok();
        }

        writeln!(f, ")]")
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Derive {
    Debug,
    Default,
    Copy,
    Clone,
    Eq, PartialEq,
    Ord, PartialOrd,
    Hash,
}

impl Display for Derive {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                Derive::Debug => "Debug",
                Derive::Default => "Default",
                Derive::Copy => "Copy",
                Derive::Clone => "Clone",
                Derive::Eq => "Eq",
                Derive::PartialEq => "PartialEq",
                Derive::Ord => "Ord",
                Derive::PartialOrd => "PartialOrd",
                Derive::Hash => "Hash",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_test() {
        assert_eq!(
            "#[derive(Debug, Default, Clone)]\n",
            Derives::debug_default_clone().to_string()
        );
    }

    #[test]
    fn hash_implies_eq_and_partial_eq() {
        let mut hash = Derives::default();
        hash.insert(Derive::Hash);

        assert!(hash.0.contains(&Derive::Hash));
        assert!(hash.0.contains(&Derive::Eq));
        assert!(hash.0.contains(&Derive::PartialEq));
    }
}