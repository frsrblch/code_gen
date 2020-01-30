use crate::Generics;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Error};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TypeName(pub String);

impl TypeName {
    pub fn new(s: &str) -> Self {
        TypeName::from_str(s).unwrap()
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl FromStr for TypeName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("TypeName: input cannot be empty".to_string());
        }

        if s.contains(' ') {
            return Err(format!("TypeName: input cannot contain spaces: {}", s));
        }

        Ok(TypeName(s.to_string()))
    }
}

impl Display for TypeName {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Type {
    pub name: TypeName,
    pub types: Generics,
}

impl Type {
    pub fn new(s: &str) -> Self {
        Self::from_str(s).unwrap()
    }
}

impl FromStr for Type {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.find('<') {
            None => s.parse()
                .map(|name| Type { name, types: Generics::none() }),
            Some(i) => {
                let name = s[0..i].parse()?;
                let types = s[i..].parse()?;
                Ok(Type { name, types })
            },
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}{}", self.name, self.types)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn parse_test() {
        let ty: Type = "Test".parse().unwrap();

        assert_eq!(ty, Type { name: "Test".parse().unwrap(), types: Generics::none() })
    }

    #[test]
    pub fn parse_test_with_generic() {
        let ty: Type = "Test<ID, T>".parse().unwrap();

        assert_eq!(ty, Type { name: "Test".parse().unwrap(), types: Generics::two(Type::from_str("ID").unwrap(), Type::from_str("T").unwrap()) })
    }
}
