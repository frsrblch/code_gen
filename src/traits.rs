use crate::*;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Error};
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TraitName(CamelCase);

impl FromStr for TraitName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(|s| TraitName(s))
    }
}

impl Display for TraitName {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone)]
pub struct Trait {
    pub name: TraitName,
    pub visibility: Visibility,
    pub associated_types: Vec<TypeName>,
    pub functions: Vec<TraitFunction>,
}

impl Trait {
    pub fn new(name: &str) -> Self {
        Trait {
            name: name.parse().unwrap(),
            visibility: Visibility::Pub,
            associated_types: Default::default(),
            functions: vec![],
        }
    }

    pub fn add_associated_type(mut self, name: &str) -> Self {
        self.associated_types.push(name.parse().unwrap());
        self
    }

    pub fn add_function_definition(mut self, function_def: TraitFunction) -> Self {
        self.functions.push(function_def);
        self
    }

    pub fn impl_for(&self, strct: &Struct) -> TraitImplementation {
        TraitImplementation {
            trait_def: self.clone(),
            typ: strct.typ.clone(),
            associated_types: Default::default(),
            functions: vec![],
        }
    }
}

impl Display for Trait {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}trait {} {}", self.visibility, self.name, '{').ok();

        if self.associated_types.is_empty() && self.functions.is_empty() {
            return writeln!(f, "{}", '}');
        }
        else {
            writeln!(f, "").ok();
        }

        for ty in self.associated_types.iter() {
            writeln!(f, "{}type {};", Indent(1), ty).ok();
        }

        for func in self.functions.iter() {
            write!(f, "{}", func).ok();
        }

        writeln!(f, "{}", '}')
    }
}

#[derive(Debug, Clone)]
pub struct TraitFunction {
    pub name: SnakeCase,
    pub parameters: String,
    pub return_type: Option<String>,
    pub lines: Vec<CodeLine>,
}

impl TraitFunction {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.parse().unwrap(),
            parameters: String::new(),
            return_type: None,
            lines: vec![],
        }
    }

    pub fn with_parameters(mut self, params: &str) -> Self {
        self.parameters = params.to_string();
        self
    }

    pub fn with_return(mut self, return_type: &str) -> Self {
        self.return_type = return_type.to_string().into();
        self
    }

    pub fn add_line(mut self, line: CodeLine) -> Self {
        self.lines.push(line);
        self
    }

    fn get_return_type(&self) -> String {
        match self.return_type.clone() {
            Some(ty) => format!(" -> {}" , ty),
            None => String::new(),
        }
    }
}

impl Display for TraitFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}fn {}({}){}",
            Indent(1),
            self.name,
            self.parameters,
            self.get_return_type(),
        ).ok();

        if self.lines.is_empty() {
            writeln!(f, "{}", ';')
        } else {
            writeln!(f, " {}", '{').ok();
            for line in self.lines.iter() {
                writeln!(f, "{}{}", Indent(1), line).ok();
            }
            writeln!(f, "    {}", '}')
        }
    }
}


#[derive(Debug, Clone)]
pub struct TraitImplementation {
    pub trait_def: Trait,
    pub typ: Type,
    pub associated_types: HashMap<TypeName, Type>,
    pub functions: Vec<TraitFunction>,
}

impl TraitImplementation {
    pub fn add_associated_type(mut self, associated_type_name: TypeName, associated_type: Type) -> Self {
        self.associated_types.insert(associated_type_name, associated_type);
        self
    }

    pub fn add_function(mut self, function_def: TraitFunction) -> Self {
        self.functions.push(function_def);
        self
    }

    fn panic_if_invalid(&self) {
        // check types
        let all_trait_types_included = self.trait_def.associated_types.iter()
            .all(|ty| self.associated_types.contains_key(ty));
        assert!(all_trait_types_included);

        let all_included_types_are_required_by_trait = self.associated_types.iter()
            .all(|(k, _)| self.trait_def.associated_types.contains(k));
        assert!(all_included_types_are_required_by_trait);

        // check functions
        let all_fns_are_required_by_trait = self.functions.iter()
            .all(|f| self.fn_matches_trait_fn(f));
        assert!(all_fns_are_required_by_trait);

        let all_nondefault_trait_fns_are_impl = self.trait_def.functions.iter()
            .filter(|f| f.return_type.is_some() && f.lines.is_empty())
            .all(|f| self.trait_fn_matches_impl_fn(f));
        assert!(all_nondefault_trait_fns_are_impl);
    }

    fn fn_matches_trait_fn(&self, function: &TraitFunction) -> bool {
        self.trait_def.functions.iter().any(|f| {
            function.name == f.name
                && function.parameters == f.parameters
                && function.return_type == f.return_type
        })
    }

