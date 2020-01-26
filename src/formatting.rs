use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Error};
use std::ops::Deref;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CamelCase(String);

impl Deref for CamelCase {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for CamelCase {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.0.fmt(f)
    }
}

impl TryFrom<&str> for CamelCase {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.contains(" ") {
            return Err(format!("CamelCase cannot contain spaces: {}", value))
        }

        if value.contains("_") {
            return Err(format!("CamelCase cannot contain underscores: {}", value))
        }

        if value.is_empty() {
            return Err("CamelCase cannot be empty".to_string());
        }

        if value.chars().nth(0).unwrap().is_lowercase() {
            return Err(format!("CamelCase cannot start with lower case: {}", value));
        }

        Ok(CamelCase(value.to_string()))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SnakeCase(String);

impl Deref for SnakeCase {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for SnakeCase {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.0.fmt(f)
    }
}

impl TryFrom<&str> for SnakeCase {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err("snake_case cannot be empty".to_string());
        }

        if value.chars().any(char::is_uppercase) {
            return Err(format!("snake_case cannot contain upper case: {}", value));
        }

        if value.contains(" ") {
            return Err(format!("snake_case cannot contain spaces: {}", value));
        }

        if value.contains("__") {
            return Err(format!("snake_case cannot contain double underscores: {}", value));
        }

        Ok(SnakeCase(value.to_string()))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ScreamingSnakeCase(String);

impl Deref for ScreamingSnakeCase {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ScreamingSnakeCase {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.0.fmt(f)
    }
}

impl TryFrom<&str> for ScreamingSnakeCase {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.chars().any(char::is_lowercase) {
            return Err(format!("SCREAMING_SNAKE_CASE cannot contain lowercase: {}", value));
        }

        if value.contains(" ") {
            return Err(format!("SCREAMING_SNAKE_CASE cannot contain spaces: {}", value));
        }

        if value.contains("__") {
            return Err(format!("SCREAMING_SNAKE_CASE cannot contain double underscores: {}", value));
        }

        Ok(ScreamingSnakeCase(value.to_string()))
    }
}

impl Into<SnakeCase> for CamelCase {
    fn into(self) -> SnakeCase {
        let mut output = String::new();

        for (i, c) in self.0.chars().enumerate() {
            if c.is_ascii_uppercase() && i != 0 {
                output.push('_');
            }

            output.push(c.to_ascii_lowercase());
        }

        SnakeCase(output)
    }
}

impl Into<SnakeCase> for ScreamingSnakeCase {
    fn into(self) -> SnakeCase {
        SnakeCase::try_from(self.0.to_lowercase().as_str()).unwrap()
    }
}

impl Into<ScreamingSnakeCase> for SnakeCase {
    fn into(self) -> ScreamingSnakeCase {
        ScreamingSnakeCase::try_from(self.0.to_uppercase().as_str()).unwrap()
    }
}

impl Into<CamelCase> for SnakeCase {
    fn into(self) -> CamelCase {
        let mut output = String::new();

        self.0.split('_')
            .for_each(|word| {
                for (i, c) in word.chars().enumerate() {
                    if i == 0 {
                        output.push(c.to_uppercase().nth(0).unwrap());
                    } else {
                        output.push(c);
                    }
                }
            });

        CamelCase::try_from(output.as_str()).unwrap()
    }
}

impl Into<CamelCase> for ScreamingSnakeCase {
    fn into(self) -> CamelCase {
        let snake: SnakeCase = self.into();
        snake.into()
    }
}

impl Into<ScreamingSnakeCase> for CamelCase {
    fn into(self) -> ScreamingSnakeCase {
        let snake: SnakeCase = self.into();
        snake.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camel_case_spaces_returns_error() {
        assert!(CamelCase::try_from("Nope Wont").is_err())
    }

    #[test]
    fn camel_case_underscore_returns_error() {
        assert!(CamelCase::try_from("Nope_Wont").is_err())
    }

    #[test]
    fn camel_case_empty_returns_error() {
        assert!(CamelCase::try_from("").is_err());
    }

    #[test]
    fn camel_case_starts_lower_case_returns_error() {
        assert!(CamelCase::try_from("nope").is_err());
    }

    #[test]
    fn camel_case_valid_input_returns_okay() {
        assert!(CamelCase::try_from("ValidInput").is_ok());
    }

    #[test]
    fn snake_case_cannot_be_empty() {
        assert!(SnakeCase::try_from("").is_err())
    }

    #[test]
    fn snake_case_cannot_contain_uppercase() {
        assert!(SnakeCase::try_from("upperCase").is_err());
    }

    #[test]
    fn snake_case_cannot_contain_spaces() {
        assert!(SnakeCase::try_from("invalid case").is_err());
    }

    #[test]
    fn snake_case_cannot_contain_double_underscores() {
        assert!(SnakeCase::try_from("invalid__case").is_err());
    }

    #[test]
    fn snake_case_valid_input() {
        assert!(SnakeCase::try_from("valid_input").is_ok());
    }

    #[test]
    fn screaming_snake_case_cannot_contain_lowercase() {
        assert!(ScreamingSnakeCase::try_from("NOPE_wont").is_err());
    }

    #[test]
    fn screaming_snake_case_cannot_contain_spaces() {
        assert!(ScreamingSnakeCase::try_from("NOPE WONT").is_err());
    }

    #[test]
    fn screaming_snake_case_cannot_contain_double_underscores() {
        assert!(ScreamingSnakeCase::try_from("NOPE__WONT").is_err());
    }

    #[test]
    fn screaming_snake_valid_input() {
        assert!(ScreamingSnakeCase::try_from("VALID_INPUT").is_ok());
    }

    #[test]
    fn to_snake_case() {
        let snake = SnakeCase::try_from("body_orbit").unwrap();
        let camel = CamelCase::try_from("BodyOrbit").unwrap();
        let screaming = ScreamingSnakeCase::try_from("BODY_ORBIT").unwrap();

        assert_eq!(snake, camel.into());
        assert_eq!(snake, screaming.into());
    }

    #[test]
    fn to_screaming_snake_case() {
        let snake = SnakeCase::try_from("body_orbit").unwrap();
        let camel = CamelCase::try_from("BodyOrbit").unwrap();
        let screaming = ScreamingSnakeCase::try_from("BODY_ORBIT").unwrap();

        assert_eq!(screaming, camel.into());
        assert_eq!(screaming, snake.into());
    }

    #[test]
    fn to_camel_case() {
        let snake = SnakeCase::try_from("body_orbit").unwrap();
        let camel = CamelCase::try_from("BodyOrbit").unwrap();
        let screaming = ScreamingSnakeCase::try_from("BODY_ORBIT").unwrap();

        assert_eq!(camel, snake.into());
        assert_eq!(camel, screaming.into());
    }
}