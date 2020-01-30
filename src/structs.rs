use crate::*;
use crate::formatting::SnakeCase;
use std::iter::FromIterator;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Struct {
    pub typ: Type,
    pub visibility: Visibility,
    pub derives: Derives,
    pub fields: Fields,
}

impl Struct {
    pub fn new(ty: &str) -> Self {
        Struct {
            typ: ty.parse().unwrap(),
            visibility: Visibility::Pub,
            derives: Default::default(),
            fields: Default::default()
        }
    }
    
    pub fn with_derives(mut self, derives: Derives) -> Self {
        self.derives = derives;
        self
    }

    pub fn with_fields(mut self, fields: Fields) -> Self {
        self.fields = fields;
        self
    }

    pub fn add_field(mut self, field: Field) -> Self {
        self.fields.0.push(field);
        self
    }

    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;
        self
    }
}

impl Display for Struct {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.derives).ok();

        write!(f, "{vis}struct {typ}", vis = self.visibility, typ = self.typ).ok();

        match self.fields.len() {
            0 => writeln!(f, ";"),
            _ => {
                writeln!(f, " {{").ok();
                write!(f, "{}", self.fields).ok();
                writeln!(f, "}}")
            } ,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Fields(Vec<Field>);

impl Fields {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item=&Field> {
        self.0.iter()
    }
}

impl Default for Fields {
    fn default() -> Self {
        Self(vec![])
    }
}

impl Extend<Field> for Fields {
    fn extend<T: IntoIterator<Item=Field>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl FromIterator<Field> for Fields {
    fn from_iter<T: IntoIterator<Item=Field>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Display for Fields {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for field in self.0.iter() {
            writeln!(f, "{}", field).ok();
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct AnonField {
    pub visibility: Visibility,
    pub field_type: String,
}

impl Display for AnonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f,
               "{}{}",
               self.visibility,
               self.field_type,
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Field {
    pub visibility: Visibility,
    pub name: SnakeCase,
    pub field_type: String,
}

impl Field {
    pub fn from_type(typ: Type) -> Self {
        let field_name: SnakeCase = CamelCase::from_str(typ.name.as_str()).unwrap().into();

        Field::new(field_name, typ)
    }

    pub fn new(name: SnakeCase, field_type: Type) -> Self {
        Field {
            visibility: Visibility::Pub,
            name,
            field_type: field_type.to_string(),
        }
    }

    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;
        self
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f,
            "{}{}{}: {},",
            Indent(1),
            self.visibility,
            self.name,
            self.field_type,
        )
    }
}

impl Into<AnonField> for Field {
    fn into(self) -> AnonField {
        AnonField {
            visibility: self.visibility,
            field_type: self.field_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_struct_none() {
        let s = Struct::new("Test")
            .with_visibility(Visibility::Private);

        assert_eq!("struct Test;\n", s.to_string());
    }

    #[test]
    fn pub_struct_none() {
        let s = Struct::new("Test");

        assert_eq!("pub struct Test;\n", s.to_string());
    }

    #[test]
    fn pub_crate_struct_none() {
        let s = Struct::new("Test").with_visibility(Visibility::PubCrate);

        assert_eq!("pub (crate) struct Test;\n", s.to_string());
    }

    #[test]
    fn field_struct() {
        let s = Struct::new("Test").add_field(Field::new("field".parse().unwrap(), "u32".parse().unwrap()));

        assert_eq!("pub struct Test {\n    pub field: u32,\n}\n", s.to_string());
    }

    #[test]
    fn example() {
        let arena = Struct::new("System")
            .add_field(Field::new("name".parse().unwrap(), "Component<Self, String>".parse().unwrap()))
            .add_field(Field::new("position".parse().unwrap(), "Component<Self, Position>".parse().unwrap()));

        assert_eq!(
            "pub struct System {\n    pub name: Component<Self, String>,\n    pub position: Component<Self, Position>,\n}\n",
            arena.to_string()
        );
    }

    #[test]
    fn struct_with_derives() {
        let s = Struct::new("Test").with_derives(Derives::with_debug_default());

        assert_eq!("#[derive(Debug, Default)]\npub struct Test;\n", s.to_string());
    }

    #[test]
    fn struct_with_generics() {
        let s = Struct::new("Test<T>").with_derives(Derives::with_debug_default());

        assert_eq!("#[derive(Debug, Default)]\npub struct Test<T>;\n", s.to_string());
    }

    #[test]
    fn struct_get_type_name() {
        let s = Struct::new("Id<T>");

        assert_eq!("Id<T>", s.typ.to_string());
    }
}