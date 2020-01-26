use crate::*;
use std::convert::TryInto;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Impl {
    pub strct: Struct,
    pub functions: Vec<Function>,
}

impl Impl {
    pub fn new(target: &Struct) -> Self {
        Self {
            strct: target.clone(),
            functions: vec![],
        }
    }

    pub fn add_function(mut self, function: Function) -> Self {
        self.functions.push(function);
        self
    }
}

impl Display for Impl {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "impl {} {}", self.strct.name, '{').ok();

        if self.functions.len() != 0 {
            writeln!(f, "").ok();
        }

        for function in self.functions.iter() {
            write!(f, "{}", function).ok();
        }

        writeln!(f, "{}", '}')
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: SnakeCase,
    pub visibility: Visibility,
    pub parameters: String,
    pub return_type: Option<String>,
    pub lines: Vec<CodeLine>,
}

impl Function {
    pub fn new(name: impl TryInto<SnakeCase,Error=impl Debug>) -> Self {
        Self {
            name: name.try_into().unwrap(),
            visibility: Visibility::Pub,
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

    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;
        self
    }

    pub fn add_line(mut self, line: CodeLine) -> Self {
        self.lines.push(line);
        self
    }

    fn get_return_type(&self) -> String {
        match self.return_type.clone() {
            Some(ty) => format!("-> {} " , ty),
            None => String::new(),
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "    {}fn {}({}) {}{}",
            self.visibility,
            self.name,
            self.parameters,
            self.get_return_type(),
            '{'
        ).ok();

        if self.lines.is_empty() {
            writeln!(f, "{}", '}')
        } else {
            writeln!(f, "").ok();
            for line in self.lines.iter() {
                writeln!(f, "    {}", line).ok();
            }
            writeln!(f, "    {}", '}')
        }
    }
}

#[derive(Debug, Clone)]
pub struct CodeLine {
    pub indent: Indent,
    pub text: String,
}

impl CodeLine {
    pub fn new(indent: u8, text: &str) -> Self {
        Self {
            indent: Indent(indent),
            text: text.to_string(),
        }
    }
}

impl Display for CodeLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}{}", self.indent, self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_impl() {
        let i = Impl::new(&Struct::new("Test"));

        println!("{}", i);
        assert_eq!("impl Test {}\n", i.to_string());
    }

    #[test]
    fn simple_impl() {
        let i = Impl::new(&Struct::new("Test"))
            .add_function(Function::new("test_fn"));

        let expected = "impl Test {\n    pub fn test_fn() {}\n}\n";

        println!("expected:\n\n{}", expected);
        println!("actual:\n\n{}", i);
        assert_eq!(expected, i.to_string());
    }

    #[test]
    fn panic_impl() {
        let i = Impl::new(&Struct::new("Test"))
            .add_function(Function::new("test_fn")
                .with_visibility(Visibility::Private)
                .add_line(CodeLine::new(0, "panic!()")));

        let expected = "impl Test {\n    fn test_fn() {\n        panic!()\n    }\n}\n";

        println!("expected:\n\n{}", expected);
        println!("actual:\n\n{}", i);
        assert_eq!(expected, i.to_string());
    }

    #[test]
    fn fn_with_return() {
        let i = Impl::new(&Struct::new("Test"))
            .add_function(Function::new("test_fn")
                .with_return("u32")
                .add_line(CodeLine::new(0, "panic!()")));

        let expected = "impl Test {\n    pub fn test_fn() -> u32 {\n        panic!()\n    }\n}\n";

        println!("expected:\n\n{}", expected);
        println!("actual:\n\n{}", i);
        assert_eq!(expected, i.to_string());
    }
}