    fn trait_fn_matches_impl_fn(&self, function: &TraitFunction) -> bool {
        self.functions.iter().any(|f| {
            function.name == f.name
                && function.parameters == f.parameters
                && function.return_type == f.return_type
        })
    }
}

impl Display for TraitImplementation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.panic_if_invalid();

        let types = self.associated_types.len() > 0;
        let fns = self.functions.len() > 0;

        write!(f, "impl {} for {} {}", self.trait_def.name, self.typ, '{').ok();

        if types || fns {
            writeln!(f, "").ok();
        }

        for (gen, conc) in self.associated_types.iter() {
            writeln!(f, "{}type {} = {};", Indent(1), {gen}, {conc}).ok();
        }

        let functions: Vec<String> = self.functions.iter()
            .map(ToString::to_string)
            .collect();

        let functions = StrConcat {
            iter: functions,
            left_bound: "",
            right_bound: "",
            item_prepend: "",
            item_append: "",
            join: "\n"
        };

        write!(f, "{}", functions).ok();

        writeln!(f, "{}", '}')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function_def_ends_with_semicolon() {
        let f = TraitFunction::new("with_thing")
            .with_parameters("mut self");

        assert_eq!("    fn with_thing(mut self);\n", f.to_string());
    }

    #[test]
    fn function_def_returning_self_ends_with_semicolon() {
        let f = TraitFunction::new("with_thing")
            .with_parameters("mut self")
            .with_return("Self");

        assert_eq!("    fn with_thing(mut self) -> Self;\n", f.to_string());
    }

    #[test]
    fn function_def_with_default_implementation() {
        let f = TraitFunction::new("with_thing")
            .with_parameters("mut self")
            .add_line(CodeLine::new(0, "panic!()"));

        assert_eq!("    fn with_thing(mut self) {\n        panic!()\n    }\n", f.to_string());
    }

    #[test]
    fn define_trait_empty() {
        let t = Trait::new("Test");

        assert_eq!("pub trait Test {}\n", t.to_string());
    }

    #[test]
    fn define_trait_single_associated_type() {
        let t = Trait::new("Test")
            .add_associated_type("Idx");

        assert_eq!("pub trait Test {\n    type Idx;\n}\n", t.to_string());
    }

    #[test]
    fn define_trait_single_fn() {
        let t = Trait::new("Test")
            .add_function_definition(TraitFunction::new("method"));

        assert_eq!("pub trait Test {\n    fn method();\n}\n", t.to_string());
    }

    #[test]
    fn simple_trait_impl() {
        let t = Trait::new("Trait");
        let s = Struct::new("Struct");
        let i =  t.impl_for(&s);

        assert_eq!("impl Trait for Struct {}\n", i.to_string());
    }

    #[test]
    #[should_panic]
    fn implementation_missing_type() {
        let t = Trait::new("Trait").add_associated_type("T");
        let s = Struct::new("Struct");

        let _should_panic = t.impl_for(&s).to_string();
    }

    #[test]
    #[should_panic]
    fn implementation_with_superfluous_type() {
        let t = Trait::new("Trait");
        let s = Struct::new("Struct");

        let _should_panic = t.impl_for(&s)
            .add_associated_type(TypeName::from_str("T").unwrap(), Type::from_str("u32").unwrap())
            .to_string();
    }

    #[test]
    #[should_panic]
    fn implementation_with_superfluous_function() {
        let t = Trait::new("Trait");
        let s = Struct::new("Struct");

        let i = t.impl_for(&s)
            .add_function(TraitFunction::new("method").add_line(CodeLine::new(0, "panic!()")));

        let _should_panic = i.to_string();
    }

    #[test]
    #[should_panic]
    fn implementation_missing_fn_that_doesnt_have_a_default_def() {
        let t = Trait::new("Trait")
            .add_function_definition(TraitFunction::new("method")
                .with_return("u32"));
        let s = Struct::new("Struct");
        let i = t.impl_for(&s);

        let _should_panic = i.to_string();
    }

    #[test]
    fn implementation_with_associated_types() {
        let t = Trait::new("Trait").add_associated_type("Idx");
        let s = Struct::new("Struct");
        let i = t.impl_for(&s)
            .add_associated_type(TypeName::from_str("Idx").unwrap(), Type::from_str("u32").unwrap());

        assert_eq!("impl Trait for Struct {\n    type Idx = u32;\n}\n", i.to_string());
    }

    #[test]
    fn implementation_with_function() {
        let fn_def = TraitFunction::new("method")
            .with_return("u32");

        let fn_impl = fn_def.clone()
            .add_line(CodeLine::new(0, "1"));

        let t = Trait::new("Trait").add_function_definition(fn_def);
        let s = Struct::new("Struct");
        let i = t.impl_for(&s).add_function(fn_impl);

        assert_eq!("impl Trait for Struct {\n    fn method() -> u32 {\n        1\n    }\n}\n", i.to_string());
    }
}