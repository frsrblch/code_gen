use std::fmt::{Display, Formatter, Error};
use crate::{SnakeCase, Visibility, Indent};

#[derive(Debug, Clone)]
pub struct Mod {
    pub name: SnakeCase,
    pub vis: Visibility,
    pub body: String,
}

impl Mod {
    pub fn new(name: &str, body: String) -> Self {
        Mod {
            name: name.parse().unwrap(),
            vis: Default::default(),
            body
        }
    }

    pub fn with_visibility(mut self, vis: Visibility) -> Self {
        self.vis = vis;
        self
    }
}

impl Display for Mod {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "{}mod {} {{", self.vis, self.name).ok();

        for line in self.body.lines() {
            writeln!(f, "{}{}", Indent(1), line).ok();
        }

        writeln!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Struct, Field};

    #[test]
    fn simple() {
        let s = Struct::new("Test").add_field(Field::new("value", "u32"));
        let m = Mod::new("test_mod", s.to_string());

        assert_eq!("pub mod test_mod {\n    pub struct Test {\n        pub value: u32,\n    }\n}\n", m.to_string());
    }
}

